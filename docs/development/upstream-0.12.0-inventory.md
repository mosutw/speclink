# 原作者 0.12.0 整合盤點

Fork：86dd8c9af661827cb0e2615c098500048dfd5615；已整合上游：64623a6bfbe866f4ae33e37b918cb39b941f38cb；本次上游：26de3b9fb3342da43b05a2dbb790ff2f942bcde8。

開發分支：feat/merge-upstream-0-12-0。開工前已有本提案與來源討論未提交，保留其內容。

只納入兩版之間的新增差異；程式交會處以 Fork 為底三方比對，衝突逐段處理。

## 來源清單

| 路徑 | 處置 |
| --- | --- |
| `.gitignore` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `CHANGELOG.md` | 保留 Fork 版本與更新日誌歷史，不採上游發版提交 |
| `CHANGELOG.zh-TW.md` | 保留 Fork 版本與更新日誌歷史，不採上游發版提交 |
| `Cargo.lock` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `Cargo.toml` | 保留 Fork 版本與更新日誌歷史，不採上游發版提交 |
| `README.md` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `README.zh-TW.md` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/core/src/discussions.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/core/src/manage.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/core/src/query.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/package.json` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src-tauri/Cargo.toml` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src-tauri/capabilities/default.json` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src-tauri/src/lib.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src-tauri/src/remote.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src-tauri/tauri.conf.json` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src-tauri/tests/it/remote_data.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/App.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/__tests__/App.test.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/__tests__/deepLink.test.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/__tests__/helpers/changeList.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/__tests__/remoteCapabilities.test.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/__tests__/remoteDataSource.test.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/__tests__/remoteResilience.test.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/__tests__/store.test.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/__tests__/tauriDataSource.test.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/__tests__/workspace.test.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/adapter/remoteDataSource.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/adapter/tauriDataSource.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/components/EmptyWorkspace.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/components/RecentList.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/components/WorkspaceChooser.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/deepLink.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/i18n/messages.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/release-notes/release-notes.json` | 保留 Fork 版本與更新日誌歷史，不採上游發版提交 |
| `apps/desktop/src/store.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/desktop/src/tray.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `apps/server-web/src/__tests__/users.test.tsx` | Fork 已有等效的等待分頁修正，保留本地原文 |
| `crates/adapters/speclink-cli/src/verbs/plan.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/adapters/speclink-cli/tests/it/plan_verbs.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/adapters/speclink-cli/tests/it/remote_plan.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/adapters/speclink-cli/tests/it/remote_read_path.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/engine/speclink-core/src/command/mod.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/engine/speclink-core/src/lifecycle/discuss.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/engine/speclink-core/src/lifecycle/discuss/cuts.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/engine/speclink-core/src/lifecycle/discuss/tests.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/engine/speclink-core/src/lifecycle/plan.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/host/speclink-server/src/api/routes.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/host/speclink-server/tests/it/admin/web_api.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/host/speclink-server/tests/it/api/discussion_routes.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/host/speclink-server/tests/it/api/plan_api.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/protocol/speclink-protocol/src/query.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/protocol/speclink-remote/src/convert.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `crates/protocol/speclink-remote/tests/it/typed_client.rs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `docs/assets/screenshots/en/herdr-panel.png` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `docs/assets/screenshots/en/herdr-skill-picker.png` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `docs/assets/screenshots/zh-TW/herdr-panel.png` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `docs/assets/screenshots/zh-TW/herdr-skill-picker.png` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `docs/development.md` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `docs/development.zh-TW.md` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `docs/product-status.md` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `docs/product-status.zh-TW.md` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `docs/roadmap.md` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `docs/roadmap.zh-TW.md` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/claude-code/speclink-skills/.claude-plugin/plugin.json` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/claude-code/speclink-skills/README.en.md` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/claude-code/speclink-skills/README.md` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/claude-code/speclink-skills/hooks/focus.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/claude-code/speclink-skills/hooks/register.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/claude-code/speclink-skills/tests/register.test.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/README.en.md` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/README.md` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/agents-list/speclink-rows.js` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/keys.json` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink-new/main.js` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink-new/mod.json` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/column-sync.js` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/i18n.js` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/badge-alert.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/badge-check.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/branch.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/check.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/chevron-down.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/chevron-right.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/copy.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/logo.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/message-plus.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/roadmap.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/shield-alert.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/shield-check.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/shield.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/spinner.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/stamp.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/x.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/icons/zap.svg` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/label-skill.mjs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/main.js` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/mod.json` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/model.js` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/package.json` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/shared.js` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/mods/speclink/skill-picker.js` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/package.json` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `integrations/herdr/spaces-badge/stage-badge.js` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `package-lock.json` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/__tests__/archivedDetailPage.test.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/__tests__/boardSearchBar.test.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/__tests__/conclusionDoc.test.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/__tests__/discussionCuts.test.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/__tests__/discussionDetailPage.test.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/__tests__/discussionModel.test.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/__tests__/engineCuts.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/__tests__/improveBadge.test.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/__tests__/kanban.test.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/__tests__/promotedDoc.test.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/__tests__/roadmapModel.test.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/__tests__/roadmapView.test.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/adapter.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/components/ArchivedDetailPage.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/components/BoardSearchBar.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/components/ConclusionDoc.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/components/DiscussionDetailPage.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/components/KanbanBoard.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/components/RoadmapView.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/components/discussionDocSummary.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/discussionModel.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/i18n.tsx` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/index.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `packages/ui/src/roadmapModel.ts` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `scripts/docs/docs-screenshots.mjs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `scripts/docs/docs-screenshots.test.mjs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `scripts/herdr/dock-panel.mjs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `scripts/herdr/panel-model.test.mjs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `scripts/npm/npm-cli-launcher.test.mjs` | 已整合新增差異；Fork 交會處與驗證結果見下方 |
| `speclink/changes/archive/2026-11-03-desktop-deep-link/.evidence.json` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-desktop-deep-link/.meta.yaml` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-desktop-deep-link/design.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-desktop-deep-link/proposal.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-desktop-deep-link/specs/desktop-app/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-desktop-deep-link/tasks.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-plan-unpromoted-cuts/.evidence.json` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-plan-unpromoted-cuts/.meta.yaml` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-plan-unpromoted-cuts/design.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-plan-unpromoted-cuts/proposal.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-plan-unpromoted-cuts/specs/change-plan/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-plan-unpromoted-cuts/specs/client-protocol/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-plan-unpromoted-cuts/specs/server-verb-api/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-03-plan-unpromoted-cuts/tasks.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-project-deep-link/.evidence.json` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-project-deep-link/.meta.yaml` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-project-deep-link/design.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-project-deep-link/proposal.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-project-deep-link/specs/desktop-app/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-project-deep-link/tasks.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-roadmap/.evidence.json` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-roadmap/.meta.yaml` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-roadmap/design.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-roadmap/proposal.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-roadmap/specs/client-protocol/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-roadmap/specs/desktop-app/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-roadmap/specs/desktop-roadmap/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-desktop-roadmap/tasks.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-discussion-cuts-from-engine/.evidence.json` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-discussion-cuts-from-engine/.meta.yaml` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-discussion-cuts-from-engine/design.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-discussion-cuts-from-engine/proposal.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-discussion-cuts-from-engine/specs/client-protocol/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-discussion-cuts-from-engine/specs/desktop-app/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-discussion-cuts-from-engine/specs/server-verb-api/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/changes/archive/2026-11-04-discussion-cuts-from-engine/tasks.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/discussions/archive/2026-11-03-herdr-speclink-panel.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/manual/about.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/manual/desktop-browse.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/manual/desktop-deep-link.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/manual/desktop-roadmap.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/manual/index.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/manual/plan.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/specs/change-plan/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/specs/client-protocol/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/specs/desktop-app/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/specs/desktop-roadmap/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |
| `speclink/specs/server-verb-api/spec.md` | 不搬入上游規格樹／歷史；本次以 delta 與現有 openspec 手冊整合 |

## Fork 保留矩陣

- spec_dir: openspec、自訂路徑、既有單向轉換，不改寫歷史。
- 安全審查、標準資料、驗收守門、品質與安全紀錄。
- Antigravity 功能保留，錯誤 .agent 路徑依 issue #15 修正。
- 系統／Server 語言、提案條列、外觀、Proxy 診斷、專案底色與排序。
- 0.11.2 產品版號、Fork updater endpoint／pubkey、Windows portable、CI 與發佈目的地。

## 驗證與交會處

本機整合與驗證已完成；外部宿主及其他平台未執行的驗證另列，不計為通過。


## 已核對的整合交會處

- 三方比對以 0.11.0 為基準，只套入 0.12.0 新增段落。Tauri Builder 同時保留 portable context、安全／updater jobs，並在外掛前註冊 single-instance；版號維持 0.11.2。
- Protocol 保留 Fork 安全及品質歷史契約，新增 cuts、next、unpromotedCuts；CLI discuss show 仍維持原鍵集合。
- ArchivedDetailPage 保留 CheckRecordsAdapter；UI 測試保留 Fork 的明確型別，增加引擎分刀 payload。
- App／store 測試的新增尾段同時保留雙方案例，不丟棄 Fork 設定、草稿與封存預覽測試。
- Server Web users.test.tsx 的邀請中分頁等待修正已存在於 Fork，保留現有寫法；上游其他兩處穩定性測試修正已納入。
- Herdr 的技能清單補上 Fork security-review，僅供使用者明確選用，不自動執行；安全驗收仍由引擎守門。新增可選變更測試先失敗後修正通過。
- 手冊不搬入上游 about.md 的能力數量與自用歷史，保留 Fork 說明並加兩個新頁入口。其餘手冊整合新內容、沿 openspec 路徑，既有安全章節保留。
- Claude Code 本機為 1.0.86，上游外掛要求 2.1.290 以上，且缺外掛宿主產生的型別。未執行／不宣稱 claude plugin test、validate 或宿主型別檢查通過；沒有改使用者 ~/.claude.json 或升級外部 CLI。新增 apps/desktop/src/__tests__/claudeFocus.test.ts，從舊版原始碼確認 discuss 保留舊焦點的錯誤，再以現版通過四個獨立測試。完整外掛互動與 Herdr 實際安裝仍未在本機驗證。

## 已通過的驗證

- 上游測試 RED：core 缺 cuts／record_cuts，UI 及 desktop 缺路線圖與 deep-link，Herdr 缺 model；issue #15 的舊路徑、共用內容、外部連結與升級失敗保留來源皆已重現。
- core lib：1241 passed、1 個既有 ignored；後續新增的舊目錄較新版守門另有 RED／GREEN 記錄。
- packages/ui：86 檔、1410 測試通過。
- apps/desktop：54 檔、1469 測試通過（maxWorkers=2，避免與 Rust 編譯同時競爭造成原本 5 秒限時測試逾時）。
- apps/server-web：21 檔、244 測試通過。
- CLI：621 整合測試通過；Server：376；desktop-core：83、22；Protocol：35；remote：104（其餘 unit／doc 結果隨最終驗證補記）。本機 socket 測試經允許在沙盒外執行，不把原先 PermissionDenied 誤判程式失敗。
- 桌面及 Server Web Vite build、桌面 TypeScript noEmit 與 cargo check -p speclink-desktop 通過。
- scripts 整組：527 通過（最後加入焦點測試後會再做最終檢查）。

- 焦點純邏輯測試放在既有桌面 Vitest 環境，使用既有 Vite 相依的 esbuild 在記憶體編譯 TypeScript，略過本機缺少的外掛宿主 tsconfig，沒有替 scripts 守門引入 esbuild 或 npm 安裝需求；CI 的第一階段仍可零外部相依執行。


## issue #15 同類位置排查

- 生成與同步：workspace/skills.rs、workspace/init.rs 一併改共用目錄；config.rs 的 .agent/skills 僅保留為遷移保留名，避免舊清理和自訂描述子碰撞。
- 探測與取消：以同一實體目錄寫入及比對，逐工具保留方向，路徑去重，取消任一種不誤刪。摘要仍依 claude、codex、copilot、antigravity 排序；舊目錄清理改成遷移說明，不把仍選取的工具誤報為已取消。
- UI／SDK／checkout：builtinTools.ts、三組前端工具選擇測試、Node render.spec.ts、Tauri connections.rs 的 checkout 綁定案例一併修正。最後一項原有測試先確認因期待 .agent 失敗，再改驗共用檔案及身分前言。
- 文件：configuration、getting-started 與 sdk-node 的中英版本均更新；configuration 留下的 .agent 是升級說明，不是新生成位置。先前 0.11.0 盤點與 openspec 封存歷史保留原文。
- .agent/skills 的受管技能已由整合後引擎清理，.agents 只有一套共用內容；.claude、.github 仍由引擎再生，未改使用者全域設定。

## 內嵌 audit 檢查

- 壞輸入／誤用：網址嚴格解碼、單一 project、絕對路徑及不可取消浮層守門沿上游；remote 連結先交既有選擇器，確認前零寫入；不增加驗收旁路。
- 複製範例：共用技能前言明訂當前宿主與 --agent 替換規則；neutral、Claude、Copilot 渲染維持原語意。摘要與型別回歸測試避免把現有使用者的選取狀態報錯。
- 目錄邊界：新 .agents 路徑在技能寫入前驗證，舊目錄清理不穿透父目錄、技能根或子目錄連結；Windows 提供 junction fixture，但本機僅執行 Unix 案例。新位置寫失敗保留舊來源、清理失敗保留新內容並可重試、舊較新版拒絕降級，均有自動測試。
- Herdr 的命令參數以 argv 傳入，包含貼上腳本的資料參數，不把變更名稱拼進 shell 原始碼；面板的推薦不取代引擎安全、認證、revision 或封存守門。


## 最終驗證結果

- core：1244 unit 測試、100 integration 測試通過，保留 1 個既有 ignored。最新 CLI：27 unit、621 integration 測試通過。Host 與 FS 的受影響完整 package 測試通過（Host 170 integration；FS 58 unit，1 個既有 ignored，其餘測試亦通過）。
- Protocol、typed remote、Server 與 desktop-core package 測試通過；Tauri 殼另跑 86 個 unit 測試全部通過，含新共用目錄綁定。
- render_golden 69 個守門全部通過，完整 core integration 的 100 個亦通過；資產版號 v1.54.0 與鎖、快照同批更新，產品仍為 0.11.2。
- Node SDK 先建置 napi binding，再執行 9 檔／63 個測試，全數通過；型別測試的既有逾時修補保留。CLI 與 binding 先對齊同一來源後比對，避免舊 debug CLI 造成假差異。
- UI 86 檔／1410、桌面 54 檔／1469、Server Web 21 檔／244 全部通過；另加入的 Claude 焦點獨立測試在 Node Vitest 環境 4 個通過，不依賴 Claude 宿主安裝，沒有讓 scripts 守門依賴 npm 套件。
- 桌面／Server Web production build、桌面 TypeScript noEmit、cargo check -p speclink-desktop 通過。
- 腳本整組最新 527 測試通過；最後文件紀錄完成後再次執行，結果以最終任務驗證為準。
- 受管技能已以新 CLI 成功同步兩次；第二次摘要依 claude、codex、copilot、antigravity 排列，沒有再把選取中的 Antigravity 誤報取消，.agent/skills 的舊受管副本已清理。
- Fork 產品版號、兩份 CHANGELOG、release-notes.json、updater endpoint／pubkey、.speclink.yaml 與發佈 workflow 未變；只在 tauri.conf.json 加 deep-link 設定。正式規格與既有封存歷史未改寫。沒有 commit、push、tag、release、建立 PR 或安裝外部 Herdr。
- macOS 本機測試不等同 Windows／Linux 實機驗收；Windows junction fixture 由平台 CI 執行。Herdr 實際安裝、完整 Claude Code 2.1.290+ 外掛互動與宿主型別檢查仍未在本機執行。

- 最後新增舊目錄不可讀案例：先重現 PermissionDenied 被略過而誤報成功，再讓清理回報含路徑的錯誤。最新 core lib 1245 通過、1 個既有 ignored；使用者內容保留且權限恢復後重跑成功。

- 最後目錄錯誤修正後，CLI 與桌面 Rust package 再驗通過，包含桌面 86 unit 與 74 integration（含 Phase 3、遠端恢復與讀寫守門）；腳本 527、桌面 TypeScript noEmit、Git 差異空白檢查與 Cargo --locked metadata 均通過。58 份受管技能均為 v1.54.0，預設仍為 openspec。
