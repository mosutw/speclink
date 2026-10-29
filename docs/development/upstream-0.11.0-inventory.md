# 原作者 0.11.0 整合盤點

Fork 基準：e272d4bebef200559a2604e822a47d6485cffa92；上游：64623a6bfbe866f4ae33e37b918cb39b941f38cb；共同祖先：05e5e65ac043d04e6e0b2a396c0bb0997e52ae70。

以 Fork 為底；本 repo 保持 openspec/。上游自用的資料夾搬移與發版號提交不直接套入，產品功能逐檔整合。

上游相對基準改動 4700 個路徑（含整棵規格樹移動）；Fork 相對基準改動 595 個路徑。

## 實作差異清單

| 路徑 | 來源 | 處置原則 | 實際結果 |
| --- | --- | --- | --- |
| `.github/workflows/ci.yml` | Fork | 保留 Fork 發佈與 CI 守門 | 保留 Fork 原文 |
| `.github/workflows/desktop-build.yml` | Fork | 保留 Fork 發佈與 CI 守門 | 保留 Fork 原文 |
| `.github/workflows/release.yml` | Fork | 保留 Fork 發佈與 CI 守門 | 保留 Fork 原文 |
| `apps/desktop/core/Cargo.toml` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/core/src/cache.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/core/src/diagnostics.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/core/src/discussions.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/core/src/lib.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/core/src/manage.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/core/src/manual.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/core/src/network.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/core/src/project.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/core/src/query.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/core/src/runtime_preferences.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/core/src/search.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/core/src/security.rs` | Fork | 保留既有功能與內容 | 已整合；交會處取捨見下表 |
| `apps/desktop/core/src/settings.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/core/src/testdata/README.md` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/core/src/testdata/proxy-ca.der` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/core/src/testdata/proxy-server-key.der` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/core/src/testdata/proxy-server.der` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/core/src/testfixture.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/core/src/updater.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/core/src/verbs.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/index.html` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/package.json` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src-tauri/Cargo.toml` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src-tauri/capabilities/default.json` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src-tauri/src/connections.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src-tauri/src/lib.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src-tauri/src/panel.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src-tauri/src/portable.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src-tauri/src/remote.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src-tauri/src/runtime_commands.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src-tauri/src/security_commands.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src-tauri/src/updater_commands.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src-tauri/src/watch.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/src-tauri/tauri.conf.json` | 雙方 | 保留 Fork 版本、updater endpoint 與 pubkey | 保留 Fork 原文 |
| `apps/desktop/src/App.tsx` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/App.test.tsx` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/aboutPage.test.tsx` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/appSettingsView.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/assetNotice.test.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/assetPrompt.test.ts` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `apps/desktop/src/__tests__/checkRecords.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/diagnostics.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/documentActionsMenu.test.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/emptyWorkspace.test.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/helpers/assetFixtures.ts` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/messages.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/migrationDialog.test.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/networkDiagnosticsPage.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/projectColors.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/projectRail.test.tsx` | Fork | 保留既有功能與內容 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/projectSettingsView.test.tsx` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/remoteDataSource.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/remoteOpen.test.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/remoteWorkspaceRecovery.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/securityDataPage.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/securityReview.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/session.test.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/settingsColumn.test.tsx` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/store.test.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/tabs.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/themeBoot.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/tray.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/trayPanel.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/updater.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/updatesPage.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/whatsNew.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/__tests__/workspace.test.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/__tests__/workspaceChooser.test.tsx` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/adapter/checkRecords.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/adapter/connections.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/adapter/remoteDataSource.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/adapter/runtime.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/adapter/security.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/adapter/tauriDataSource.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/adapter/updater.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/adapter/workspace.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/assetPrompt.ts` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/builtinTools.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/components/AssetNoticeCard.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/components/AssetNoticeDialog.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/components/ProjectRail.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/components/ProjectSettingsColumn.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/components/WorkspaceChooser.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/components/projectTile.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/core/updater.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/core/whatsNew.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/diagnostics.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/i18n/messages.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/main.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/panel/TrayPanel.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/panel/main.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/projectColors.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/release-notes/release-notes.json` | 雙方 | 保留 Fork 歷史，發版號與新條目留待發版決策 | 保留 Fork 原文 |
| `apps/desktop/src/session.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/settingsCatalog.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/store.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/tabs.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/tray.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/views/AppSettingsView.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/views/ProjectSettingsView.tsx` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/views/project-settings/ContextPage.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/views/project-settings/ReviewRulesPage.tsx` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/views/project-settings/RulesPage.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/views/project-settings/SpeclinkPage.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/views/project-settings/WorkflowPage.tsx` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/views/project-settings/useProjectSettings.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `apps/desktop/src/views/settings/AboutPage.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/views/settings/GeneralPage.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/views/settings/NetworkDiagnosticsPage.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/views/settings/SecurityDataPage.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/desktop/src/views/settings/UpdatesPage.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/server-web/src/__tests__/build.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/server-web/src/__tests__/users.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `apps/server-web/tsconfig.json` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-cli/Cargo.toml` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-cli/src/common.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/src/main.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/src/verbs/config.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/src/verbs/connection.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-cli/src/verbs/init.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/src/verbs/instructions.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/src/verbs/lifecycle.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-cli/src/verbs/mod.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-cli/src/verbs/plan.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/src/verbs/query.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/src/verbs/security.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-cli/src/verbs/station.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-cli/tests/it/archive_evidence_gate.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/archive_merge_gate.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/archive_readiness_gate.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/change_schema.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `crates/adapters/speclink-cli/tests/it/config_fail_closed.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/copilot_tools.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-cli/tests/it/discuss_conclude_auto_archive.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/discuss_content_guard.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/discuss_kind.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/discuss_promote_snapshot.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/discuss_seal.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/discuss_search.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/discuss_slug.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/doc_verbs.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/in_progress_remove.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/init_tools.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/instructions_policy.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/list_delta_capabilities.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/main.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/manual_pages_dir_ignored.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/manual_task_gates.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/meta_fail_closed.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/meta_time_stamps.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `crates/adapters/speclink-cli/tests/it/mode_dispatch.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/new_artifact.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/no_spec_change_lifecycle.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `crates/adapters/speclink-cli/tests/it/plan_verbs.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/reconclude_restale.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/remote_connect.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/remote_drift.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/remote_plan.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/remote_read_path.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/remote_section.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/remote_verb_parity.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/remote_write_path.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/review_verbs.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/schema_commands.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/security.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-cli/tests/it/spec_dir_layout.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `crates/adapters/speclink-cli/tests/it/spec_dir_migration.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `crates/adapters/speclink-cli/tests/it/task_done_stamps.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/task_stable_id.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/task_undone.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/tools_descriptor.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/trace.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/update_downgrade_guard.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `crates/adapters/speclink-cli/tests/it/validate_specs.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/verify_verbs.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/workflow_config.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-cli/tests/it/worktree_overlay.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-node/__test__/actor.spec.ts` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-node/__test__/dispatch-contract.spec.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-node/__test__/helpers.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-node/__test__/render.spec.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-node/__test__/review-verify-verbs.spec.ts` | Fork | 保留既有功能與內容 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-node/__test__/types.spec.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-node/__test__/types/render-api.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-node/__test__/types/tsconfig.json` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-node/__test__/write-path.spec.ts` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-node/index.d.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-node/package-lock.json` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-node/package.json` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/adapters/speclink-node/src/lib.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-node/src/render.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/adapters/speclink-node/src/store_bridge.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/Cargo.toml` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/assets/schema/no-spec-change/fork.schema.yaml` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `crates/engine/speclink-core/assets/schema/no-spec-change/proposal.template.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `crates/engine/speclink-core/assets/schema/spec-driven/fork.schema.yaml` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/assets/schema/spec-driven/proposal.template.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/assets/security/rule-locales.json` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/assets/security/rules.json` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/assets/security/sources.json` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/assets/security/spec_registry.json` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/assets/skills/apply.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/assets/skills/archive.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/assets/skills/audit.md` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/assets/skills/baseline.md` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/assets/skills/commit.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/assets/skills/config.md` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/assets/skills/discuss.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/assets/skills/drift.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/assets/skills/improve.md` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/assets/skills/ingest.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/assets/skills/propose.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/assets/skills/quality.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/assets/skills/review.md` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/assets/skills/security-review.md` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/assets/skills/verify.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/command/mod.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/command/tests.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/command/typed.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/demo.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/lib.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/lifecycle/archive.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/lifecycle/archive/tests.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/lifecycle/change_schema.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `crates/engine/speclink-core/src/lifecycle/cost.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `crates/engine/speclink-core/src/lifecycle/discard.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/lifecycle/discuss.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/lifecycle/discuss/tests.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/lifecycle/inprogress.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/lifecycle/mod.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/lifecycle/model.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/lifecycle/newcmd.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/lifecycle/plan.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/lifecycle/tasks.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/quality/analyzer.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/quality/history.rs` | Fork | 保留既有功能與內容 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/quality/history/archive.rs` | Fork | 保留既有功能與內容 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/quality/mod.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/quality/station.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/quality/station/tests.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/quality/validate.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/quality/validate/tests.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/security/acceptance.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/security/boundary.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/security/data.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/security/input.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/security/knowledge.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/security/mod.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/security/policy.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/security/prepared.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/security/report.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/security/report/deep.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/security/rules.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/security/standards.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-core/src/store.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/testkit.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/teststore.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/util.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/workspace/config.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/workspace/config/tests.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/workspace/init.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/workspace/init/tests.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/workspace/instructions.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/workspace/migrate.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `crates/engine/speclink-core/src/workspace/mod.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/workspace/schema.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/workspace/skills.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/src/workspace/workspace.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/golden/assets.lock` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/golden/claude-worktree.snapshot.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/golden/claude.snapshot.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/golden/codex.snapshot.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/golden/copilot.snapshot.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/golden/neutral-cli.snapshot.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/golden/neutral-tool-call.snapshot.md` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/golden/security-review-antigravity.snapshot.md` | Fork | 保留既有功能與內容 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/golden/security-review-claude.snapshot.md` | Fork | 保留既有功能與內容 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/golden/security-review-codex.snapshot.md` | Fork | 保留既有功能與內容 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/golden/security-review-copilot.snapshot.md` | Fork | 保留既有功能與內容 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/it/no_direct_fs.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/it/render_golden.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-core/tests/it/skill_authoring.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-fs/Cargo.toml` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-fs/src/archive_checks.rs` | Fork | 保留既有功能與內容 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-fs/src/checks.rs` | Fork | 保留既有功能與內容 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-fs/src/layout.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-fs/src/lib.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/engine/speclink-fs/src/security.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-fs/src/security/knowledge.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-fs/src/security/prepared.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-fs/src/security/reports.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-fs/src/security/scan.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-fs/src/security_packages.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-fs/tests/fixtures/security-guidance.zip` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/engine/speclink-fs/tests/store_fs.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-host/Cargo.toml` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-host/src/bridge.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-host/src/commit.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-host/src/context.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-host/src/drift.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-host/src/history.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-host/src/lib.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-host/src/network.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-host/src/policy.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-host/src/portable.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-host/src/projection.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-host/src/security.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-host/src/security/deep.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-host/src/security/remote.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-host/src/security_data.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-host/src/security_paths.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-host/src/worktree.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-host/tests/security_remote.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-server/Cargo.toml` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-server/src/api.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-server/src/api/context.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-server/src/api/error.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-server/src/api/events.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-server/src/api/read_api.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-server/src/api/routes.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-server/src/api/security.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-server/src/api/verb.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-server/src/app.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-server/src/identity/auth.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-server/tests/it/api/context_api.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-server/tests/it/api/plan_api.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-server/tests/it/api/policy_write.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-server/tests/it/api/query_routes.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-server/tests/it/api/read_api.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-server/tests/it/api/review_api.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/host/speclink-server/tests/it/api/verb_api.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-server/tests/it/api/write_guard.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-server/tests/it/e2e_cli.rs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `crates/host/speclink-server/tests/it/phase2_chain.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/protocol/speclink-protocol/src/binding.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/protocol/speclink-protocol/src/command.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/protocol/speclink-protocol/src/error.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/protocol/speclink-protocol/src/lib.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/protocol/speclink-protocol/src/query.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/protocol/speclink-protocol/src/security.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/protocol/speclink-remote/Cargo.toml` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/protocol/speclink-remote/src/auth.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/protocol/speclink-remote/src/client.rs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `crates/protocol/speclink-remote/src/credentials.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/protocol/speclink-remote/src/lib.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/protocol/speclink-remote/tests/it/client_errors.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/protocol/speclink-remote/tests/it/typed_client.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/store/speclink-store-fs/src/lib.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/store/speclink-store-postgres/src/lib.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/store/speclink-store-sqlite/src/lib.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/store/speclink-store/src/conformance/mod.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/store/speclink-store/src/memory.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/store/speclink-store/src/types.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `crates/store/speclink-store/src/uow.rs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/analyze-reports-only/scaffold.sh` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/apply-leaves-manual-task/graders/code-task-checked.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/apply-leaves-manual-task/graders/manual-task-left-open.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/archive-folds-delta/graders/no-hand-edit.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/archive-folds-delta/graders/no-hand-write.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/archive-folds-delta/graders/spec-merged.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/archive-folds-delta/prompt.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/baseline-map-before-specs/graders/no-specs-yet.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/drift-reports-only/scaffold.sh` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/fixtures.sh` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/ingest-folds-new-requirement/graders/delta-has-new-requirement.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/manual-generates-pages/graders/manual-page-written.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/manual-generates-pages/prompt.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/manual-generates-pages/scaffold.sh` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/propose-from-discussion/graders/manual-marker-after-checkbox.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/propose-from-discussion/graders/no-marker-after-number.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/security-review-missing-local-knowledge/case.yaml` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/security-review-missing-local-knowledge/graders/no-resource-fetch.md` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/security-review-missing-local-knowledge/graders/reports-missing-knowledge.md` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/security-review-missing-local-knowledge/prompt.md` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `integrations/claude-code/skill-evals/evals/security-review-missing-local-knowledge/scaffold.sh` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `integrations/claude-code/speclink-skills/README.en.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `integrations/claude-code/speclink-skills/README.md` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `integrations/claude-code/speclink-skills/hooks/groups.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/archivedDetailPage.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/archivedList.test.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `packages/ui/src/__tests__/baselineSmellHelp.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/changeDetailPage.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/discussionDetailPage.test.tsx` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `packages/ui/src/__tests__/forkThemeMigration.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/improveBadge.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/processCost.test.ts` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `packages/ui/src/__tests__/processCostSummary.test.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `packages/ui/src/__tests__/proposalDoc.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/reviewBadge.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/securityReviewPanel.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/sonner.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/specDetailPage.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/theme.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/themePreference.test.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/__tests__/ui.test.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `packages/ui/src/__tests__/verifyBadge.test.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/adapter.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `packages/ui/src/checkRecords.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/components/ArchivedDetailPage.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/components/ArchivedList.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `packages/ui/src/components/BaselineSmellList.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/components/ChangeDetailPage.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/components/CheckRecords.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/components/DiscussionDetailPage.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `packages/ui/src/components/ProcessCostSummary.tsx` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `packages/ui/src/components/ProposalDoc.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/components/ReviewArchiveDialog.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/components/SecurityReviewPanel.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/components/ui/sonner.tsx` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/i18n.tsx` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `packages/ui/src/index.ts` | 雙方 | 保留 Fork，逐段合入上游新功能 | 已整合；交會處取捨見下表 |
| `packages/ui/src/processCost.ts` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 新增上游功能／測試 |
| `packages/ui/src/security.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/theme.css` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/src/theme.ts` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `packages/ui/tsconfig.json` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `scripts/ci/ci-scope.mjs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `scripts/ci/ci-scope.test.mjs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `scripts/claude-code/skill-groups.test.mjs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `scripts/desktop/desktop-ci-gate.mjs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `scripts/desktop/desktop-ci-gate.test.mjs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `scripts/desktop/desktop-release-version.mjs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `scripts/desktop/desktop-release.test.mjs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `scripts/desktop/windows-portable.mjs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `scripts/desktop/windows-portable.test.mjs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `scripts/docs/docs-screenshots.mjs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `scripts/docs/vocabulary-guard.test.mjs` | 上游 | 採入上游功能及測試，依本 Fork 路徑預設調整 | 已整合；交會處取捨見下表 |
| `scripts/release/delivery-gate.test.mjs` | 雙方 | 保留 Fork，逐段合入上游新功能 | 保留 Fork 原文 |
| `scripts/release/release-notes.mjs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |
| `scripts/release/release-notes.test.mjs` | Fork | 保留既有功能與內容 | 保留 Fork 原文 |

## 開工前使用者變更

.speclink.yaml 的 tools 新增 antigravity，及 .agent/ 受管技能在開工前已存在；保留並在此基礎補入 spec_dir。提案與來源討論在開工前尚未提交，不覆蓋或丟棄。

## 驗證紀錄

各階段的實測結果於通過期追加；此盤點不代表尚未執行的測試已通過。

## 實際交會處取捨

| 交會處 | 整合結果 |
| --- | --- |
| workspace init、CLI init、desktop project、spec-dir 測試 | 入口預設選 openspec 並明寫；update 先過工具／降級守門，再補設定，再交給上游遷移規劃。舊 OpenSpec 原地採用，已使用 speclink 或 custom 的專案不反向搬移。probe 不對將原地補設定的專案聲稱要搬目錄。 |
| core model、FS layout/lib、Host bridge/projection | 採入 .meta.yaml 優先及 .openspec.yaml fallback；實際無法讀新檔不回退掩蓋。遠端活動 metadata 維持 DocumentId::ChangeMeta，封存鍵讀新優先、舊鍵 fallback，不批次轉寫遠端資料。 |
| core quality station/history、FS checks/archive_checks | 新欄位放進既有 ChecksCommit，保留完整工單、索引、規則與安全證據；同時比較來源舊檔與目的新檔。journal 恢復允許兩個檔名；新增競爭案例拒絕中途出現的新檔。稽核另重現「兩次讀舊檔之間被改」後，改用產生章的原文（含當時缺席的狀態）當 CAS 基準。 |
| archive 與 history/archive | 保留 Fork 的封存交易，archived_at 寫入前比較原 metadata，交易期望值取實際寫出的同一份文字，歷史封存摘要與同一時間一致；不以重讀掩蓋競爭。 |
| schema runtime、Protocol、Remote、Server、CLI | 採入兩個內建 schema、change schema 型別與端點、no-spec-change 的拒絕 delta 守門；同值零事件、reader/revision/離線零寫入、清五個驗證章欄位但留輪數與歷史。 |
| Node SDK | 保留 Antigravity／neutral 語法及 Host 品質歷史。除了上游 Store 能力，補上 SDK change schema 路由，測試先重現 unsupported verb，再確認切換、同值及帶 delta 拒絕。 |
| desktop settings、useProjectSettings、ProjectSettingsView、App | 接上實際設定檔路徑與 schema；保留 Fork 專案順序／底色、外觀、Proxy 診斷與安全設定。安全審查入口改走共用的實際規格路徑解析。D8 新增本機 `openspec/` → `speclink/` 單向轉換，需明確確認，完成後不提供反向選項；custom 與 remote 沒有轉換入口。 |
| spec-driven fork.schema.yaml、propose／ingest／verify assets | 合入兩種流程分流，保留 Fork 提案列點、安全指引與語言預設；不複製上游生成檔覆蓋本地工具差異。 |
| quality asset、golden、所有受管技能 | 合入主分支交集及 CI 提醒，依設計追加 -z/NUL 路徑分隔；先測出缺失，再重產。資產版號為 v1.53.1，用來區分本 Fork 修改後的內容，與產品發佈版號分開。 |
| Claude Code hooks 與 register.test.tsx | 對照後無須改動：透過 CLI JSON 取得 plan/list/status/tickets，不直接讀任一 metadata 檔名。withCommand/withArgument 與 archived 完成標記的測試仍保留原文；只更新外掛 README 的資料檔說明。執行驗證的限制見下方。 |
| README、docs、manual | 合入新定位、schema、時間／輪數、成本摘要、品質提醒與資料檔說明；明寫本 Fork 的 openspec 預設，維持四工具及安全流程。修正路徑時分清規格資料夾、speclink/* 分支與遠端 speclink/ 投影，不把三者一起改名。 |

## Audit 搜尋與邊界核對

- `spec_dir.is_some`：唯一 runtime 命中在 workspace/migrate.rs 的明確設定／remote 免遷移守門。desktop remote 匯入改比較解析後的目錄名，接受 openspec、speclink，custom 與兩目錄並存於上傳前拒絕。
- `DEFAULT_SPEC_DIR`：保留上游引擎的 speclink fallback 及遠端虛擬文件路徑。Fork 的 CLI／desktop 新初始化與 SDK 技能預設使用 openspec；既有專案一律用共用解析器，不靠字串替換猜位置。
- `.openspec.yaml`：保留在讀取 fallback、舊檔遷移、journal 恢復與相容測試。品質交易刪除舊檔有來源雜湊；不得只把寫入鍵換名而漏掉 expected。未改寫本 repo 既有封存記錄。
- `.meta.yaml`：唯一新寫入名稱。Host commit_checks 映射至 typed metadata；遠端封存仍用原有 Artifact 身分規則，新舊讀取有測試。
- schema 名稱：先驗 kebab-case 再解析／接路徑；../、斜線、空名、換行等輸入拒絕零寫入。本機沿 project → user → built-in，沒有本機專案的遠端只用內建，不借用 client/server user schema。
- 預設設定：先讀取與驗證設定及工具，再寫 spec_dir；壞 YAML、未知工具描述子、降級守門與補設定寫失敗都不搬資料、不再生技能。補鍵沿 KeyLines 與既有原子寫入函式，保留註解／未知鍵／工具，重試不重複。
- 三種誤用角度：惡意輸入不能用 schema 名稱讀任意路徑；偷省步驟不能用 accept 繞過 Fork 安全驗收；誤解目錄設定不會先搬走 openspec 才補鍵，也不會把流程成本 0 當成缺資料。相關拒絕與競爭測試通過。
- D8 模式轉換搜尋：`spec_dir.is_some` 的通用 update 守門仍避免自動搬動已明確設定的專案；專用 `convert_spec_dir_to_speclink` 只接受實際來源為 `openspec/`、目的 `speclink/` 不存在的本機專案。搜尋 `DEFAULT_SPEC_DIR` 確認保留上游 `speclink` fallback 與既有目錄辨識；桌面選項依實際路徑顯示 OpenSpec／Speclink，自訂路徑唯讀呈現；remote 不渲染本機設定頁，也沒有轉換 provider/命令。搜尋 `convert_spec_dir_to_openspec`、OpenSpec 目的地轉換入口及 UI 反向選項均無結果。轉換不重寫工單或 checks；receipt 僅記錄來源與目的設定以供前向復原，先拒絕既有目的地，custom／remote／反向要求都不寫入。 |

## Fork 保留矩陣

| 項目 | 證據與結果 |
| --- | --- |
| Fork 更新來源、現有 updater 公鑰、產品版號 | tauri.conf.json 與 HEAD blob 相同，仍為 mosutw/speclink 與 0.10.2；未套入作者的 bump。Cargo.toml、Cargo.lock 與套件清單也未變。 |
| GHCR、CI、三平台與 Windows portable | .github/workflows/ 與 scripts/desktop/ 全部未改。Node scripts 386 項通過，含 delivery、desktop release、Windows portable 契約；本機沒有宣稱 Windows/Linux 實機建置。 |
| 更新日誌歷史 | release-notes.json 與兩份 CHANGELOG 逐位元保留；執行既有 renderer 後內容無差異，--check 通過，沒有預建 0.11.0 發佈條目。 |
| Fork 安全與品質歷史 | Core/FS/Host/Server/desktop-core 既有安全、完整歷史及交易測試通過；新增舊檔寫入競爭回歸。安全政策原值、audit/tdd 選項不變。 |
| 四種工具與語言 | .speclink.yaml 保留 copilot/codex/claude 及開工前新增的 antigravity；四套受管技能由整合引擎更新，既有系統／Server 語言決策與安全技能留在原稿及 golden 測試。 |
| 桌面原有功能 | UI 及 desktop-core 全套通過，涵蓋主題、專案順序／底色、安全資料路徑、品質歷史及 Proxy 診斷；TypeScript 檢查與 Tauri cargo check 通過。 |
| 現有內容與使用者修改 | openspec/specs、changes/archive、discussions/archive 逐位元未變；原本未提交的提案、來源討論與 .agent/ 保留。沒有 merge/commit/push/tag/release。 |

## 實測紀錄（2026-10-28）

| 執行命令／範圍 | 結果 |
| --- | --- |
| cargo test -p speclink-core | 最終 1,185 單元＋100 整合通過；另 1 個官方資料下載案例依原設定 ignored，包含補設定寫失敗及舊檔中途建立／改動的拒絕案例。D8 轉換 guard、回復與同向重試案例包含在內。 |
| cargo test -p speclink-fs | 58 單元＋1 舊無 metadata 蓋章＋27 Store 整合通過；1 個須下載官方資料的案例依原設定 ignored。 |
| cargo test -p speclink-host -- --test-threads=1 | 169 單元＋5 雙路徑＋1 安全發布整合通過。串行避免既有共用安全資料鎖的測試競爭；修正 CAS 後重跑通過。 |
| cargo test -p speclink-protocol / speclink-remote | Protocol 78＋20，Remote 104，全通過。 |
| cargo test -p speclink-server（與其餘 Host stack 串行） | 35 單元＋360 整合通過；1 個 helper 由主測試另啟程序。PostgreSQL 3 個外部服務案例因未設 SPECLINK_TEST_POSTGRES_URL skipped。 |
| cargo test -p speclink-desktop-core | 最終 374 全通過，包含 D8 轉換 snapshot、custom／remote 拒絕案例；localhost 測試在沙箱外執行。 |
| cargo test -p speclink-cli | 最終 616 全通過，含 default/metadata/schema/remote/worktree/security 回歸。localhost 測試在沙箱外執行，沒有把沙箱的連線拒絕當產品失敗。 |
| npm test -w packages/ui / apps/desktop / apps/server-web | 1,370／1,366／244 全通過；桌面全套包含 D8 確認取消、成功、失敗重試、remote/custom 限制及雙語提示。 |
| npm --prefix crates/adapters/speclink-node run build && npm --prefix crates/adapters/speclink-node test | 最終 63 項全通過，含 schema 切換與公開 TypeScript 宣告。 |
| cargo test -p speclink-core --test it render_golden:: | 69 項通過；先以 UPDATE_GOLDEN、UPDATE_ASSETS_LOCK 重產並審閱，最後不帶更新開關確認。 |
| target/debug/speclink update | claude、codex、copilot、antigravity 四套成功更新；openspec/ 原地不動。 |
| node --test scripts/*.test.mjs scripts/*/*.test.mjs | 最終 386 全通過。 |
| D8 搜尋與設定核對 | 本 repo `.speclink.yaml` 仍為 `spec_dir: openspec`；只有 `convert_spec_dir_to_speclink`／`convertSpecDirToSpeclink` 前向入口，搜尋沒有 OpenSpec 反向命令或 UI 選項。專用流程拒絕 remote、custom、已存在目的地，receipt 不儲存工單或 checks；沒有發現本 repo 被轉換。 |
| tsc --noEmit -p apps/desktop、cargo check -p speclink-desktop、兩個 app 的 npm build | 全通過。 |
| release-notes-render.mjs --check、git diff --check、validate merge-upstream-0-11-0 --strict | 通過；validate 只有新 capability 名稱相似提醒，spec-dir-migration 指本機佈局，與 remote 的 workspace-migration 及格式驗證 spec-validation 不同。 |
| claude plugin test integrations/claude-code/speclink-skills | 使用者確認已在可用 Claude Code 環境執行並通過。此機本機 CLI 為 1.0.86（README 要求至少 2.1.290），本次沙箱外重跑回覆 `Credit balance is too low`，故無本機測試輸出；依使用者確認記錄外部環境結果。 |

RED 證據：初次新 schema/命令缺席、舊資料檔交易恢復 unsafe target、43 個 archive 舊 CAS 衝突、桌面初始化預設假設、技能 golden 差異皆先重現再整合；稽核另用 `legacy_stamp_cas_uses_the_metadata_used_to_build_the_stamp` 重現覆寫競爭，用 `quality_skill_warns_before_stale_stamps` 重現缺少 NUL 路徑分隔，SDK schema 測試重現 unsupported verb。修正後對應套件通過。未執行線上安全資料下載、PostgreSQL 實例、多平台 CI 與原生 GUI 手動操作，這些不列為本機測試通過。
