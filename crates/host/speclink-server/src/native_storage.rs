//! Open the reference server's Store Port backends. Legacy files are left
//! in place so the old server can still back them up and serve them.

use crate::config::StoreConfig;
use speclink_store_port::{PortError, RecordedEvent, ScopeId, StorePort};
use std::sync::Arc;

mod team_compat;
use std::fmt;

#[derive(Debug)]
pub enum OpenError {
    Legacy { location: String },
    Failed(String),
}

impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenError::Legacy { location } => write!(f,
                "The store at {location} uses the previous Speclink layout and was left untouched. \
                Run `speclink-server backup --config <old-config> --output <backup>` with the old version, \
                then restore the backup into an empty store using the new version."),
            OpenError::Failed(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for OpenError {}

/// Native event and health operations used only by the reference server.
pub trait ServerPort: StorePort {
    fn has_committed_scopes(&self) -> Result<bool, PortError>;
    fn kind(&self) -> &'static str;
    fn probe(&self) -> Result<(), PortError>;
    fn events_after(
        &self,
        scope: &ScopeId,
        after: u64,
        limit: usize,
    ) -> Result<Vec<(u64, RecordedEvent)>, PortError>;
    fn event_floor(&self, scope: &ScopeId) -> Result<u64, PortError>;
    fn trim_events(&self, scope: &ScopeId, through: u64) -> Result<(), PortError>;
}

macro_rules! server_port {
    ($driver:ty, $kind:literal) => {
        impl ServerPort for $driver {
            fn has_committed_scopes(&self) -> Result<bool, PortError> {
                <$driver>::has_committed_scopes(self)
            }
            fn kind(&self) -> &'static str {
                $kind
            }
            fn probe(&self) -> Result<(), PortError> {
                <$driver>::probe(self)
            }
            fn events_after(
                &self,
                scope: &ScopeId,
                after: u64,
                limit: usize,
            ) -> Result<Vec<(u64, RecordedEvent)>, PortError> {
                <$driver>::events_after(self, scope, after, limit)
            }
            fn event_floor(&self, scope: &ScopeId) -> Result<u64, PortError> {
                <$driver>::event_floor(self, scope)
            }
            fn trim_events(&self, scope: &ScopeId, through: u64) -> Result<(), PortError> {
                <$driver>::trim_events(self, scope, through)
            }
        }
    };
}

server_port!(speclink_store_port_fs::FsPort, "serverfs");
server_port!(speclink_store_port_sqlite::SqlitePort, "sqlite");
server_port!(speclink_store_port_postgres::PostgresPort, "postgres");

pub fn open_for_server(config: &StoreConfig) -> Result<crate::state::SharedStore, OpenError> {
    if matches!(config, StoreConfig::Memory) {
        return Ok(Arc::new(speclink_store::memory::MemoryStore::new()));
    }
    if let StoreConfig::Postgres { url } = config {
        if speclink_store_port_postgres::url_embeds_password(url) {
            eprintln!("warning: the postgres store url carries a password; prefer leaving it out and setting {} instead", speclink_store_port_postgres::PASSWORD_VAR);
        }
    }
    Ok(Arc::new(team_compat::NativeTeamStore(open(config)?)))
}

pub fn open(config: &StoreConfig) -> Result<Box<dyn ServerPort>, OpenError> {
    macro_rules! opened {
        ($opening:expr, $driver:ident, $location:expr) => {
            match $opening {
                Ok(store) => Ok(Box::new(store) as Box<dyn ServerPort>),
                Err($driver::OpenError::UnknownLayout {
                    format: Some(marker),
                    ..
                }) if matches!(
                    marker.as_str(),
                    "speclink-team-store" | "speclink-team-store-fs"
                ) =>
                {
                    Err(OpenError::Legacy {
                        location: $location,
                    })
                }
                Err(error) => Err(OpenError::Failed(format!(
                    "could not open {}: {error}",
                    $location
                ))),
            }
        };
    }
    match config {
        StoreConfig::Memory => Err(OpenError::Failed(
            "a persistent Store Port backend is required".into(),
        )),
        StoreConfig::Sqlite { path } => opened!(
            speclink_store_port_sqlite::SqlitePort::open(path),
            speclink_store_port_sqlite,
            path.display().to_string()
        ),
        StoreConfig::ServerFs { path } => opened!(
            speclink_store_port_fs::FsPort::open(path),
            speclink_store_port_fs,
            path.display().to_string()
        ),
        StoreConfig::Postgres { url } => opened!(
            speclink_store_port_postgres::PostgresPort::connect(url),
            speclink_store_port_postgres,
            "the configured PostgreSQL database".to_owned()
        ),
    }
}

pub fn scope_id(project: &str, repo: &str) -> Result<ScopeId, speclink_store_port::InvalidValue> {
    let project = project.replace('%', "%25").replace('/', "%2F");
    ScopeId::new(format!("{project}/{repo}"))
}
