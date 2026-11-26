## 1. 基準與分批整合

- [ ] 1.1 建立整合清冊。比較 fork 0.12.2 與上游 26de3b9f 到 7f1105a5 的變動，逐一將更新日誌、規格與使用者可見差異對到納入、既有、相容處理或上游自用；為跨切面的項目列後續任務及來源。範圍 docs/development/、openspec/。驗證：來源提交確實對應兩個 tag，清冊有每個版本的來源與驗收證據欄，隔離工作樹以含 b57c99d0 的 fork main 為基準。 <!-- speclink-task:tsk_01M8C31J3133KG6DZ5FMJTTX13 -->
- [ ] 1.2 建立 fork 回歸對照。用 fork 現有 fixture 記下安全審查與驗收、品質紀錄、工作流語言與自訂路徑、四種工具、Windows 免安裝版、DMG 簽章及更新來源。範圍 openspec/specs/、crates/adapters/speclink-cli/tests/、scripts/、.github/workflows/。驗證：記錄哪些原有測試已通過，以及整合後須保持的輸出與配置值。 <!-- speclink-task:tsk_01M8C31J31YAH5FCGHBNHRMKEG -->

## 2. 宿主、共用動詞與儲存分層

- [ ] 2.1 建立宿主與儲存回歸測試。先重現新 SDK 入口、三個操作與單次提交、範圍匯入模式、宿主保留自由欄位、內嵌宿主的授權與事件、匯出與宿主工具尚未可用；加舊 createEngine 與原回傳形狀對照。範圍 crates/host/speclink-host/、crates/store/、crates/adapters/speclink-node/。驗證：相關 cargo test 與 Node binding 測試可區分新契約未實作與舊契約已存在。 <!-- speclink-task:tsk_01M8C31J31ANZE7YDKZWB40KQF -->
- [ ] 2.2 接上共用動詞與儲存。引入 speclink-verbs 與 Store Port，讓 CLI、SDK 與 Host 用同一套動詞解析與輸出，組裝 JS／SQLite／PostgreSQL／FS，保留舊 SDK 相容入口。範圍 crates/adapters/、crates/engine/、crates/store/、crates/host/speclink-host/。驗證：cargo test -p speclink-verbs、cargo test -p speclink-host 及三個 Store Port package 的合格測試，確認版本衝突零寫入及事件共提交。 <!-- speclink-task:tsk_01M8C31J31YHTHCETF2FJ0RANP -->
- [ ] 2.3 接上公開 SDK。提供 createHost、host.dispatch、HTTP、匯入匯出、掛鉤、命令及事件資訊、技能裁剪，型別和 runtime 同步。範圍 crates/adapters/speclink-node/、crates/host/speclink-host/。驗證：npm --prefix crates/adapters/speclink-node run build 後跑 npm --prefix crates/adapters/speclink-node test，確認缺失與錯誤參數、舊 SDK 形狀、匯入差異列表及雙資源事件。 <!-- speclink-task:tsk_01M8C31J316EQAW14STZ9ZTH8R -->

## 3. Server 資料升級與復原、遠端契約

- [ ] 3.1 建立遠端與升級回歸測試。先用三個舊後端建立備份 fixture，包含角色、範圍、品質和安全歷史；補上參考 Server 儲存升級、登入發現與連線狀態、詳細查詢與舊版容錯測試。範圍 crates/protocol/、crates/host/speclink-server/、crates/adapters/speclink-cli/tests/。驗證：cargo test -p speclink-server 與 cargo test -p speclink-cli，確認可重現舊資料與缺失新欄位的情況。 <!-- speclink-task:tsk_01M8C31J31Q822DW20FMM6C5HW -->
- [ ] 3.2 接入新版 Server 儲存。固化舊版備份、新版空庫還原、損毀或非空時拒絕及舊庫復原；接通新 Host 路由及 revision 邊界。範圍 crates/host/speclink-server/、crates/store/、apps/server-web/、docs/。驗證：cargo test -p speclink-server 與 npm test -w @speclink/server-web，三後端升級後都保留原授權、規格和品質／安全資料。 <!-- speclink-task:tsk_01M8C31J31TGKV8FPKFVF858NJ -->
- [ ] 3.3 接上用戶端。完成登入順序、系統信任、金鑰圈登出、多 repo 綁定、同 origin SSE、失敗原因與遠端排程／查詢；保留既有服務端相容。範圍 crates/protocol/、crates/adapters/speclink-cli/、crates/host/speclink-server/、apps/desktop/core/。驗證：cargo test -p speclink-cli 與 cargo test -p speclink-desktop-core，確認舊服務端缺欄位仍能讀、5xx 和 4xx 有可辨狀態、認證失效和 revision 不符不被當成寫入成功。 <!-- speclink-task:tsk_01M8C31J31RWHATH7KFT4S8EEW -->

## 4. 生命週期、工作流與語言

- [ ] 4.1 建立流程回歸測試。先重現缺少就緒時間與工作樹讀取、自由欄位與追蹤操作、新增工具及遠端技能來源、品質模型與產出語言。範圍 crates/engine/speclink-core/、crates/adapters/speclink-cli/tests/。驗證：cargo test -p speclink-core 和 cargo test -p speclink-cli 可指出新增契約缺口，既有 golden 可辨識預期差異。 <!-- speclink-task:tsk_01M8C31J313ZH83TX1V9AFCNTF -->
- [ ] 4.2 接上生命週期資料。讓本機、遠端與宿主取得相同的詳細查詢、品質狀態、工單全文、已封存證據及時間資訊。寫入自由欄位、落地及就緒時間，補齊認領、討論還原、詞彙與遠端排序。範圍 crates/engine/、crates/host/、crates/protocol/、crates/adapters/。驗證：cargo test -p speclink-cli 的 local／remote 對照及 cargo test -p speclink-core 的 golden，斷言 whyExcerpt、stage、createdById、readyAt、modifiedAt、review、verify、promotedTo、landedCommit 的拼法、型別和缺席條件。 <!-- speclink-task:tsk_01M8C31J31YVF9E79ZY9K4GBF9 -->
- [ ] 4.3 接上政策與技能。加入 Pi／Cursor、模型選項、舊 schema 空值相容、spec_locale、遠端技能產物與語言回歸；同步 propose、ingest、quality 等技能並保留 fork 安全政策。範圍 crates/engine/speclink-core/、crates/adapters/speclink-cli/、crates/host/、apps/desktop/。驗證：cargo test -p speclink-core 與 cargo test -p speclink-cli，比對六種工具、tw／ja／en 與未設定、保留正式規格原名、設定重寫及舊語言 fallback。 <!-- speclink-task:tsk_01M8C31J31Y7H217B7QGD2REEB -->

## 5. 桌面與 Herdr

- [ ] 5.1 建立桌面與面板回歸測試。先重現看板日期、建立者與任務摘要、遠端專案狀態同步、工作標籤、分頁與等候原因、可見性、指令面板與詳情連結的缺口。範圍 apps/desktop/、packages/ui/、integrations/herdr/、scripts/herdr/。驗證：桌面和 UI 的 Vitest 及 Herdr 腳本測試能走到午夜、改名、失聯、終端機環境和窗體可見性分支，也涵蓋繼續本機分頁跨重啟。 <!-- speclink-task:tsk_01M8C31J3175D18WYXCAZ3QXEA -->
- [ ] 5.2 接上桌面顯示與設定。看板、詳情和系統匣用共同的時間及狀態，新增 Pi／Cursor、品質模型與三語任務標記，保留 fork 的裝置排序、專案設定與安全歷史。範圍 apps/desktop/、packages/ui/。驗證：cargo test -p speclink-desktop-core、npm test -w @speclink/desktop 與 npm test -w @speclink/ui，涵蓋本地日期、缺欄位、遠端被收回後復原、並存專案分頁啟動及既有更新來源。 <!-- speclink-task:tsk_01M8C31J31CN94Y3TV3PGBCE0P -->
- [ ] 5.3 接上 Herdr 面板與 CLI 回報。沿用現有面板及工作樹流程，支援 hc run、Pi／Codex 等 agent、等待標籤、路徑開啟、看不見面板時停抓及回報 opt-out。範圍 integrations/herdr/、scripts/herdr/、crates/adapters/speclink-cli/。驗證：cargo test -p speclink-cli 和 node --test scripts/*.test.mjs scripts/*/*.test.mjs，在 fixture 模擬舊版、目前版與非 Herdr 終端，stdout 和退出碼不受回報影響。 <!-- speclink-task:tsk_01M8C31J318W8XZZKMCJQQN3WA -->

## 6. 收尾驗證

- [ ] 6.1 對新 API、參數與設定套用 audit 檢查清單，逐項在清冊記錄身分、scope、前綴、版本衝突、備份空目標與無變更寫入行為的驗證結果，將與 fork 行為的差異回填文件、規格與手冊。補上 macOS 本機安裝的簽章身分，驗證明示身分優先、找不到憑證仍可建置、有身分卻驗不過時不安裝，並保留正式發行簽章。範圍 crates/、docs/、openspec/、.github/workflows/、scripts/。驗證：逐項對照清冊及 affected crates 測試，確認正反案例、舊資料相容與發行來源；執行 node --test scripts/*.test.mjs scripts/*/*.test.mjs 確認中英文件、連結與發行腳本。 <!-- speclink-task:tsk_01M8C31J31Y9MXN914CMS97GAF -->
