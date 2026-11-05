## Why

- 透過 AI 代理操作 SDD 的開發者、PO 與 PM，需要在討論轉提案、排程開工與品質收尾時使用原作者 v0.12.0 的路線圖、專案連結與 Herdr 面板，同時保留 Fork 0.11.2 已有的安全審查、品質歷史、桌面客製與獨立發佈來源。
- issue #15 回報 Antigravity 的介面和生成目錄誤用 .agent；改為 .agents 時會與 Codex 共用技能目錄，必須一起修正生成、過期判定與取消清理，避免互相覆寫。

## What Changes

- 從 Fork 86dd8c9a 出發，只整合已納入的上游 0.11.0（64623a6b）到 v0.12.0（26de3b9f）的新增差異；實作第一步建立並切換 feat/merge-upstream-0-12-0，保留既有未提交文件，不在 main 改程式。
- plan 增列未轉出工作，討論詳情、CLI、Server 與桌面共用分刀解析；新增依波次呈現的路線圖與複製轉出指令入口。
- 桌面支援開啟專案、變更、討論與路線圖的 speclink:// 連結，以及 Windows／Linux 再次啟動時喚起原視窗；沿用專案切換、遠端確認與系統匣流程。
- 納入 Herdr 0.1.13 以上的面板、技能選單、Agent 追蹤、側欄進度、系統語言及 Claude Code 即時回報，包含相關文件、截圖與測試，提供安裝來源但不自動修改外部應用程式。
- 修正 issue #15：Antigravity 的顯示與生成改用 .agents；Codex 與 Antigravity 共用同一份可由兩種工具執行的技能內容，單一目標寫入和探測，按實際工具記錄執行身分，取消一種不誤刪另一種技能；舊 .agent 僅清理受管內容。
- 保留 spec_dir: openspec、自訂路徑、安全審查與標準資料、品質與安全紀錄、Antigravity 功能、系統／Server 語言、提案條列、桌面外觀與專案客製、Windows 免安裝版及 Fork 更新來源與簽章。issue #15 的錯誤路徑是明確修正例外。
- 核對並納入上游測試穩定性修補，保留目前 CI 與 Node 型別測試修正；依整合後來源更新技能、快照及手冊，不複製上游整棵規格與歷史。

## Capabilities

### New Capabilities

- `desktop-roadmap`: 看板內依波次、依賴與未轉出工作呈現的唯讀路線圖；現有 desktop-app 規定看板與詳情頁、change-plan 規定排程資料，皆未規定路線圖視圖。
- `herdr-integration`: Herdr 面板、技能送達、Agent 追蹤與側欄整合；現有 claude-code-workflow-panel 只涵蓋 Claude Code 內部面板，並未涵蓋 Herdr 的工作區與版面。

### Modified Capabilities

- `change-plan`: plan 的未轉出工作欄位、人眼輸出與共用分刀規則。
- `client-protocol`: 排程、清單與討論詳情的新增欄位及舊 Server 相容。
- `server-verb-api`: plan 與討論詳情回應同源、唯讀與既有授權語意。
- `desktop-app`: 網址導航、再次啟動喚起與討論結論改用引擎分刀。
- `workspace-tools`: Antigravity 正確目錄、與 Codex 共用生成和清理及舊佈局升級。
- `node-sdk`: Codex／Antigravity 共用技能渲染與 CLI 同源，不新增公開選項。
- `claude-code-session-focus`: Herdr 即時回報與不把背景提及誤當正在處理的變更。

## Impact

- 相關規格掃描：change-plan、desktop-app、workspace-tools 的 Purpose 與既有條文支持沿用現有能力；擴大核對 client-protocol、server-verb-api、node-sdk、claude-code-session-focus 的受影響契約。上列九項能力各有 delta 規格。
- 受影響 crate／app：speclink-core、speclink-protocol、speclink-remote、speclink-server、speclink-cli、speclink-node、speclink-desktop-core、apps/desktop、packages/ui；speclink-host、speclink-fs 既有安全、品質、路徑與交易契約維持原狀。
- 新增：`crates/engine/speclink-core/src/lifecycle/discuss/cuts.rs`、`apps/desktop/src/deepLink.ts`、`packages/ui/src/roadmapModel.ts`、`packages/ui/src/components/RoadmapView.tsx`、`integrations/herdr/`、`scripts/herdr/`、`docs/development/upstream-0.12.0-inventory.md`、對應測試與手冊頁。
- 修改：`crates/engine/speclink-core/src/lifecycle/plan.rs`、`crates/engine/speclink-core/src/lifecycle/discuss.rs`、`crates/engine/speclink-core/src/workspace/skills.rs`、`crates/engine/speclink-core/src/workspace/init.rs`、`crates/engine/speclink-core/src/workspace/config.rs`、`crates/protocol/speclink-protocol/src/query.rs`、`crates/protocol/speclink-remote/src/convert.rs`、`crates/host/speclink-server/src/api/routes.rs`、`crates/adapters/speclink-cli/src/verbs/plan.rs`、`crates/adapters/speclink-node/src/render.rs`、`apps/desktop/core/src/discussions.rs`、`apps/desktop/core/src/query.rs`、`apps/desktop/src-tauri/src/lib.rs`、`apps/desktop/src-tauri/src/remote.rs`、`apps/desktop/src-tauri/Cargo.toml`、`apps/desktop/src-tauri/tauri.conf.json`、`apps/desktop/src-tauri/capabilities/default.json`、`apps/desktop/src/App.tsx`、`apps/desktop/src/store.ts`、`apps/desktop/src/builtinTools.ts`、`packages/ui/src/adapter.ts`、`packages/ui/src/discussionModel.ts`、相關元件、adapter、語言字典、Cargo.lock、package-lock.json、README 與 docs 文件。
- Claude Code：修改 `integrations/claude-code/speclink-skills/hooks/focus.ts`、`integrations/claude-code/speclink-skills/hooks/register.tsx` 與測試；沿用上游套件相依，不直接沿用發版號提交。
- CLI 與相容性影響：不新增 CLI 動詞或旗標。plan 維持 --json、--no-color、--strict-overlap 與無 stdin 的唯讀查詢；成功 exit code 0、失敗非 0、錯誤走 stderr。JSON 刻意新增 unpromotedCuts，人眼只在有未轉出工作時多一段；discuss show 的人眼與 JSON 不新增 cuts。既有 JSON 使用者須容許新欄位，新 client 讀舊 Server 時預設空清單。
- 工具與設定：不新增工作流設定鍵，spec_dir 與 tools 格式、預設值及原有四工具選擇維持；Antigravity 由 .agent/skills 改為 .agents/skills。Codex／Antigravity 的所有受管技能（含政策啟用時的 worktree 技能、安全審查技能）採共用渲染，屬刻意更新生成內容；Claude／Copilot 的渲染與自訂 neutral API 維持。更新資產版本鎖與對應 golden，使用者執行既有 update 或桌面同步即可升級。
- 發佈邊界：不更改產品版號、既有更新日誌歷史、Fork updater endpoint／pubkey 或發佈目的地；不 push、tag、release、安裝或啟用 Herdr。若後續建立 PR，base 固定 mosutw/speclink 的 main。
