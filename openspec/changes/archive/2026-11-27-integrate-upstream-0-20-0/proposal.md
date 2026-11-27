## Why

- 用 AI 跑規格、變更與品質關卡的開發者、PO 和 PM，需要用到原作者 0.20.0 的桌面看板、遠端流程及 Herdr 改善。
- 把 Speclink 接進自家系統的開發者需要宿主 SDK、匯入匯出及事件契約。升級也要保住 fork 的安全審查、品質歷史、工作區資料與發行設定。

## What Changes

- 以 fork 0.12.2（origin/main 的 b57c99d0）為整合基準。上游從已納入的 26de3b9f 比到 v0.20.0 的 7f1105a523189f2a390be97d9522e896510e0880，含 0.13.0 到 0.20.0 的累積行為。
- 納入 Node 宿主 SDK、儲存接口與三種後端、共用動詞、身分與權限掛鉤、事件查詢及範圍匯出／匯入。舊 SDK 的建構和回傳形式保留相容入口，新入口採上游契約。
- 納入瀏覽器登入、系統 TLS 信任、跨 repo 綁定、遠端技能、存取收回與復原提示。新版參考 Server 的儲存格式與舊版不同，提供舊版備份、新版還原的升級路徑和回復驗證。
- 納入自由欄位、任務證據、落地 commit、章與工單、已封存查詢、就緒日與建立者身分。CLI、桌面、Server 與內嵌宿主共用資料來源。
- 納入 Pi 與 Cursor、各品質關卡的工具模型設定、依宿主能力裁剪的技能、正確的規格語言及任務說明。保留 Codex／Antigravity 共用檔案與 fork 安全技能。
- 納入看板卡片的更新時間、建立者篩選、遠端圖示提示、繼續本機分頁的恢復，以及 Herdr 的新分頁派工、工作標籤、等候原因、可見性與深層連結。
- 本機重裝 macOS 桌面版時，若有 Developer ID 則使用它，讓同一簽章身分下的 CLI 和桌面可繼續讀取金鑰圈。

## Capabilities

### New Capabilities

- `host-kit` 提供 Node 內嵌宿主與 HTTP 入口。node-sdk 規定既有綁定，host-runtime 規定內部執行，兩者皆未定義這個由呼叫端注入身分、儲存與掛鉤的公開入口。
- `store-port-contract` 定義可由 JS 或 Rust 實作的範圍文件儲存接口與合格測試。teamstore-contract 的既有專案與資料庫接口涵蓋的是舊後端。

### Modified Capabilities

- `node-sdk` 新增匯出、技能渲染選項、命令與事件查詢，處理舊呼叫端相容。
- `reference-server` 加入新儲存格式、升級檢查、認證與權限行為。
- `client-protocol` 擴充遠端登入、事件、詳細資料與動詞。
- `change-lifecycle` 擴充變更與討論的自由欄位、就緒狀態與落地資訊。
- `workspace-tools` 加入 Pi、Cursor、遠端技能與裁剪規則。
- `workflow-config` 加入品質站模型、規格語言與無變更寫入規則。
- `desktop-app` 新增看板卡片、詳情、任務列和遠端提示行為。
- `desktop-release` 加入本機安裝的 Developer ID 簽章與驗證。
- `herdr-integration` 改善終端機回報、面板更新、技能送達與開工分頁。

## Impact

- 已核對 node-sdk、desktop-app、client-protocol 的 Purpose，另比對 host-runtime、teamstore-contract 的涵蓋範圍。
- 影響範圍涵蓋 `crates/engine/`、`crates/store/`、`crates/host/`、`crates/protocol/`、`crates/adapters/`、`apps/desktop/`、`apps/server-web/`、`packages/ui/`、`integrations/herdr/`、`scripts/herdr/`、`docs/` 與 `openspec/` 下的規格、手冊和相關回歸測試。
- CLI 新增或延伸 list --archived --json、list／show --json --detail、review／verify show --json --detail、evidence show、change landed、change／discuss field、discuss restore、claim --release／--json、language replace --stdin 和 workflow-config subagent-model。各動詞以 v0.20.0 共用解析器及對照測試釘住旗標、回應、退出碼與輸出位置。language replace 從 stdin 接收完整內容。
- 相容性影響：詳細查詢與討論輸出會增加欄位，逐字比對完整 JSON 的呼叫端需改讀所需欄位；舊欄位名稱與預設人眼輸出依現行契約保留。宿主採命令白名單時須加入上游的新命令。新版參考 Server 遇舊儲存需先走備份還原。
- subagent_model 未指定時沿用原工作流選擇；SPECLINK_TERMINAL_REPORT=false 可關閉 Herdr 終端機回報。既有專案的 spec_dir 設定與 fallback 路徑保留。技能會更新 Claude Code、Codex、Copilot、Antigravity，並加入 Pi 與 Cursor；依設定同步安全與 worktree 技能。
