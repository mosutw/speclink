use chrono::{Duration, Utc};
use speclink_server::audit::{AuditActor, AuditSource};

use speclink_server::backup;
use speclink_server::config::{IdentityConfig, ServerConfig, StoreConfig};
use speclink_server::events::EventSettings;
use speclink_server::identity::{IdentitySqlite, IdentityStore, MembershipRole, NewInvitation};
use speclink_store::{CommandContext, DocumentId, ProjectId, RepoId, Scope};
use std::path::Path;

const CHECK_ID: &str = "01ARZ3NDEKTSV4RRFFQ69G5FAV";

pub fn assert_legacy_backup(source: StoreConfig, target: StoreConfig, folder: &Path) {
    let old = speclink_server::build_store(&source).unwrap();
    let identity = IdentitySqlite::open(folder.join("old-identity.db")).unwrap();
    identity.create_project("acme", "Acme").unwrap();
    for repo in ["web", "api"] {
        identity.create_repo("acme", repo, repo).unwrap();
    }

    let invite = identity
        .create_invitation(NewInvitation {
            email: "reader@example.test".into(),
            display: "Fixture reader".into(),
            memberships: vec!["acme".into()],
            admin: false,
            expires_at: Utc::now() + Duration::hours(1),
        })
        .unwrap();
    let user = identity
        .accept_invitation(&invite, "fixture-reader-password")
        .unwrap();
    let actor = AuditActor::user(&user, AuditSource::Web);
    identity
        .admin_set_membership(&actor, &user, "acme", MembershipRole::Reader, true)
        .unwrap();
    let (_, token) = identity.create_pat(&user, "fixture", None).unwrap();

    let web = Scope::new(ProjectId::new("acme"), RepoId::new("web"));
    let api = Scope::new(ProjectId::new("acme"), RepoId::new("api"));
    let mut unit = old
        .begin_unit_of_work(
            &web,
            CommandContext {
                actor: "fixture".into(),
                command: "seed".into(),
            },
        )
        .unwrap();
    unit.create(
        DocumentId::WorkflowConfig,
        "schema: spec-driven\ntdd: true\naudit: true\n",
    );
    unit.create(
        DocumentId::CanonicalSpec {
            capability: "auth".into(),
        },
        "# Auth\n",
    );
    unit.create(
        DocumentId::ChangeMeta {
            change: "add-auth".into(),
        },
        "schema: spec-driven\n",
    );
    unit.create(
        DocumentId::ChangeEvidence {
            change: "add-auth".into(),
        },
        "{}\n",
    );
    for (path, content) in [
        (format!("checks/review/{CHECK_ID}.md"), "## Review ticket\n"),
        (
            format!("checks/security/{CHECK_ID}/report.json"),
            "{\"report\":\"fixture\"}\n",
        ),
        (
            format!("checks/security/{CHECK_ID}/decisions.jsonl"),
            "{\"decision\":\"accept\"}\n",
        ),
    ] {
        unit.create(
            DocumentId::ChangeArtifact {
                change: "add-auth".into(),
                artifact: path,
            },
            content,
        );
    }
    unit.create(
        DocumentId::ArchivedChange {
            change: "2026-11-01-old-auth".into(),
            doc: format!("checks/verify/{CHECK_ID}.md"),
        },
        "## Verify ticket\n",
    );
    old.commit(unit, Vec::new()).unwrap();

    let mut unit = old
        .begin_unit_of_work(
            &api,
            CommandContext {
                actor: "fixture".into(),
                command: "seed".into(),
            },
        )
        .unwrap();
    unit.create(
        DocumentId::CanonicalSpec {
            capability: "api-auth".into(),
        },
        "# API Auth\n",
    );
    old.commit(unit, Vec::new()).unwrap();

    let bundle = folder.join("legacy-format-1.tar");
    assert_eq!(
        backup::create(old.as_ref(), &identity, &bundle)
            .unwrap()
            .backup_format_version,
        1
    );
    assert_eq!(backup::verify(&bundle).unwrap().scope_count, 2);
    let target_identity = folder.join("restored-identity.db");
    let config = ServerConfig {
        store: target,
        identity: IdentityConfig::Sqlite {
            path: target_identity.clone(),
        },
        public_url: "http://127.0.0.1".into(),
        events: EventSettings::default(),
    };
    assert!(matches!(
        speclink_server::native_storage::open(&source),
        Err(speclink_server::native_storage::OpenError::Legacy { .. })
    ));
    let report = backup::restore_to_port(&config, &bundle).unwrap();
    assert!(report.ok);
    let restored = speclink_server::native_storage::open(&config.store).unwrap();
    for scope in [web, api] {
        use speclink_host::transfer::{self, key_of_document, legacy_document_id};
        let expected: std::collections::BTreeMap<_, _> = old
            .export(&scope)
            .unwrap()
            .documents
            .into_iter()
            .map(|doc| {
                let key = key_of_document(&legacy_document_id(doc.doc)).unwrap();
                (key.as_str().to_owned(), (doc.content, doc.digest))
            })
            .collect();
        let scope =
            speclink_server::native_storage::scope_id(scope.project.as_str(), scope.repo.as_str())
                .unwrap();
        let actual: std::collections::BTreeMap<_, _> = transfer::export(&*restored, &scope)
            .unwrap()
            .documents
            .into_iter()
            .map(|doc| (doc.key, (doc.content, doc.digest)))
            .collect();
        assert_eq!(actual, expected, "scope {}", scope.as_str());
    }
    let identity = IdentitySqlite::open(&target_identity).unwrap();
    assert_eq!(
        identity.authenticate_pat(&token).unwrap().unwrap().1.id,
        user
    );
    assert_eq!(
        identity.membership_role(&user, "acme").unwrap(),
        Some(MembershipRole::Reader)
    );
    assert_eq!(identity.list_repos("acme").unwrap().len(), 2);
    drop(identity);
    drop(restored);
    assert!(backup::restore_to_port(&config, &bundle)
        .unwrap_err()
        .to_string()
        .contains("not empty"));
}
