//! 上游 0.20.0 第 4 批（生命週期、工作流與語言）CLI 流程回歸測試。
//!
//! 覆蓋 `change-lifecycle`、`workspace-tools` 與 `workflow-config` 的新增契約：
//! 1. `ready_at` 寫入、隱藏與 `worktree` 下的 `show --json` 一致性
//! 2. `change landed`、`evidence show`、`claim --release`、`discuss restore` 與自由欄位防呆
//! 3. `pi`／`cursor` 工具共存、`.agents/skills` 共用保留與遠端 `update` 失敗不覆寫
//! 4. `workflow-config subagent-model` 冪等寫入、`spec_locale` 與不明 artifact ID 提示

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;

struct TempRepo {
    root: PathBuf,
    repo: PathBuf,
    home: PathBuf,
}

impl TempRepo {
    fn new(tag: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "speclink-cli-u020-wf-{tag}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        let repo = root.join("repo");
        let home = root.join("home");
        std::fs::create_dir_all(repo.join("openspec").join("changes")).unwrap();
        std::fs::create_dir_all(&home).unwrap();
        std::fs::write(repo.join(".speclink.yaml"), "tools:\n  - claude\n").unwrap();
        std::fs::write(
            repo.join("openspec").join("config.yaml"),
            "schema: spec-driven\nlocale: tw\n",
        )
        .unwrap();

        let t = Self { root, repo, home };
        t.git(&["init", "-q", "-b", "main"]);
        t.git(&["config", "user.name", "Sandbox Tester"]);
        t.git(&["config", "user.email", "sandbox@example.com"]);
        t.git(&["config", "core.autocrlf", "false"]);
        t.git(&["add", "-A"]);
        t.git(&["commit", "-q", "-m", "init"]);
        t
    }

    fn git(&self, args: &[&str]) {
        let out = Command::new("git")
            .args(args)
            .current_dir(&self.repo)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    fn run(&self, args: &[&str]) -> Output {
        self.run_in(&self.repo, args)
    }

    fn run_in(&self, dir: &Path, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_speclink"))
            .args(args)
            .current_dir(dir)
            .env_remove("SPECLINK_STORE_URL")
            .env_remove("SPECLINK_TOKEN")
            .env_remove("SPECLINK_LOCALE")
            .env_remove("SPECLINK_SPEC_LOCALE")
            .env_remove("NO_COLOR")
            .env_remove("CLICOLOR")
            .env_remove("CLICOLOR_FORCE")
            .env("USERPROFILE", &self.home)
            .env("HOME", &self.home)
            .env("XDG_CONFIG_HOME", &self.home)
            .output()
            .expect("run speclink binary")
    }

    fn write(&self, rel: &str, content: &str) {
        let path = self.repo.join(rel.split('/').collect::<PathBuf>());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    fn read(&self, rel: &str) -> String {
        std::fs::read_to_string(self.repo.join(rel.split('/').collect::<PathBuf>())).unwrap()
    }

    fn exists(&self, rel: &str) -> bool {
        self.repo.join(rel.split('/').collect::<PathBuf>()).exists()
    }

    fn add_change(&self, name: &str, tasks_md: &str) {
        self.write(
            &format!("openspec/changes/{name}/.meta.yaml"),
            "schema: spec-driven\ncreated: 2026-07-01\n",
        );
        self.write(
            &format!("openspec/changes/{name}/proposal.md"),
            "## Why\n\nWorkflow regression fixture.\n",
        );
        self.write(&format!("openspec/changes/{name}/tasks.md"), tasks_md);
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn stdout_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn stderr_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).to_string()
}

fn json_of(out: &Output) -> serde_json::Value {
    serde_json::from_str(&stdout_of(out)).unwrap_or_else(|e| {
        panic!(
            "invalid json ({e}):\nstdout: {}\nstderr: {}",
            stdout_of(out),
            stderr_of(out)
        )
    })
}

#[test]
fn ready_at_written_on_last_task_done_and_hidden_on_undone() {
    let t = TempRepo::new("ready-at");
    t.add_change("demo", "## 1. Group\n\n- [ ] 1.1 first\n- [ ] 1.2 second\n");

    // 完成第一項：尚未全部完成，show 不應輸出 readyAt
    let first = t.run(&["task", "done", "--change", "demo", "1"]);
    assert!(first.status.success(), "{}", stderr_of(&first));
    let show_mid = json_of(&t.run(&["show", "demo", "--json"]));
    assert!(
        show_mid.get("readyAt").is_none(),
        "readyAt must be omitted when tasks remain: {show_mid}"
    );

    // 完成最後一項：應寫入 ready_at，show 與 list 一致回傳 YYYY-MM-DD
    let second = t.run(&["task", "done", "--change", "demo", "2"]);
    assert!(second.status.success(), "{}", stderr_of(&second));
    let meta_after_done = t.read("openspec/changes/demo/.meta.yaml");
    assert!(
        meta_after_done.contains("ready_at:"),
        "ready_at must be stamped in .meta.yaml:\n{meta_after_done}"
    );
    let show_ready = json_of(&t.run(&["show", "demo", "--json"]));
    let ready_at = show_ready["readyAt"]
        .as_str()
        .expect("readyAt must be YYYY-MM-DD string after last task done");
    assert_eq!(ready_at.len(), 10, "expected YYYY-MM-DD, got {ready_at}");

    let list_json = json_of(&t.run(&["list", "--json"]));
    let list_item = list_json["changes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["name"] == "demo")
        .unwrap();
    assert_eq!(list_item["readyAt"].as_str(), Some(ready_at));

    // 取消勾選後：.meta.yaml 保留時戳歷史，但後續 show 不輸出 readyAt
    let undone = t.run(&["task", "undone", "--change", "demo", "2"]);
    assert!(undone.status.success(), "{}", stderr_of(&undone));
    let meta_after_undone = t.read("openspec/changes/demo/.meta.yaml");
    assert!(
        meta_after_undone.contains("ready_at:"),
        "ready_at history must remain in .meta.yaml:\n{meta_after_undone}"
    );
    let show_undone = json_of(&t.run(&["show", "demo", "--json"]));
    assert!(
        show_undone.get("readyAt").is_none(),
        "readyAt must be hidden after task undone: {show_undone}"
    );
}

#[test]
fn worktree_show_reads_same_copy_as_list() {
    let t = TempRepo::new("worktree-show");
    t.write("openspec/config.yaml", "schema: spec-driven\nworktree: true\n");
    t.add_change("wt-demo", "## 1. Group\n\n- [ ] 1.1 first\n- [ ] 1.2 second\n");
    t.git(&["add", "-A"]);
    t.git(&["commit", "-q", "-m", "add change"]);

    let wt_path = t.root.join("repo.worktrees").join("wt-demo");
    t.git(&[
        "worktree",
        "add",
        "-q",
        "-b",
        "speclink/wt-demo",
        wt_path.to_str().unwrap(),
    ]);

    // 在 worktree 內完成 1，主工作區的 show 與 list 應看到同一份進度
    let done = t.run_in(&wt_path, &["task", "done", "--change", "wt-demo", "1"]);
    assert!(done.status.success(), "{}", stderr_of(&done));

    let list_json = json_of(&t.run(&["list", "--json"]));
    let list_item = list_json["changes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["name"] == "wt-demo")
        .unwrap();
    assert_eq!(list_item["completedTasks"], 1);

    let show_json = json_of(&t.run(&["show", "wt-demo", "--json"]));
    assert!(
        show_json["tasks"]
            .as_str()
            .unwrap_or("")
            .contains("[x] 1.1"),
        "main checkout `show` must read the active worktree copy: {show_json}"
    );
}

#[test]
fn change_landed_and_evidence_show_survive_archive() {
    let t = TempRepo::new("landed-evidence");
    t.add_change("land-demo", "## 1. Group\n\n- [x] 1.1 done\n");
    t.write(
        "openspec/changes/land-demo/design.md",
        "## Context\n\nDemo.\n",
    );
    t.write(
        "openspec/changes/land-demo/.evidence.json",
        "{\"entries\":[{\"taskId\":\"1\",\"touchedFiles\":[\"src/lib.rs\"]}]}\n",
    );

    let landed = t.run(&["change", "landed", "land-demo", "--commit", "abc1234"]);
    assert!(landed.status.success(), "change landed failed: {}", stderr_of(&landed));

    let detail = json_of(&t.run(&["show", "land-demo", "--json", "--detail"]));
    assert_eq!(detail["landedCommit"].as_str(), Some("abc1234"));

    let archived = t.run(&["archive", "land-demo", "--yes"]);
    assert!(archived.status.success(), "archive failed: {}", stderr_of(&archived));

    let evidence = json_of(&t.run(&["evidence", "show", "land-demo", "--json"]));
    assert_eq!(evidence["change"].as_str(), Some("land-demo"));
    assert!(
        evidence.to_string().contains("src/lib.rs"),
        "archived evidence must remain readable: {evidence}"
    );
}

#[test]
fn invalid_field_key_or_missing_change_fails_closed() {
    let t = TempRepo::new("field-guard");
    t.add_change("fld-demo", "## 1. Group\n\n- [ ] 1.1 first\n");
    let before_meta = t.read("openspec/changes/fld-demo/.meta.yaml");

    let bad_key = t.run(&["change", "field", "fld-demo", "--set", "bad key=1"]);
    assert!(!bad_key.status.success(), "invalid field key must fail");
    assert!(!stderr_of(&bad_key).trim().is_empty());
    assert_eq!(t.read("openspec/changes/fld-demo/.meta.yaml"), before_meta);

    let missing = t.run(&["change", "field", "no-such-change", "--set", "owner=alice"]);
    assert!(!missing.status.success(), "missing change must fail");
    assert!(!stderr_of(&missing).trim().is_empty());
}

#[test]
fn discuss_restore_reopens_archived_discussion() {
    let t = TempRepo::new("discuss-restore");
    let created = t.run(&["discuss", "new", "topic-a"]);
    assert!(created.status.success(), "{}", stderr_of(&created));
    let archived = t.run(&["discuss", "archive", "topic-a"]);
    assert!(archived.status.success(), "{}", stderr_of(&archived));

    let restored = t.run(&["discuss", "restore", "topic-a"]);
    assert!(restored.status.success(), "discuss restore failed: {}", stderr_of(&restored));
    assert!(t.exists("openspec/discussions/topic-a.md"));
}

#[test]
fn init_and_update_support_pi_cursor_and_shared_agents_pruning() {
    let t = TempRepo::new("pi-cursor");
    let init = t.run(&["init", "--force", "--tools", "codex,antigravity,pi,cursor"]);
    assert!(init.status.success(), "init failed: {}", stderr_of(&init));
    assert!(t.exists(".agents/skills/speclink-apply/SKILL.md"));
    assert!(t.exists(".pi/skills/speclink-apply/SKILL.md"));
    assert!(t.exists(".cursor/skills/speclink-apply/SKILL.md"));

    // 取消 codex 但保留 antigravity，共用 .agents/skills 仍可用
    t.write(
        ".speclink.yaml",
        "tools:\n  - antigravity\n  - pi\n  - cursor\n",
    );
    let updated = t.run(&["update"]);
    assert!(updated.status.success(), "update failed: {}", stderr_of(&updated));
    assert!(t.exists(".agents/skills/speclink-apply/SKILL.md"));
    assert!(t.exists(".pi/skills/speclink-apply/SKILL.md"));
    assert!(t.exists(".cursor/skills/speclink-apply/SKILL.md"));
}

#[test]
fn remote_update_failure_preserves_existing_skills() {
    let t = TempRepo::new("remote-update-fail");
    let init = t.run(&["init", "--force", "--tools", "claude"]);
    assert!(init.status.success(), "{}", stderr_of(&init));
    let sentinel = "<!-- custom-sentinel -->\n";
    t.write(".claude/skills/speclink-apply/SKILL.md", sentinel);
    t.write(
        ".speclink.yaml",
        "tools:\n  - claude\nremote:\n  url: http://127.0.0.1:9\n",
    );

    let out = Command::new(env!("CARGO_BIN_EXE_speclink"))
        .args(["update"])
        .current_dir(&t.repo)
        .env("SPECLINK_STORE_URL", "http://127.0.0.1:9")
        .env("SPECLINK_TOKEN", "invalid-token")
        .env("USERPROFILE", &t.home)
        .env("HOME", &t.home)
        .env("XDG_CONFIG_HOME", &t.home)
        .output()
        .expect("run speclink update");
    assert!(!out.status.success(), "offline remote update must fail");
    assert!(!stderr_of(&out).trim().is_empty());
    assert_eq!(
        t.read(".claude/skills/speclink-apply/SKILL.md"),
        sentinel,
        "existing skills must not be overwritten when remote update fails"
    );
}

#[test]
fn workflow_config_subagent_model_is_idempotent() {
    let t = TempRepo::new("subagent-model");
    let init = t.run(&["init", "--force", "--tools", "codex"]);
    assert!(init.status.success(), "{}", stderr_of(&init));

    let set_first = t.run(&[
        "workflow-config",
        "subagent-model",
        "--station",
        "review",
        "--tool",
        "codex",
        "--model",
        "o3",
    ]);
    assert!(
        set_first.status.success(),
        "subagent-model failed: {}",
        stderr_of(&set_first)
    );
    let cfg_path = t.repo.join("openspec").join("config.yaml");
    let content_first = std::fs::read_to_string(&cfg_path).unwrap();
    let mtime_first = std::fs::metadata(&cfg_path).unwrap().modified().unwrap();

    // 確保檔案系統時間解析度可辨識是否重寫
    thread::sleep(Duration::from_millis(50));

    let set_second = t.run(&[
        "workflow-config",
        "subagent-model",
        "--station",
        "review",
        "--tool",
        "codex",
        "--model",
        "o3",
    ]);
    assert!(set_second.status.success(), "{}", stderr_of(&set_second));
    assert_eq!(std::fs::read_to_string(&cfg_path).unwrap(), content_first);
    assert_eq!(
        std::fs::metadata(&cfg_path).unwrap().modified().unwrap(),
        mtime_first,
        "no-op subagent-model write must not touch config mtime"
    );
}

#[test]
fn instructions_specs_reports_spec_locale_and_unknown_artifact_lists_choices() {
    let t = TempRepo::new("spec-locale");
    t.write(
        "openspec/config.yaml",
        "schema: spec-driven\nlocale: tw\nspec_locale: ja\n",
    );
    t.add_change("loc-demo", "## 1. Group\n\n- [ ] 1.1 first\n");

    let specs_out = t.run(&["instructions", "specs", "--change", "loc-demo", "--json"]);
    assert!(specs_out.status.success(), "{}", stderr_of(&specs_out));
    let specs_json = json_of(&specs_out);
    let loc = specs_json["locale"].as_str().unwrap_or("");
    assert!(
        loc == "ja" || loc.contains("日本語") || loc.contains("Japanese"),
        "expected Japanese spec_locale in instructions specs, got: {loc:?}"
    );

    let bad_artifact = t.run(&["instructions", "no-such-artifact", "--change", "loc-demo"]);
    assert!(!bad_artifact.status.success());
    let err = stderr_of(&bad_artifact);
    assert!(
        err.contains("proposal") && err.contains("specs") && err.contains("tasks"),
        "unknown artifact error must list available artifact IDs: {err}"
    );
}

#[test]
fn herdr_terminal_report_respects_opt_out_and_never_affects_exit_or_stdout() {
    let t = TempRepo::new("herdr-report");
    t.add_change("hr-demo", "## 1. Group\n\n- [ ] 1.1 first\n- [ ] 1.2 second\n");

    // 即使在 Herdr 環境內且 HERDR_BIN 不存在，stdout 與退出碼仍須維持成功
    let with_missing_herdr = Command::new(env!("CARGO_BIN_EXE_speclink"))
        .args(["task", "done", "--change", "hr-demo", "1", "--json"])
        .current_dir(&t.repo)
        .env("HERDR_TAB_ID", "tab-1")
        .env("HERDR_BIN", "nonexistent-herdr-bin-xyz")
        .env("USERPROFILE", &t.home)
        .env("HOME", &t.home)
        .env("XDG_CONFIG_HOME", &t.home)
        .output()
        .expect("run speclink");
    assert!(with_missing_herdr.status.success(), "{}", stderr_of(&with_missing_herdr));
    assert_eq!(json_of(&with_missing_herdr)["change"], "hr-demo");

    // SPECLINK_TERMINAL_REPORT=false opt-out 同樣維持成功且不影響 stdout
    let opt_out = Command::new(env!("CARGO_BIN_EXE_speclink"))
        .args(["task", "done", "--change", "hr-demo", "2", "--json"])
        .current_dir(&t.repo)
        .env("HERDR_TAB_ID", "tab-1")
        .env("SPECLINK_TERMINAL_REPORT", "false")
        .env("HERDR_BIN", "nonexistent-herdr-bin-xyz")
        .env("USERPROFILE", &t.home)
        .env("HOME", &t.home)
        .env("XDG_CONFIG_HOME", &t.home)
        .output()
        .expect("run speclink");
    assert!(opt_out.status.success(), "{}", stderr_of(&opt_out));
    assert_eq!(json_of(&opt_out)["change"], "hr-demo");
}

#[test]
fn language_replace_stdin_writes_and_reads_back_via_language_show() {
    use std::io::Write;
    use std::process::Stdio;

    let t = TempRepo::new("lang-replace");
    let expected = "# Language Guide\n\nUse Traditional Chinese.\n";
    let mut child = Command::new(env!("CARGO_BIN_EXE_speclink"))
        .args(["language", "replace", "--stdin", "--json"])
        .current_dir(&t.repo)
        .env("USERPROFILE", &t.home)
        .env("HOME", &t.home)
        .env("XDG_CONFIG_HOME", &t.home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn language replace");
    child.stdin.take().unwrap().write_all(expected.as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success(), "language replace failed: {}", stderr_of(&out));
    let json = json_of(&out);
    assert_eq!(json["updated"], true);

    let show = t.run(&["language", "show"]);
    assert!(show.status.success(), "{}", stderr_of(&show));
    assert_eq!(String::from_utf8_lossy(&show.stdout), expected);
}
