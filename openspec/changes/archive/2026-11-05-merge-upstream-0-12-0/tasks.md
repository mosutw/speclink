## 1. 新分支與來源盤點

- [x] 1.1 開發第一步完成 D1 新分支：檢查 Git 狀態、保存既有未提交文件，從含 Fork 86dd8c9a 的基底建立並切換 feat/merge-upstream-0-12-0；同名已存在則先核對、不重設。範圍：整個 repo 的 Git 分支、本提案與 openspec/discussions/merge-upstream-0-12-0.md。驗證：git branch --show-current、git status -sb 與 HEAD／祖先核對，在任何應用程式碼變動前記錄結果。 <!-- speclink-task:tsk_01M6NWPB1GS7WRJ8D944M9JEC1 -->
- [x] 1.2 依 D1 將上游 64623a6b 至 26de3b9f 的全部差異分類為採入、已存在、保留 Fork 或略過並附理由，列出安全、品質歷史、Proxy、外觀、工具、路徑與發佈保留矩陣；範圍 docs/development/upstream-0.12.0-inventory.md、docs/development/upstream-0.11.0-inventory.md。驗證：逐項對照 git diff 的路徑清單，每項有處置且沒有引用標籤以後的來源。 <!-- speclink-task:tsk_01M6NWPB1GZQCF6B4PAP5VY1Q3 -->

## 2. 分刀與排程

- [x] 2.1 先為 D2「plan 動詞輸出波次與阻擋清單」「plan 列出討論結論中尚未轉出的刀」建立失敗案例：單刀多變更、部分轉出、無結論、註解／程式區塊、非短橫線列點、讀不到討論及原波次不變。範圍 crates/engine/speclink-core/src/lifecycle/discuss/、crates/engine/speclink-core/src/lifecycle/plan.rs。驗證 cargo test -p speclink-core，確認缺解析或欄位的斷言先失敗。 <!-- speclink-task:tsk_01M6NWPB1GRNZQZ43XKB8Q7G4M -->
- [x] 2.2 納入共用 cuts 解析與 plan 未轉出工作，保持既有 Store 邊界與波次／封存順序，所有入口共用一份計算，無程式碼使用新儲存抽象；範圍 crates/engine/speclink-core/src/lifecycle/discuss.rs、crates/engine/speclink-core/src/lifecycle/discuss/cuts.rs、crates/engine/speclink-core/src/lifecycle/plan.rs、crates/engine/speclink-core/src/command/mod.rs。驗證前項 core 測試轉綠，unpromotedCuts 五欄 camelCase 的型別及順序符合規格，重構後同組測試仍通過。 <!-- speclink-task:tsk_01M6NWPB1G23161FVA34SH8FMM -->

## 3. 協定與遠端轉換

- [x] 3.1 先補「plan 回應 payload」「討論詳情回應的分刀 payload」「討論搜尋回應 payload」的欄位、預設值與回傳測試，含舊 Server 缺欄位及無效操作轉換；範圍 crates/protocol/speclink-protocol/src/query.rs、crates/protocol/speclink-remote/tests/it/typed_client.rs。驗證 cargo test -p speclink-protocol 與 cargo test -p speclink-remote，新增欄位斷言先失敗。 <!-- speclink-task:tsk_01M6NWPB1G139PCYF20VY1594X -->
- [x] 3.2 依 D2 納入型別、JSON Schema 與 typed client 逐欄轉換，保留 Fork 既有安全與品質契約；範圍 crates/protocol/speclink-protocol/src/query.rs、crates/protocol/speclink-remote/src/convert.rs。驗證兩個 package 測試轉綠，斷言 cuts 每個鍵的型別、changesInBold、unpromotedCuts 空陣列及不遺失原欄位。 <!-- speclink-task:tsk_01M6NWPB1GV316DXGTWGSC5JCD -->

## 4. CLI 與 Server

- [x] 4.1 先補「plan 唯讀衍生查詢端點」「討論詳情回應攜帶分刀」與 CLI plan 的 local／remote 對照：reader 可讀、成環 409、無名稱 404、認證失效、未轉出工作的 no-color 與 JSON、discuss show 原鍵集合；範圍 crates/adapters/speclink-cli/tests/it/plan_verbs.rs、crates/adapters/speclink-cli/tests/it/remote_plan.rs、crates/adapters/speclink-cli/tests/it/remote_read_path.rs、crates/host/speclink-server/tests/it/api/plan_api.rs、crates/host/speclink-server/tests/it/api/discussion_routes.rs。驗證 cargo test -p speclink-cli 與 cargo test -p speclink-server，先證明新增契約斷言失敗。 <!-- speclink-task:tsk_01M6NWPB1G7B2WAZ7W87VJG65W -->
- [x] 4.2 接上 Server 查詢與 CLI 呈現並維持 D2 同源：plan --json 增 unpromotedCuts、無新 stdin／旗標、人眼無資料時不變、--no-color 無 ANSI，discuss show 不暴露 cuts，成功 0／失敗非零與 stderr 語意維持；範圍 crates/host/speclink-server/src/api/routes.rs、crates/adapters/speclink-cli/src/verbs/plan.rs、crates/adapters/speclink-cli/src/verbs/、前項測試。驗證兩個 package 測試轉綠，遠端 strict-overlap 仍在零請求下拒絕，原有 parity 與認證測試通過。 <!-- speclink-task:tsk_01M6NWPB1GKY1X2SVRP8XGQ9BY -->

## 5. 桌面資料來源與討論結論

- [x] 5.1 先為「變更清單的排程欄位」「remote 變更清單的排程欄位」「討論結論以欄位標籤呈現」補本地與遠端 fixture：同次 next／unpromotedCuts、成環、舊 Server 無 cuts 與 Fork 安全／品質欄位保留；範圍 apps/desktop/core/src/query.rs、apps/desktop/core/src/discussions.rs、apps/desktop/src-tauri/tests/it/remote_data.rs、apps/desktop/src/__tests__/remoteDataSource.test.ts、packages/ui/src/__tests__/conclusionDoc.test.tsx、packages/ui/src/__tests__/discussionModel.test.ts。驗證 cargo test -p speclink-desktop-core、npm test -w apps/desktop、npm test -w packages/ui，新增契約先失敗。 <!-- speclink-task:tsk_01M6NWPB1GZ6SJWSW56FMA684S -->
- [x] 5.2 接好 D2 的本地／遠端桌面清單及討論查詢、adapter 與 UI，清單欄位名為 next 而非另造 planNext；前端改讀引擎 cuts、保留非分刀細節與已封存畫面；範圍 apps/desktop/core/src/query.rs、apps/desktop/core/src/discussions.rs、apps/desktop/core/src/manage.rs、apps/desktop/src-tauri/src/remote.rs、apps/desktop/src/adapter/、packages/ui/src/adapter.ts、packages/ui/src/discussionModel.ts、packages/ui/src/components/ConclusionDoc.tsx、packages/ui/src/components/DiscussionDetailPage.tsx、packages/ui/src/components/ArchivedDetailPage.tsx。驗證前項測試轉綠，主查詢失敗仍報錯，plan 不可用時依規格退回 board 而不新增寫入。 <!-- speclink-task:tsk_01M6NWPB1GNGHKAVZGKHCRZCB5 -->

## 6. 路線圖

- [x] 6.1 先為 D3「看板頁的路線圖檢視」「路線圖依波次排列變更」「路線圖畫出依賴線並亮起路線」「路線圖列出尚未轉出的刀」「路線圖的成環與空狀態」加入資料模型與元件失敗測試，涵蓋空資料、壞 metadata、已封存前置、複製但不執行及返回保留狀態；範圍 packages/ui/src/__tests__/roadmapModel.test.ts、packages/ui/src/__tests__/roadmapView.test.tsx、apps/desktop/src/__tests__/store.test.ts、apps/desktop/src/__tests__/App.test.tsx。驗證 npm test -w packages/ui 與 npm test -w apps/desktop，尚無新視圖時的契約先失敗。 <!-- speclink-task:tsk_01M6NWPB1GFEZHKDSDC61KP69J -->
- [x] 6.2 整合路線圖模型、波次／依賴呈現、切換入口及詳情宿主，沿用共用 Button／Badge／Tooltip／ScrollArea 等原語；範圍 packages/ui/src/roadmapModel.ts、packages/ui/src/components/RoadmapView.tsx、packages/ui/src/components/KanbanBoard.tsx、packages/ui/src/components/BoardSearchBar.tsx、packages/ui/src/index.ts、apps/desktop/src/App.tsx、apps/desktop/src/store.ts、相關 i18n。驗證前項測試轉綠；看板保持掛載但隱藏時不可互動，往返保留搜尋／捲動／篩選、不多查一次資料、不改專案欄高亮。 <!-- speclink-task:tsk_01M6NWPB1GRMT6V86BWNDY13ZP -->

## 7. 網址與視窗

- [x] 7.1 先為 D3「以 speclink 網址開啟詳情頁」補四種網址、編碼與控制字元、重複 project、Windows 路徑、未知種類、找不到目標、啟動佇列、切專案競爭、不可取消浮層與 remote 選擇器的失敗案例；範圍 apps/desktop/src/__tests__/deepLink.test.ts、apps/desktop/src/__tests__/store.test.ts、apps/desktop/src/__tests__/workspace.test.ts。驗證 npm test -w apps/desktop，新增解析與派送斷言先失敗。 <!-- speclink-task:tsk_01M6NWPB1GW68AWZMKZR3DDE1V -->
- [x] 7.2 納入唯一 deepLink 入口與 store／App／tray 導航，依序處理啟動網址、不越過遠端確認且不呼叫寫入動詞；範圍 apps/desktop/src/deepLink.ts、apps/desktop/src/store.ts、apps/desktop/src/App.tsx、apps/desktop/src/tray.ts、apps/desktop/src/components/WorkspaceChooser.tsx、apps/desktop/src/components/EmptyWorkspace.tsx、apps/desktop/src/components/RecentList.tsx、apps/desktop/src/i18n/messages.ts。驗證前項測試轉綠，project 一律回卡片、roadmap 到路線圖、缺目標訊息正確。 <!-- speclink-task:tsk_01M6NWPB1GK3723F8Y303W9CZK -->
- [x] 7.3 接上 Tauri deep-link 與 single-instance 外掛、四種類網址註冊及 Windows／Linux 喚起既有視窗，保留 Fork portable、Proxy、安全命令與 updater 設定；範圍 apps/desktop/src-tauri/src/lib.rs、apps/desktop/src-tauri/Cargo.toml、apps/desktop/src-tauri/tauri.conf.json、apps/desktop/src-tauri/capabilities/default.json、apps/desktop/package.json、Cargo.lock、package-lock.json。驗證 cargo check -p speclink-desktop、npm run build -w apps/desktop、腳本整組測試；比對 updater endpoint／pubkey 與產品版號不變，三平台 OS 註冊檢查由既有 CI 承接。 <!-- speclink-task:tsk_01M6NWPB1G5WBNEJDZ7DZSGAP4 -->

## 8. Herdr

- [x] 8.1 先納入 D4 模型失敗測試，涵蓋「技能推薦與送達」「Agent 追蹤與側欄呈現」「開工與技能選單」：worktree 優先、兩站章、待重新反映、等待回答、閒置不阻擋、討論歸屬、一次一個 archive+commit、不繞過安全守門；範圍 scripts/herdr/panel-model.test.mjs。驗證 node --test scripts/*.test.mjs scripts/*/*.test.mjs，現有缺少模型的環境先失敗且 fixture 不接外部服務。 <!-- speclink-task:tsk_01M6NWPB1G30FYAA3BA7ND08P5 -->
- [x] 8.2 納入目標版本 Herdr model.js／shared.js 與 Agent 標籤 helper，對應前項規則並使用既有 CLI 資料；範圍 integrations/herdr/mods/speclink/model.js、integrations/herdr/mods/speclink/shared.js、integrations/herdr/mods/speclink/label-skill.mjs、integrations/herdr/agents-list/speclink-rows.js、integrations/herdr/spaces-badge/stage-badge.js。驗證腳本整組測試轉綠，不把一般提及綁成工作者，不自動略過封存失敗。 <!-- speclink-task:tsk_01M6NWPB1G13DETQTM5NJQAZDX -->
- [x] 8.3 先為「面板跟隨工作區與資料狀態」「桌面導航與初始化提示」「系統語言與安裝邊界」補純模型／安裝 fixture 斷言：space 各自開關、四種連結、未初始化與壞設定區分、中文弱偵測與不寫使用者設定；範圍 scripts/herdr/panel-model.test.mjs、scripts/herdr/dock-panel.mjs 的測試 fixture。驗證腳本整組測試先失敗，無 Herdr 安裝也能執行。 <!-- speclink-task:tsk_01M6NWPB1GZZCARDA4S1745V63 -->
- [x] 8.4 納入面板、選單、停靠、快捷鍵、圖示、系統語言與安裝腳本來源，讓停靠／浮動模式依規格運作；範圍 integrations/herdr/mods/speclink/main.js、integrations/herdr/mods/speclink/skill-picker.js、integrations/herdr/mods/speclink/column-sync.js、integrations/herdr/mods/speclink/i18n.js、integrations/herdr/mods/speclink/icons/、integrations/herdr/mods/speclink/mod.json、integrations/herdr/mods/speclink-new/、integrations/herdr/keys.json、integrations/herdr/package.json、scripts/herdr/dock-panel.mjs。驗證腳本整組測試轉綠、檢查打包引用完整；只寫 repo 來源，沒有執行外部安裝。 <!-- speclink-task:tsk_01M6NWPB1G5AJ35AJW76MA22DJ -->

## 9. Claude Code 焦點

- [x] 9.1 先為 D4「從使用者指令與工具呼叫辨識焦點」「Herdr 內即時回報已確認的變更」加入非變更技能清舊焦點、propose 新建回填、主／子代理隔離及缺 Herdr 不出錯的失敗測試；範圍 integrations/claude-code/speclink-skills/tests/register.test.tsx。驗證 claude plugin test integrations/claude-code/speclink-skills，確認新增行為先失敗；若本機無支援版本，記錄缺少的工具與未執行測試，不宣稱通過。 <!-- speclink-task:tsk_01M6NWPB1GA5MEG1EZG706T2F5 -->
- [x] 9.2 納入 withVerb／commandChange 與 Herdr 即時標籤回報，保留技能列、標題、clear 與 Windows CLI 呼叫；範圍 integrations/claude-code/speclink-skills/hooks/focus.ts、integrations/claude-code/speclink-skills/hooks/register.tsx、integrations/claude-code/speclink-skills/.claude-plugin/plugin.json。驗證前項測試轉綠及 claude plugin validate integrations/claude-code/speclink-skills；可用時執行 npx tsc -p integrations/claude-code/speclink-skills，記錄型別依賴限制，不更改使用者全域設定。 <!-- speclink-task:tsk_01M6NWPB1GC7DQE5299FGX49AX -->

## 10. 共用技能渲染

- [x] 10.1 先為 D5「Codex 與 Antigravity 共用技能內容」「Copilot 渲染目標」建立各技能兩 target 同內容、宿主入口前言、當前 --agent 參數、無生成模板及政策集合的失敗測試；範圍 crates/engine/speclink-core/src/workspace/skills.rs、crates/engine/speclink-core/tests/it/render_golden.rs。驗證 cargo test -p speclink-core，Codex／Antigravity 內容不同時先紅，Claude／Copilot／neutral 舊快照仍作保護。 <!-- speclink-task:tsk_01M6NWPB1GW9JEBAGC811AG9YG -->
- [x] 10.2 實作共用渲染分支與宿主前言，讓技能引用、互動與執行身分有明確選擇規則，不殘留模板、不改中性 API；範圍 crates/engine/speclink-core/src/workspace/skills.rs、crates/engine/speclink-core/src/workspace/init.rs、crates/engine/speclink-core/tests/golden/、crates/engine/speclink-core/tests/golden/assets.lock。驗證前項測試轉綠；提升 ASSET_VERSION 而非產品版號，依既有生成方式更新鎖與快照，只接受已明記的共用內容及版本戳差異。 <!-- speclink-task:tsk_01M6NWPB1G3WVHPYN7SRMCG783 -->

## 11. 共用目錄同步與舊目錄升級

- [x] 11.1 先為 D6「built-in tools 權威收斂」「技能檔過期探測」「Antigravity 技能生成與獨立清理」加入三種選集、雙向切換、取消其一／全部、差異路徑去重與連續同步現版的失敗測試；範圍 crates/engine/speclink-core/src/workspace/init/tests.rs、crates/adapters/speclink-cli/tests/it/。驗證 cargo test -p speclink-core 與 cargo test -p speclink-cli，確認直接共用常數的覆寫／誤刪案例先失敗。 <!-- speclink-task:tsk_01M6NWPB1G2316YE2W772HDN5S -->
- [x] 11.2 合併 SyncPlan 的實體生成／探測目標，依目錄仍有選取工具決定清理，保持兩種邏輯工具摘要、worktree 政策、未知鍵與自訂描述子；範圍 crates/engine/speclink-core/src/workspace/init.rs、crates/engine/speclink-core/src/workspace/skills.rs、crates/engine/speclink-core/src/workspace/config.rs。驗證前項測試轉綠，differingFiles 路徑僅一份、工具方向兩份，取消其一後檔案仍在、重試不再改內容。 <!-- speclink-task:tsk_01M6NWPB1GQ4V5YN36V9FPAVH1 -->
- [x] 11.3 先為「Antigravity 舊目錄升級」建立舊 .agent、混合自有技能／規則、生成失敗、清理失敗、降級拒絕、錯誤描述子及目錄／父目錄／技能子目錄連結的失敗測試；範圍 crates/engine/speclink-core/src/workspace/init/tests.rs、crates/adapters/speclink-cli/tests/it/。驗證 core／CLI package 測試先紅，Windows reparse point 與 Unix symlink 分別有對應可執行 fixture。 <!-- speclink-task:tsk_01M6NWPB1GVP59F8EWGQB6G8TH -->
- [x] 11.4 依 D6 先驗證、再完整寫新位置、最後清理舊受管內容，保留舊路徑保留名與既有降級守門；範圍 crates/engine/speclink-core/src/workspace/init.rs、crates/engine/speclink-core/src/workspace/config.rs。驗證前項測試轉綠，錯誤非零且含失敗路徑、失敗後重跑收斂，自有檔案與連結目標位元不變，不刪整個 .agent。 <!-- speclink-task:tsk_01M6NWPB1GBEZN1PNDC4MWP3MS -->

## 12. 桌面與 SDK 的目錄修正

- [x] 12.1 先補 issue #15 UI 標籤、桌面同步與「渲染 API」對照的失敗測試：五個 target 型別不變，Codex／Antigravity 及 CLI 三種選集內容相同；範圍 apps/desktop/src/__tests__/projectSettingsView.test.tsx、apps/desktop/src/__tests__/workspaceChooser.test.tsx、apps/desktop/src/__tests__/App.test.tsx、crates/adapters/speclink-node/__test__/render.spec.ts、crates/adapters/speclink-node/__test__/types/render-api.ts。驗證 npm test -w apps/desktop；Node 先 npm --prefix crates/adapters/speclink-node run build，再 npm --prefix crates/adapters/speclink-node test，先證明舊路徑／內容斷言失敗。 <!-- speclink-task:tsk_01M6NWPB1GWSKZ7F1FPPT4FY2Q -->
- [x] 12.2 讓 builtinTools 顯示 antigravity [.agents]、SDK 兩 target 沿 D5 共用引擎渲染，保留公開 options、未知輸入錯誤與目前型別測試的逾時修補；範圍 apps/desktop/src/builtinTools.ts、crates/adapters/speclink-node/src/render.rs、crates/adapters/speclink-node/index.d.ts、相關前項測試。驗證前項桌面與 Node 測試轉綠；SDK 不寫檔，neutral、Claude、Copilot 原契約仍通過。 <!-- speclink-task:tsk_01M6NWPB1G5QXASGC5TA5ZK0M3 -->

## 13. 同類位置、文件與整合驗證

- [x] 13.1 排查同類路徑並逐項註記採修或保留原因：以 rg 搜尋 antigravity、\.agent/skills、\.agent\b、Tool::Antigravity、skills_dir，命中的 crates/engine/speclink-core/src/workspace/、crates/adapters/speclink-node/、apps/desktop/src/builtinTools.ts、apps/desktop/src/__tests__/、README.md、README.zh-TW.md、docs/、.agent/skills/ 與 .agents/skills/，同步修改現在有效的生成／顯示／測試／文件；歷史 openspec/changes/archive/、openspec/discussions/archive/ 的舊記錄不改，舊路徑只在升級／相容測試中保留。依 D7 把結果記入 docs/development/upstream-0.12.0-inventory.md；驗證重新搜尋無未解釋的現行錯誤路徑，舊歷史逐位元不變。 <!-- speclink-task:tsk_01M6NWPB1GTCDDT06XA09ZGQ9A -->
- [x] 13.2 依 D7 納入 README／中英產品、路線圖與 SDK／Herdr 文件及截圖、更新 openspec/manual/ 的 plan、desktop-roadmap、desktop-deep-link 與索引，不覆寫既有更新日誌；更新 .agents/skills/、.claude/skills/、.github/skills/ 等本 repo 所選工具的生成檔，按 D6 處理舊 .agent。範圍 README.md、README.zh-TW.md、docs/、integrations/herdr/README.md、integrations/herdr/README.en.md、integrations/claude-code/speclink-skills/README.md、integrations/claude-code/speclink-skills/README.en.md、openspec/manual/、scripts/docs/。驗證腳本整組測試與生成前後差異，四工具引用正確、規格路徑為 openspec、沒有預建發版條目或修改使用者設定。 <!-- speclink-task:tsk_01M6NWPB1GY0NJNKVMSEJBNTSW -->
- [x] 13.3 整合 D7 的測試穩定性修補與 Fork 保留矩陣回歸，逐項確認上游三處測試修補是否已由本分支解決，已解決則保留原作；範圍 scripts/npm/npm-cli-launcher.test.mjs、apps/server-web/src/__tests__/users.test.tsx、crates/host/speclink-server/tests/it/admin/web_api.rs、apps/desktop/src/__tests__/、packages/ui/src/__tests__/、crates/host/speclink-host/、crates/engine/speclink-fs/、docs/development/upstream-0.12.0-inventory.md。驗證受影響的 cargo test -p speclink-host、cargo test -p speclink-fs、core／CLI／protocol／remote／Server／desktop-core 測試、npm test -w packages/ui、npm test -w apps/desktop、npm test -w apps/server-web、npm run build -w apps/server-web、Node build 後測試及 golden／CLI parity；依 speclink instructions --skill audit 的內嵌清單檢查 URL 輸入、認證邊界、目錄清理與旗標預設，記錄未實跑環境限制，不宣稱三平台或外部 Herdr 已驗證。最後必跑 node --test scripts/*.test.mjs scripts/*/*.test.mjs；完整 npm run test:all 保持由既有 CI 守門，確認版號、更新日誌歷史、updater／簽章／portable 及發佈目標不變。 <!-- speclink-task:tsk_01M6NWPB1GNJZRYV5M1KMJ8MRZ -->
