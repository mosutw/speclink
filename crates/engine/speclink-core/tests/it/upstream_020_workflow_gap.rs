//! 上游 0.20.0 第 4 批（生命週期、工作流與語言）核心引擎回歸測試。
//!
//! 驗證：
//! 1. `ToolSelection::resolve` 支援 `pi`（`.pi/skills`）與 `cursor`（`.cursor/skills`）內建工具
//! 2. `speclink_core::config::resolve_policy` 與 `instructions` 對 `spec_locale` 與既有規格原名的契約

use speclink_core::config::{resolve_policy, AppConfig, EnvOverrides, WorkflowConfig};
use speclink_core::init::ToolSelection;

#[test]
fn tool_selection_recognizes_pi_and_cursor_as_builtin_tools() {
    let dir = std::env::temp_dir().join(format!(
        "speclink-core-pi-cursor-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join(".speclink.yaml"),
        "tools: [claude, codex, copilot, antigravity, pi, cursor]\n",
    )
    .unwrap();

    let app = AppConfig::load(&dir.join(".speclink.yaml")).expect("load .speclink.yaml");
    let sel = ToolSelection::resolve(&dir, &app);

    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        sel.notes.is_empty(),
        "pi and cursor must be recognized without unknown-tool notes: {:?}",
        sel.notes
    );
    let dirs: Vec<&str> = sel.builtins.iter().map(|t| t.skills_dir()).collect();
    assert!(dirs.contains(&".pi/skills"), "missing .pi/skills in {dirs:?}");
    assert!(
        dirs.contains(&".cursor/skills"),
        "missing .cursor/skills in {dirs:?}"
    );
}

#[test]
fn workflow_config_recognizes_subagent_model_field() {
    let wf = WorkflowConfig::from_text(Some(
        "schema: spec-driven\nlocale: tw\nspec_locale: ja\nsubagent_model:\n  review:\n    codex: o3\n",
    ))
    .expect("parse workflow config");
    let policy = resolve_policy(&EnvOverrides::default(), &wf);
    assert_eq!(policy.spec_locale.as_deref(), Some("ja"));
}
