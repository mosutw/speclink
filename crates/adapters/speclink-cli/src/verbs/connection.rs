//! Connection management: link, unlink, auth.
//!
//! ModeFree by declaration — these verbs change the store mode rather than
//! consume it, so they resolve the connection themselves instead of going
//! through dispatch's mode shapes.

use anyhow::{bail, Result};
use clap::{Args, Subcommand};
use speclink_core as core;
use speclink_protocol::query as protocol_query;
use speclink_remote::auth as remote_auth;
use speclink_remote::client::Client as RemoteClient;
use speclink_remote::credentials as remote_credentials;
use speclink_remote::login as remote_login;
use std::io::IsTerminal;
use std::process::Stdio;

use crate::color;
use crate::common::{print_json, read_stdin};
use crate::remote_base::{ensure_repo_registered, git_reference_warning};

#[derive(Args)]
pub(crate) struct LinkArgs {
    /// Project-scoped connection URL
    url: String,
    /// This repo's registered name in the remote project
    #[arg(long)]
    repo: Option<String>,
}
#[derive(Args)]
pub(crate) struct AuthArgs {
    #[command(subcommand)]
    command: AuthCommands,
}
#[derive(Subcommand)]
enum AuthCommands {
    /// Log in to the connected server (device authorization by default)
    Login {
        /// Sign in through the server's login service in a browser
        #[arg(long, conflicts_with_all = ["device", "pat", "token_stdin"])]
        browser: bool,
        /// Authorize this device on the server's login page
        #[arg(long, conflicts_with_all = ["browser", "pat", "token_stdin"])]
        device: bool,
        /// Read a personal access token from stdin (CI/scripted use)
        #[arg(long = "token-stdin", conflicts_with = "pat")]
        token_stdin: bool,
        /// Paste a personal access token instead of authorizing this device
        #[arg(long = "pat")]
        pat: bool,
    },
    /// Show the current identity and repo validation result
    Status {
        /// Emit the identity and credential source as JSON
        #[arg(long)]
        json: bool,
    },
    /// Log out: revoke this device's credential family and clear local credentials
    Logout,
}
pub(crate) fn cmd_link(a: LinkArgs) -> Result<()> {
    let root = std::env::current_dir()?;
    let origin = remote_auth::origin_of(&a.url);
    // The full ladder, not just the file: a desktop login already covers this
    // origin, and reporting "no credentials yet" at it would be a lie.
    match remote_auth::resolve_credential(&origin).ok().flatten() {
        Some(resolved) => {
            let token = resolved.token;
            let whoami = RemoteClient::new(&a.url, &token, None).whoami()?;
            if a.repo.is_none() && whoami.repos.len() > 1 {
                let choices: Vec<&str> = whoami.repos.iter().map(|repo| repo.name.as_str()).collect();
                bail!("This project has multiple repositories. Specify --repo (available: {})", choices.join(", "));
            }
            if let Some(repo) = a.repo.as_deref() {
                ensure_repo_registered(&whoami, repo)?;
                println!("{} Repo '{repo}' is registered in this project", color::green("✓"));
            }
            core::config::write_remote_section(&root, &a.url, a.repo.as_deref())?;
            println!("{} Linked to {}", color::green("✓"), a.url);
            git_reference_warning(&root, a.repo.as_deref(), &whoami);
            Ok(())
        }
        None => {
            core::config::write_remote_section(&root, &a.url, a.repo.as_deref())?;
            println!("{} Linked to {}", color::green("✓"), a.url);
            println!("  No credentials yet — run `speclink auth login` to connect");
            Ok(())
        }
    }
}
pub(crate) fn cmd_unlink() -> Result<()> {
    let root = core::workspace::Workspace::discover_cwd()?
        .map(|ws| ws.root)
        .unwrap_or(std::env::current_dir()?);
    if !core::config::remove_remote_section(&root)? {
        bail!("No remote connection to remove (no `remote:` section in .speclink.yaml)");
    }
    println!(
        "{} Removed the remote section from .speclink.yaml — back to the local fs store",
        color::green("✓")
    );
    Ok(())
}
pub(crate) fn cmd_auth(a: AuthArgs) -> Result<()> {
    // Both auth verbs need the connection (its origin keys the credentials).
    let Some(ws) = core::workspace::Workspace::discover_cwd()? else {
        bail!("Not connected to a remote store — run `speclink link <url>` first");
    };
    let conn = match speclink_host::context::resolve_store_mode(&ws)?.mode {
        core::workspace::StoreMode::Remote(conn) => conn,
        core::workspace::StoreMode::Fs => {
            bail!("Not connected to a remote store — run `speclink link <url>` first")
        }
    };
    let origin = remote_auth::origin_of(&conn.url);
    match a.command {
        AuthCommands::Login { browser, device, token_stdin, pat } => {
            use speclink_remote::discovery::{self, Method, Plan};
            if speclink_host::portable::data_dir().is_some() {
                bail!("Portable mode does not save logins. Set SPECLINK_TOKEN for this CLI session, or sign in within the desktop app.");
            }
            if token_stdin || pat {
                return login_with_pat(&conn, &origin, token_stdin);
            }
            if !std::io::stdin().is_terminal() {
                bail!("`speclink auth login` needs a terminal — use `--token-stdin` to pipe a personal access token instead");
            }
            let preferred = if browser {
                Some(Method::Browser)
            } else if device {
                Some(Method::Device)
            } else {
                None
            };
            let plan = discovery::choose(&discovery::discover(&origin)?, preferred)?;
            match plan {
                Plan::Token => login_with_pat(&conn, &origin, false),
                _ => login_with_browser_or_device(&conn, &origin, &plan),
            }
        }
        AuthCommands::Status { json } => {
            let resolved = remote_auth::resolve_credential(&origin)
                .map_err(|e| anyhow::anyhow!(e.message()))?;
            let Some(resolved) = resolved else {
                bail!("Not logged in to {origin} — run `speclink auth login`");
            };
            let whoami = RemoteClient::new(&conn.url, &resolved.token, None).whoami()?;
            if json {
                return print_json(&serde_json::json!({
                    "user": {
                        "name": whoami.user.name,
                        "handle": whoami.user.handle,
                    },
                    "credentialSource": resolved.source.as_str(),
                }));
            }
            print_identity(&whoami);
            println!("  Credential from: {}", resolved.source.describe());
            if let Some(repo) = conn.repo.as_deref() {
                match ensure_repo_registered(&whoami, repo) {
                    Ok(()) => println!("Repo '{repo}' is registered in this project"),
                    Err(e) => println!("! {e}"),
                }
            }
            git_reference_warning(&ws.root, conn.repo.as_deref(), &whoami);
            Ok(())
        }
        AuthCommands::Logout => cmd_auth_logout(&origin),
    }
}
/// Validate an access key before storing it. Interactive login uses the OS
/// keyring when available. Piped input keeps the existing CI file path.
fn login_with_pat(
    conn: &core::workspace::RemoteConnection,
    origin: &str,
    token_stdin: bool,
) -> Result<()> {
    if !token_stdin {
        eprint!("Paste your personal access token: ");
    }
    let token = read_stdin().trim().to_string();
    if token.is_empty() {
        bail!("empty token — nothing stored");
    }
    let whoami = RemoteClient::new(&conn.url, &token, None).whoami()?;
    let dir = remote_auth::speclink_config_dir();
    let store = remote_credentials::client_store();
    if !token_stdin && store.set(origin, remote_credentials::CredentialKind::Pat, &token).is_ok() {
        remote_auth::clear_keychain_login(origin, store);
        remote_auth::remove_token_at(&dir, origin)?;
    } else {
        remote_auth::save_token_at(&dir, origin, &token)?;
    }
    print_identity(&whoami);
    Ok(())
}
/// Terminal and clock for the shared login flow.
struct CliLoginIo;
impl remote_login::LoginIo for CliLoginIo {
    fn phase(&self, phase: remote_login::LoginPhase) {
        match phase {
            remote_login::LoginPhase::AwaitingDevice { verification_uri, user_code, .. } => {
                println!("Open {verification_uri} and enter code: {user_code}");
                println!("Waiting for approval…");
            }
            remote_login::LoginPhase::AwaitingBrowser { url, .. } => {
                println!("Open {url} to sign in");
                println!("Waiting for sign-in…");
            }
        }
    }

    fn cancelled(&self) -> bool { false }

    fn open_browser(&self, url: &str) -> bool {
        #[cfg(target_os = "macos")]
        let mut cmd = std::process::Command::new("open");
        #[cfg(target_os = "windows")]
        let mut cmd = {
            let mut c = std::process::Command::new("rundll32");
            c.arg("url.dll,FileProtocolHandler");
            c
        };
        #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
        let mut cmd = std::process::Command::new("xdg-open");

        cmd.arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    fn sleep_secs(&self, secs: u64) {
        std::thread::sleep(std::time::Duration::from_secs(secs));
    }
}
fn login_with_browser_or_device(
    conn: &core::workspace::RemoteConnection,
    origin: &str,
    plan: &remote_login::Plan,
) -> Result<()> {
    use remote_login::LoginOutcome;
    let target = remote_login::LoginTarget {
        origin,
        credentials: remote_credentials::client_store(),
        lock_dir: &remote_auth::speclink_config_dir(),
    };
    match remote_login::login(&target, plan, &CliLoginIo) {
        LoginOutcome::LoggedIn { display, .. } => {
            println!("{} Logged in as {display}", color::green("✓"));
            if let Some(repo) = conn.repo.as_deref() {
                println!("Connected to repo '{repo}'");
            }
            Ok(())
        }
        LoginOutcome::LoggedInWithoutIdentity { message } => {
            println!("{} Login saved", color::green("✓"));
            eprintln!("speclink: warning: could not read the account identity ({message})");
            Ok(())
        }
        LoginOutcome::Denied => bail!("authorization denied — nothing was stored"),
        LoginOutcome::Expired => bail!("authorization expired before it was approved — run `speclink auth login` again"),
        LoginOutcome::Cancelled => bail!("login cancelled"),
        LoginOutcome::Unsupported | LoginOutcome::TokenRequired => bail!("this server does not offer device authorization — use `speclink auth login --pat`"),
        LoginOutcome::KeychainUnavailable => bail!("no system keychain available on this machine — use `speclink auth login --pat` or set SPECLINK_TOKEN"),
        LoginOutcome::Failed(message) => bail!("{message}"),
    }
}
/// Logout revokes the credential family on the server, then clears every local
/// credential for the origin. The family is shared with the desktop app, so
/// this logs the whole machine out — the symmetric cost of one login covering
/// both. The server-side PAT is left alone: it is a separate credential the
/// user manages, possibly in use from another machine.
fn cmd_auth_logout(origin: &str) -> Result<()> {
    let dir = remote_auth::speclink_config_dir();
    let result = remote_login::logout(origin, remote_credentials::client_store(), Some(&dir))
        .map_err(anyhow::Error::msg)?;
    if !result.was_logged_in {
        bail!("Not logged in to {origin}");
    }
    if let Some(message) = result.revoke_warning {
        eprintln!("speclink: warning: {message}");
    }
    println!("{} Logged out of {origin}", color::green("✓"));
    Ok(())
}
fn print_identity(whoami: &protocol_query::WhoamiResponse) {
    if whoami.user.handle.is_empty() {
        println!("{} Logged in as {}", color::green("✓"), whoami.user.name);
    } else {
        println!(
            "{} Logged in as {} (@{})",
            color::green("✓"),
            whoami.user.name,
            whoami.user.handle
        );
    }
}
