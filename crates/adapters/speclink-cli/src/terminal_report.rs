//! Best-effort Herdr terminal activity reporting for successful mutating CLI commands.
//!
//! Only runs inside a Herdr terminal (when `HERDR_TAB_ID` or `HERDR_AGENT_ID` is set)
//! and unless `SPECLINK_TERMINAL_REPORT=false` opts out. Errors are silently ignored so
//! stdout, stderr, and exit codes are never affected.

use std::process::{Command, Stdio};

pub(crate) enum ActivityTarget {
    Change(String),
    Discussion(String),
}

pub(crate) fn report_if_active(target: Option<ActivityTarget>) {
    let Some(target) = target else {
        return;
    };
    if std::env::var_os("SPECLINK_TERMINAL_REPORT")
        .and_then(|v| v.into_string().ok())
        .is_some_and(|v| v.eq_ignore_ascii_case("false"))
    {
        return;
    }
    let tab_id = std::env::var("HERDR_TAB_ID").ok().filter(|s| !s.trim().is_empty());
    let agent_id = std::env::var("HERDR_AGENT_ID").ok().filter(|s| !s.trim().is_empty());
    if tab_id.is_none() && agent_id.is_none() {
        return;
    }

    let bin = std::env::var("HERDR_BIN")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "herdr-app".to_string());

    let (kind, name) = match target {
        ActivityTarget::Change(name) => ("speclink:change", name),
        ActivityTarget::Discussion(slug) => ("speclink:discussion", slug),
    };
    let payload = serde_json::json!({
        "label": format!("{kind}:{name}"),
        "tabId": tab_id,
        "agentId": agent_id,
    });

    let _ = Command::new(bin)
        .args(["do", "set-label", &payload.to_string()])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}
