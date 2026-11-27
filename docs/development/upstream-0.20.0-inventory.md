# 上游 0.20.0 整合清冊

## 比對基準

- Fork 從 origin/main 的 b57c99d0（0.12.2）開始。該提交包含先前整合、fork 的安全與品質功能，以及正式 macOS DMG 簽章修正。
- 上一次整合採用的是上游發行分支的 26de3b9f。正式 v0.12.0 標籤指向 fd2eab0cbf961bec16b5fd9a90188fc1b984beb9。兩者間有 90 個差異檔，產品程式的兩筆差異位於桌面測試模組，其餘產品路徑差異是測試和快照的描述子名稱。規格、手冊與歷史的差異也另行核對。上一版的功能以 fork 既有行為為準。
- 終點為上游 v0.20.0。標籤物件 dd4289caa0c8f6694ccd5d1553ba43a2eb43bbd1 指向 7f1105a523189f2a390be97d9522e896510e0880。用 git diff 26de3b9f 7f1105a5 查實際整合差異，用 git log fd2eab0c..7f1105a5 看正式標籤之間的 160 個提交與 1,413 個差異檔。更新日誌查閱 CHANGELOG.zh-TW.md 的 0.13.0 到 0.20.0 各節。
- 清冊中的驗收欄列的是目標與預定測試。每個產品批次需在勾選前補上實際測試結果。

## 版本與驗收

| 版本 | 上游差異與來源 | 處理 | 任務與驗收 |
| --- | --- | --- | --- |
| 0.13.0 | Host Kit、createHost、HTTP 與動詞入口。來源 host-kit-agent-door、host-kit-client-door、hook-read-function。 | 納入 | 2.1–2.3。訪問身分、scope、唯讀 hook、拒絕時不提交及提交後事件。 |
| 0.13.0 | Store Port 與 JS 合格測試、Rust SQLite／FS／PostgreSQL 後端。來源 store-port-contract、engine-on-store-port、store-port-examples。 | 納入 | 2.1–2.2。所有後端跑同一組版本、範圍隔離、原子寫入與事件測試。 |
| 0.13.0 | 參考 Server 換用 Store Port，舊 store 退役。來源 server-routes-on-store-port、server-on-port-examples、retire-teamstore-and-drivers。 | 相容處理 | 3.1–3.2。舊版備份、新空庫還原、資料與權限對照及回復。保留舊 SDK 依賴的舊儲存契約，保護 fork 新增的歷史與安全紀錄。 |
| 0.13.0 | 舊 createEngine 自訂 store 退役，SDK 與 CLI 共用解析及輸出，claim 的新回傳形狀和宿主唯讀限制。來源 node-retire-legacy-store、shared-verb-output、shared-verb-parsing、verb-output-for-embedded、claim-release-and-output。 | 相容處理 | 2.1–2.3、4.2。既有 store 與呼叫端形狀做回歸，另驗證新宿主入口與旗標拒絕。Windows 的用法指令以 speclink 顯示。 |
| 0.13.0 | Host 匯出匯入、空範圍保護、本機匯出、桌面遷移遇符號連結拒絕。來源 import-export-on-port、engine-export-scope、show-detail-and-export-check。 | 納入 | 2.3、3.2。格式 2、排序摘要、大小限制、舊資料夾、本機缺規格與符號連結對照。 |
| 0.13.0 | 瀏覽器登入優先順序、金鑰圈及登出，系統 CA 信任、SSE 路徑與心跳。來源 client-browser-login、client-os-trust-store、event-payloads-and-push-url。 | 納入 | 3.1、3.3。握手能力切換、舊 Server、錯誤憑證、SSE 只接受同 origin，參考 Server 拒絕 45 秒以上心跳。 |
| 0.13.0 | 自由欄位、認領／釋放與顯示、變更落地及章帶 commit。來源 host-free-fields、station-status-and-landing、claim-release-and-output。 | 納入 | 4.1–4.2。對照 JSON 形狀、禁止搶占、空值和拒絕時零寫入、章與 landing 各自追蹤。 |
| 0.13.0 | 封存證據及產出物、遠端 trace／plan --strict-overlap／rank、語言整份寫入及討論還原。來源 remote-read-verbs、language-replace-and-discuss-restore。 | 納入 | 3.3、4.2。遠端與本機一致，封存後唯讀，同名還原拒絕。 |
| 0.13.0 | 章的檔案指紋、遠端品質狀態、封存帶未結工單的標示、詳細清單。來源 station-status-and-landing、show-detail-and-export-check。 | 納入 | 4.2、5.2。重開工單蓋過舊章，查詢留存原始品質與安全資料。 |
| 0.13.0 | Pi／Cursor、模型按站與工具、遠端技能隨 Server 設定、技能用動詞而非直接寫檔。來源 add-pi-cursor-tools、station-subagent-model、desktop-subagent-model-page、remote-skill-delivery、skills-verbs-first-wave、skills-verbs-second-wave。 | 相容處理 | 4.3、5.2。驗證六種工具、遠端技能版本、無 checkout 的替代流程與既有安全、共用技能。 |
| 0.13.0 | 共用流程設定讀寫、變動鍵事件、無差異不重寫、宿主唯讀限制。來源 workflow-config-commands、event-payloads-and-push-url。 | 納入 | 2.3、4.3。對照 dry-run、寫入前驗證、語言欄位、無改動沒有事件或技能重生。 |
| 0.13.0 | Herdr 我的技能、Codex／Pi、已有 agent 的 worktree、未記錄討論、按鈕／歸屬、英文與 0.1.14–0.1.16 支援。來源 6c910876、75e8b099、c6265693、ec272582 及 integrations/herdr 差異。 | 納入 | 5.1、5.3。用面板模型測識別、分頁、標籤、語系、停靠與舊版環境。 |
| 0.13.0 | 路線圖、桌面網址、Herdr 基本面板和 0.1.13 支援出現在前次上游基準前。上游 v0.12.0 更新日誌和 fork 前次封存供對照。 | Fork 已有 | 1.2、5.3。保留路線圖、討論分刀、專案網址、側邊欄與 Agent 追蹤。 |
| 0.13.0 | 文件將遠端正式用途定位為內嵌宿主，參考 Server 定位為示範。來源 docs-embedded-host。 | 相容處理 | 6.1。說明 fork 支持的使用方式、授權和舊版升級流程。 |
| 0.13.1 | SDK TypeScript 宣告的註解修補與發行前 consumer 檢查。來源 sdk-types-parse-check、scripts/npm/npm-engine-consumer-check.mjs。 | 納入 | 2.3、6.1。SDK 型別、實際 binding、套件匯入及發行守門。 |
| 0.14.0 | commandNames、匯入／匯出的穩定原因、宿主相對 path、workflow-config help 與凍結的舊欄位。來源 sdk-command-names-and-transfer-reasons、verb-output-for-embedded、docs/verb-contract.zh-TW.md。 | 納入 | 2.3、4.2–4.3。SDK 測試與 CLI、Node 的空值、欄位名、path 及錯誤 golden 對照。 |
| 0.15.0 | 多 repo 綁定、登入狀態用詞、權限收回、服務 5xx 原因、非離線 4xx 復原。來源 project-whoami-and-denied-state。 | 納入 | 3.3、5.2。缺 --repo 時顯示選項，失聯、認證失效、403 與 5xx 可區分，權限恢復後回到可寫。 |
| 0.16.0 | importScope replace 與 dryRun、新增／變動／移除鍵，空 schema 相容與匯出錯誤指出變更及位置。來源 import-replace-mode、export-failure-reason-and-location。 | 納入 | 2.3、3.2、4.3。預覽零寫入、revision 拒絕保持舊資料、schema 空值讀得出且檔案不改。 |
| 0.17.0 | 已封存 CLI、清單詳情、工單全文、討論 promotedTo。來源 archived-list-and-detail-reads。 | 納入 | 3.3、4.2。三入口同語意，舊 Server 缺欄可讀。 |
| 0.17.0 | 渲染的 skills／checkout、拒絕未知選項、工具說明過濾。來源 render-host-options-and-tool-verbs。 | 納入 | 2.3、4.3。對照 SDK 型別與宿主渲染快照。 |
| 0.17.0 | 內嵌直接讀取路由進 beforeCommand，命令白名單增加四項，保留欄位前綴，多資源事件及 eventNames／eventResource。來源 door-reads-ask-before-command、reserved-field-prefixes、cross-resource-events-and-event-names。 | 納入 | 2.1–2.3。讀取拒絕回 409；保留欄位拒絕早於 hook。跨變更、討論的事件一起提交。 |
| 0.17.0 後 | 開發機安裝使用 Developer ID、Team ID 驗證。來源 local-install-developer-id-signing。 | 相容處理 | 6.1。套用本機安裝行為與測試，維持 0.12.2 正式 DMG 簽章與來源。 |
| 0.18.0 | 遠端專案不必打開即可檢查連線；路徑與 remote 並存時保留桌面的「繼續本機」選擇。來源 rail-remote-connection-check、continue-local-tab-reactivation。 | 納入 | 5.1–5.2。啟動、聚焦、週期檢查及重啟、背景、本機根驗證。 |
| 0.18.0 | SDK 渲染 gitHistory、instructions specs 回規格語言、TDD／audit 指引不叫宿主不支援的指令。來源 render-git-history-option、instructions-spec-language-and-tasks-pointer。 | 相容處理 | 2.3、4.3。驗證 tw／ja／en、auto、fork 的舊預設及只讀 checkout。 |
| 0.18.0 | Herdr 點變更或討論路徑開桌面詳情、worktree 對回主專案、已封存討論不誤列到未記錄。來源 7809f277、c235e1b7。 | 納入 | 5.1、5.3。以主／工作樹路徑、封存 fixture 及 Herdr 版本對照。 |
| 0.19.0 | Herdr 0.1.18 視窗不可見停止拉取、顯示後補一次、指令面板入口。來源 9125a428。 | 納入 | 5.1、5.3。驗證無查詢及補抓次數，舊版仍可使用既有功能。 |
| 0.19.0 | 指向正式規格用原需求／情境名稱、ingest 列改名對照、不明 artifact ID 提示可用值。來源 spec-names-and-instructions-ids。 | 納入 | 4.1、4.3。對照語言、舊名稱及錯誤提示。 |
| 0.20.0 | whyExcerpt、stage、createdById、readyAt、討論 modifiedAt 及 worktree show，搭配桌面的相對時間與人物篩選。來源 change-and-discussion-list-fields、desktop-uses-list-fields。 | 納入 | 4.1–4.2、5.1–5.2。驗證最後勾選、取消、午夜、時區、同名／改名及舊欄位缺失。 |
| 0.20.0 | CLI 在 Herdr 回報標籤、可 opt-out，Herdr 等候原因、hc run 新分頁及 herdr-speclink 技能。來源 cli-terminal-activity-report、1284654f、116419b0、c5bc035b。 | 納入 | 5.1、5.3。有無 Herdr、失敗、opt-out 及所屬 space 對照，輸出與退出碼維持。 |
| 0.20.0 | 任務短標題與中／日／英驗證標記。來源 task-title-convention。 | 納入 | 4.3、5.2。產出指示有短標題及可驗證行為，舊格式仍能展開。 |
| 跨版本 | 上游自身 speclink/ 佈局、上游其餘歷史和發行版號、產出物與截圖目錄的重排。來源 .speclink.yaml、Cargo.toml、speclink/changes/archive、spec-dir 相關封存。 | 上游自用 | 1.2、6.1。沿用 fork 的 openspec、獨立更新來源和發行設定；上游佈局的應用程式相容由專案測試查核。 |
| 跨版本 | docs 圖表、連結、規格語言用詞、Cargo 清理與 CI 調整。來源 scripts/docs、scripts/dev、.github/workflows 差異。 | 相容處理 | 6.1。帶入產品功能所需的文件與檢查，保留 fork 的發行與 CI 邊界。 |

## Fork 0.12.2 回歸基準

工作樹從包含 b57c99d0 的提交開始。CLI 使用此工作樹的來源重新執行測試，Cargo 快取放在主 repo 的既有 target，沒有使用已安裝的 CLI 代替測試。開啟本機測試伺服器後，整組 CLI 整合測試 621 項通過。Node 腳本測試 545 項通過、4 項跳過、0 項失敗。

| Fork 行為 | 來源與已通過的測試 | 後續比對值 |
| --- | --- | --- |
| 安全審查、接受與歷史 | CLI 的 security、archive_readiness_gate、workflow_config；正式規格 security-review、security-review-report、security-standard-data | 本機資料來源、遮罩及安全驗收繼續受既有政策控制；遠端政策保留 CAS，封存保留啟用的檢查紀錄。 |
| 品質工單與封存 | CLI 的 review_verbs、verify_verbs、archive_readiness_gate；正式規格 review-station、verify-station、change-lifecycle | 判定結果、紀錄與封存要求維持既有語意；新 commit 欄位與遠端顯示可有增量。 |
| 自訂路徑、語言、四種工具 | CLI 的 spec_dir_layout、spec_dir_migration、workflow_config、init_tools、copilot_tools、tools_descriptor、remote_section；正式規格 workspace-tools、workflow-config | 既有 spec_dir: openspec、自訂根、locale／spec_locale、.agents 共用目標與工具路徑、未識別自訂設定維持；新工具只新增其目標。 |
| Windows 免安裝版、DMG 與更新來源 | scripts/desktop/windows-portable.test.mjs、scripts/release/macos-artifact-verify.test.mjs、scripts/release/desktop-bundle-config.test.mjs、scripts/release/signing-gate.test.mjs；apps/desktop/src-tauri/tauri.conf.json | 保留免安裝啟動與隔離資料、DMG 的 app／CLI 簽章驗證、既有公開金鑰和 mosutw/speclink 更新位址。 |

## 目前驗證結果

- CLI 的查詢、文件、指示、建立、排程、任務與自由欄位使用共用動詞。整合自由欄位後 Rust 各套件連同測試可編譯，共用動詞 146 項單元測試及 4 項檢查通過。
- Store Port 的記憶體 42 項、FS 36 項、SQLite 34 項、OrbStack 上的 PostgreSQL 38 項通過。FS 和 SQLite 各跳過專用程序測試 3 項及 1 項。四套 Rust 後端驗證只讀版本條件衝突時，文件、事件及 scope 版本都不變。
- Host 240 項單元測試通過，含舊安全政策、只讀版本條件、原子提交與匯入匯出。SDK 測試涵蓋舊入口、JS 合格測試、掛鉤、保留欄位、匯入預覽、跨資源事件和技能裁剪。HTTP 涵蓋綁定、輪詢、列清單、規格、搜尋、快照、看板讀取、建立紀錄、欄位編輯和範圍移轉。匯入取代沿用上游逐文件版本條件。
- 桌面遷移共 12 項測試通過。進行中與封存的審查資料可隨範圍搬移，並保留舊中繼資料檔名的錯誤訊息。
- Server 換用 Store Port 後，舊 SQLite、ServerFS 和 PostgreSQL 備份均可還原到新版空庫，帳號、角色、repo、品質／安全紀錄和驗收證據都可讀回；損毀備份、非空目標及舊版資料目錄會被拒絕且不覆寫來源。備份升級方式已補入中英文手冊。
- Server 測試通過 35 項單元及 369 項整合測試（另 1 項既有忽略），網頁 244 項通過；CLI 623 項與 Desktop Core 376 項通過。非互動登入會先提示 `--token-stdin`，不依賴連線才能顯示正確用法。
- Desktop 依 `/binding` 握手宣告的 SSE 網址連線，只向同來源送憑證；未提供有效 SSE 宣告時退回輪詢。SSE 4 項和 Desktop 握手網址分頁隔離測試通過。
- 生命週期、工作流與語言（4.1–4.3）：在主 checkout 執行 `show --json` 可讀取活動 worktree；`change landed`、`evidence show`（含封存後讀取）、`discuss restore`（同名拒絕）、`claim --release` 均已接通並通過 CLI 回歸與 `speclink-verbs`（146 項單元測試 + 4 項檢查）；`Pi`（`.pi/skills`）與 `Cursor`（`.cursor/skills`）納入內建工具、`instructions specs` 回報有效 `spec_locale`、不明 artifact ID 列出可用值、遠端 `update` 離線不覆寫既有技能、`workflow-config subagent-model` 支援無變更寫入不改寫內容與 mtime。`speclink-core` 專案測試 303 項、`command::` 113 項與 `upstream_020_workflow_gap` 2 項、`speclink-protocol` 88 項全數通過。
- 桌面、Herdr 與發行收尾（5.1–6.1）：修正 `packages/ui/src/time.ts` 為本地日曆日計算以通過午夜跨日測試，`apps/desktop/src/builtinTools.ts` 加入 `pi` 與 `cursor`；`@speclink/ui` 全套 Vitest 與 `@speclink/desktop` 56 檔 1,474 項測試、`speclink-desktop-core` 371 項通過。CLI 加入 Herdr 終端活動最佳努力回報與 `SPECLINK_TERMINAL_REPORT=false` opt-out，stdout 與退出碼不受回報影響。`scripts/desktop` 與 `scripts/release` 181 項腳本測試通過（確認 macOS 本機安裝 Developer ID 優先／ad-hoc 回退／驗不過不安裝及正式發行簽章保留）。
- Audit 檢查：新參數（`change landed --commit`、`workflow-config subagent-model`、`claim --release`、`discuss restore`、`evidence show`）與環境變數（`SPECLINK_TERMINAL_REPORT`）皆已驗證空值／無效值 fail-closed、無變更不重寫設定、且終端機回報失敗不影響主指令退出碼。

## Server 安全驗收的儲存差異

Fork 的 TeamStore 在遠端驗證與封存時會檢查政策、驗收索引及證據文件的版本，這些文件中有些只讀取。檢查與封存或蓋章在同一筆交易內，來源變更時整筆拒絕。上游 Store Port 的 Commit 只有 Put 與 Delete 的版本條件。直接移植會缺少只讀文件的版本保證，額外寫回同內容會更新這些文件的版本和修改時間。

採用內建 Rust 儲存的唯讀版本條件。檢查在提交鎖或資料庫交易內執行，支援記憶體、檔案、SQLite 和 PostgreSQL。SDK 的三個 JS 儲存方法與呼叫形狀維持相容。
