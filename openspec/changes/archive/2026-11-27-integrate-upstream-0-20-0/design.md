## Context

前次已把上游 26de3b9f 整合進 fork。這次的來源是上游 v0.20.0 標籤 dd4289ca 指向的 7f1105a523189f2a390be97d9522e896510e0880。兩個上游提交之間有 1,027 個提交、1,482 個變動檔，當中包含大量上游的工作流歷史。fork 0.12.2 的 origin/main b57c99d0 包含本機 main 尚未有的 macOS DMG 簽章修正。

## Goals / Non-Goals

- 範圍內：把上游 0.13.0–0.20.0 的累積產品行為接回 fork，保住既有公開入口、資料與發行來源。以工作流、遠端與 SDK、桌面與 Herdr 分批接合，每批有對照測試。
- 範圍外：搬移本 repo 的 openspec 資料夾、複製上游封存歷史、改本產品版號、公開發版、操作任何線上 Server 或安裝外部 Herdr。後續 PR 的目標固定為 mosutw/speclink 的 main。

## Decisions

### 基準與分批整合

在隔離的工作樹以 fork 0.12.2 或包含該提交的較新 fork main 為起點，先確認本機現有 Herdr 修改和未追蹤檔未被帶進工作樹。以 Git 比較 26de3b9f 至 7f1105a5 的實際檔案和上游規格、更新日誌，建立可逐項驗收的整合清冊。每項標記為納入、fork 已有、需相容處理或屬上游專案自用。整桶覆蓋會抹掉 fork 的規格歷史、發行及安全政策。Git 路徑以 repo-relative 表示，Windows 的分隔符與換行以原有測試守住。

### 共用動詞與儲存分層

speclink-core 保留領域判定，speclink-fs 處理工作區 IO，speclink-host 負責身分、範圍、權限、提交與事件。引入上游 speclink-verbs，讓 CLI 與 Node 共用解析及輸出，遠端經 Rust protocol typed DTO，桌面與 Server 沿用同一套 Host 邏輯。以 Store Port 承接範圍內文件提交，SQLite、PostgreSQL、FS 跑同一組合格測試。新版 Host Kit 採上游公開入口。既有 createEngine 自訂 store 與原回傳保留在舊入口，沿用 fork 的查詢轉接，不複製動詞實作。

驗收所讀取的政策、索引和證據使用 Rust 內建後端的唯讀版本條件。資料庫交易或檔案鎖內先核對條件，再寫入章、封存資料和事件。落後的版本讓整筆提交失敗，來源文件的內容、版本與修改時間維持原樣。第三方 JS store 繼續使用 list、read、commit，未實作此能力的後端收到帶條件的提交時回報拒絕。

### 資料升級與復原

舊 TeamStore 維持舊 SDK 和舊版匯出的相容邊界。新版 Server 遇到舊儲存先拒絕啟動，由舊 CLI 做備份，驗證後還原至空的新後端；驗證資料與身分後才切換流量。失敗保持舊庫可重新掛回，空目標的還原交易失敗時不得留下半份範圍。新入口也支援非空範圍的 replace，dryRun 唯讀，revision 競爭時回報衝突且保留原狀。備份及匯入流程保留 fork 的規格、工作流政策、品質與安全歷史。

### 工作流與語言

Pi／Cursor 走 workspace 工具描述，新增工具沿用同步、狀態辨識與清理。保留現有 Codex／Antigravity 共用目錄、Copilot、Claude 與安全技能。遠端技能取自 Server，沿用 checkout、gitHistory 和現有政策判定。設定用既有 YAML 純量跳脫，讀取舊 YAML 時新鍵可省略。預設規格資料夾支援原 fork 與上游的佈局辨識，這個 repo 繼續明寫 openspec。新 delta 依 spec_locale 出稿，沿用正式規格既有名稱。

### 桌面與 Herdr

桌面資料整理歸 speclink-desktop-core，Tauri command 只轉接。重用 @speclink/ui 的看板卡片、任務列、工單、提示與篩選元件，設定沿用既有表單及共用編輯器。日期分組由共用 helper 處理，遠端連線由單一狀態來源更新圖示列與系統匣。有遠端設定的本機專案保留先前選擇，背景檢查與切回分頁都確認其路徑仍是專案根。macOS 本機安裝使用明示或找到的 Developer ID，驗證 bundle 內 CLI 的簽章才安裝；無憑證走 ad-hoc，正式發行維持 fork 的簽章流程。Herdr 面板沿用現有模組及語言字典，派工依既有 worktree 政策，透過 hc run 開新分頁。CLI 終端回報只在 Herdr 環境中嘗試，與 stdout、stderr、退出碼分離。

## Implementation Contract

- 身分與資料：桌面、CLI、Server、SDK 處理同一份生命週期欄位。登入優先使用服務端宣告的瀏覽器、裝置、存取金鑰順序，登出清掉同一 Server 的本機憑證。多 repo 的綁定要求明確指定，認證失效、離線、存取被收回或資料 revision 衝突均回報可辨的狀態，拒絕的寫入保留原資料。
- 讀取契約：以 v0.20.0 的具名動詞、旗標及樣本對照詳細輸出、歸檔、證據、工單與遠端 trace／plan。新增欄位以 camelCase 表達，凍結的舊欄位保留原拼法。成功 CLI 輸出到 stdout，錯誤到 stderr 且非零退出，no-color 以無 ANSI 輸出對照現有 golden。遠端讀取使用同一 Host 授權。
- 寫入契約：新欄位、認領釋放、討論還原、落地 commit 及流程政策維持各路徑等價。最後一個任務完成時記錄 ready_at，任務仍未做完就不輸出 readyAt。組合寫入在同一交易交付，若本機寫入就緒時間後任務檔案更新失敗，保留時戳但隱藏 readyAt，讓重試重算。
- 驗收：Rust 分 crate 單元與整合測試、CLI local／remote parity、SDK 型別與實際 binding、各工作區 Vitest、三種儲存的備份還原、fork 的 DMG 簽章腳本與各平台 CI。整合清冊須能用提交、規格或測試對照每個使用者可見差異。

## Risks / Trade-offs

- 大量差異容易蓋掉 fork 行為。以現行 golden、CLI 整合測試和 fork 專屬規格逐批對照，只有列入清冊的輸出差異更新 golden。
- Windows、macOS、Linux 的路徑、憑證、終端及發行流程不同。CI 驗證三平台，Herdr 橋接同時測有／無 Herdr 環境和 opt-out。
- 遠端資料和權限易在升級時退化。對每個現行後端跑舊版備份、新版空庫還原與失敗回復，資料對照包含安全歷史。舊版 SDK 以獨立相容 fixture 驗證，型別和 runtime 一起檢查。

## Migration Plan

先用舊版做可驗證備份，保留舊程式與舊儲存的唯讀副本。新版以新的空儲存還原，驗證範圍、身分與可讀寫內容，再切換。還原或驗證不過時恢復舊版及舊儲存，作廢這次新目標。既有本機專案依原路徑開啟，技能由既有 update 流程同步。沒有可靠對照的上游行為在整合清冊標記證據缺口，補上對照後才算驗收。
