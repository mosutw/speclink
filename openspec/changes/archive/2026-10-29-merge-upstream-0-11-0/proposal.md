## Why

- 透過 AI 代理操作 SDD 的開發者，以及使用桌面／遠端專案協作的 PO、PM，需要取得原作者 0.11.0 的產出流程、流程成本與品質關卡改進，同時保留目前 Fork 的安全審查、品質歷史及交付功能。
- 原作者把預設規格資料夾改為 speclink/；本 Fork 依討論結論，只以明確的 spec_dir: openspec 保留原路徑，其餘盡量沿用上游，降低後續整合負擔。保留目錄不等於 OpenSpec 全流程互通已驗證。

- 2026-10-28 操作回饋：桌面專案設定尚未顯示規格資料夾。使用者要求兩種資料夾模式：OpenSpec（openspec/，預設）與 Speclink（speclink/），並由使用者確認後單向轉換，選取 Speclink 時先提示轉換後無法轉回 OpenSpec 模式。

## What Changes

- 以 Fork e272d4be 為保留基準，整合上游 v0.11.0 對應 64623a6bfbe866f4ae33e37b918cb39b941f38cb；共同祖先固定為 05e5e65ac043d04e6e0b2a396c0bb0997e52ae70。逐檔盤點原作者與 Fork 改動，不帶入上游後續 main 或直接搬走本 repo 的規格。
- 本機初始化及啟用預設產生 spec_dir: openspec；本 repo 補上同值；既有 openspec/ 專案更新先補值再判定遷移，已有自訂 spec_dir 及已使用 speclink/ 的專案維持實際路徑。桌面開啟、更新與遷移到遠端使用同一條路徑判斷。
- 桌面「專案設定 → .speclink.yaml」增加規格資料夾模式選項，以實際 spec_dir 顯示 OpenSpec 或 Speclink；選取 Speclink 先開確認框，取消零寫入，明確確認才搬移 openspec/ 並寫入 spec_dir: speclink。轉換成功後 OpenSpec 選項停用，程式不提供反向轉換。既有自訂路徑唯讀顯示，remote 不提供本機轉換。
- 明確轉換與一般 update 分開：一般更新仍保留 openspec 預設；明確轉換搬整個資料夾、保留規格／討論／品質歷史與工具選集，再依新路徑同步技能與監看。兩種模式只區分資料夾，不新增 OpenSpec 專用格式或完整互通保證。
- 新增 no-spec-change 內建產出流程、change schema 切換，以及不允許在沒有 specs 步驟的變更寫入 delta 的守門；propose、ingest、verify 配合調整。
- 增加品質輪數累加、帶時區的秒級 metadata 與已封存頁流程成本摘要；統計寫入併入 Fork 現有品質歷史交易，保留安全驗收條件。
- quality 首輪前比對主分支重疊檔，收尾蓋章選項提醒 CI；沿用上游的提醒語意，不增加自動 push 或強制 CI 守門。
- **BREAKING**：新 metadata 寫入採 .meta.yaml，讀取相容舊 .openspec.yaml；寫入同一變更時淘汰舊檔。預設資料夾留在 openspec/，但依賴舊資料檔名稱的外部腳本仍須調整。
- 保留安全審查與資料包、品質紀錄、Antigravity、系統／Server 語言預設、提案條列、桌面外觀／專案排序／Proxy 診斷、Windows 免安裝版，以及 Fork 更新來源、簽章與 CI 發佈設定。技能、快照及更新日誌從整合來源重產。

## Capabilities

### New Capabilities

- `spec-dir-migration`: 上游路徑解析、遷移與技能同步契約，加上本 Fork 的明確預設與使用者確認後的單向資料夾轉換。最接近的 workspace-tools 管設定與技能生成、workflow-schemas 管產出流程、change-lifecycle 管變更狀態；它們均未定義新舊規格目錄探索、遷移規劃及 metadata 改名的整體契約；名稱近似的 workspace-migration 處理本機到遠端，spec-validation 處理規格品質驗證，也不涵蓋此邊界。

### Modified Capabilities

- `board-card-order`: 既有 metadata 引用跟隨上游 .meta.yaml，其他行為保留。
- `change-lifecycle`: 秒級時戳、封存時間與新資料檔名稱。
- `change-plan`: 既有 metadata 引用跟隨上游 .meta.yaml，其他行為保留。
- `claude-code-session-focus`: 既有 metadata 引用跟隨上游 .meta.yaml，其他行為保留。
- `claude-code-skill-bar`: 既有 metadata 引用跟隨上游 .meta.yaml，其他行為保留。
- `claude-code-workflow-panel`: 既有 metadata 引用跟隨上游 .meta.yaml，其他行為保留。
- `client-protocol`: 桌面已封存清單的五個選填成本欄位。
- `command-runtime`: 既有 metadata 引用跟隨上游 .meta.yaml，其他行為保留。
- `context-projection`: 沿用上游投影佈局與新資料檔名，維持遠端唯讀投影。
- `desktop-app`: 已封存流程成本摘要與更新提示。
- `desktop-config`: 雙內建產出流程、預設路徑及實際路徑顯示，另提供兩種資料夾模式與不可逆確認。
- `discussion-docs`: 既有 metadata 引用跟隨上游 .meta.yaml，其他行為保留。
- `ingest-skill`: 需求改變時先切回 spec-driven 再補 delta。
- `propose-skill`: 依需求選擇是否改規格的產出流程。
- `quality-skill`: 首輪前主分支比對及收尾 CI 提醒。
- `remote-board-order`: 既有 metadata 引用跟隨上游 .meta.yaml，其他行為保留。
- `review-station`: 審查輪數累加並保留完成紀錄與原子寫入。
- `server-verb-api`: schema 切換端點與角色／revision 守門。
- `store-abstraction`: 新資料檔寫入、舊檔讀取與遠端封存鍵。
- `trace-verb`: 既有 metadata 引用跟隨上游 .meta.yaml，其他行為保留。
- `user-documentation`: Fork 預設路徑、單向模式轉換、純文字可攜性、品質關卡提醒與相容範圍。
- `verify-skill`: no-spec-change 行為檢查及可執行修正建議。
- `verify-station`: 驗證輪數累加並保留安全驗收與完成紀錄。
- `workflow-config`: show 新增 schema 並保留安全政策與系統語言語意。
- `workflow-schemas`: 新增 no-spec-change、切換 schema 與無 specs 拒寫 delta。
- `workspace-migration`: 明確 openspec 預設可遷移到遠端並保留本地備份。
- `workspace-tools`: 初始化及更新明寫 openspec 預設，啟用不搬移原規格。
- `worktree-overlay`: 既有 metadata 引用跟隨上游 .meta.yaml，其他行為保留。

## Impact

- 規格掃描：優先檢查 workspace-tools、workflow-schemas、change-lifecycle；後續依上游已封存增量及 Fork 的舊資料檔引用展開為上述能力，避免新增重複能力。
- 主要修改：`.speclink.yaml`、`crates/engine/speclink-core/src/workspace/`、`crates/engine/speclink-core/src/lifecycle/`、`crates/engine/speclink-core/src/quality/`、`crates/engine/speclink-fs/src/`、`crates/host/speclink-host/src/`、`crates/protocol/`、`crates/adapters/`、`crates/host/speclink-server/src/`、`apps/desktop/core/src/`、`apps/desktop/src/`、`apps/desktop/src-tauri/src/`、`packages/ui/src/`、`integrations/claude-code/speclink-skills/`。
- 主要新增：`crates/engine/speclink-core/src/workspace/migrate.rs`、`crates/engine/speclink-core/src/lifecycle/change_schema.rs`、`crates/engine/speclink-core/src/lifecycle/cost.rs`、`crates/engine/speclink-core/assets/schema/no-spec-change/`、`packages/ui/src/components/ProcessCostSummary.tsx`、`packages/ui/src/processCost.ts`。
- 衍生資產與操作文件：`crates/engine/speclink-core/assets/`、`crates/engine/speclink-core/tests/golden/`、`.agents/skills/`、`.claude/skills/`、`.github/skills/`、`README.md`、`README.zh-TW.md`、`docs/`、`openspec/manual/`、`apps/desktop/src/release-notes/release-notes.json`、`CHANGELOG.md`、`CHANGELOG.zh-TW.md`。逐檔盤點新增於 `docs/development/upstream-0.11.0-inventory.md`。
- 保留交付設定：`.github/workflows/`、`apps/desktop/src-tauri/tauri.conf.json`、`scripts/desktop/`、`scripts/release/`；只接受完成此整合必要的修改，不套用上游發佈帳號或簽章金鑰。本次不選定新版本號或發佈。
- CLI 契約：新增 change schema <name> <schema>，支援 --json 及 --no-color、無 stdin，成功 0、拒絕非 0；new artifact spec 保留 --stdin／--force 但在無 specs 流程拒寫；init／update 的規格預設及新舊資料檔效果如上。人眼與 JSON 的完整 payload 見 delta 規格。
- 相容性影響：workflow-config show 刻意新增人眼 schema 行與 JSON schema 欄位，保留 securityVerification 與系統語言語意；metadata 寫入精確時戳但既有清單仍輸出日期；桌面封存清單新增五個選填成本欄位，server 清單形狀不變。其餘輸出維持既有契約，刻意改動同批更新 golden 與 CLI 回歸。
- 技能影響：propose、ingest、verify、quality 及含路徑的所有受管技能，依整合 renderer 為 claude／codex／copilot／antigravity 與中性目標產生，保留 Fork 安全指引與對話／文件語言分工。安全政策目前停用，本提案不新增或啟動安全標準審查。

- 本次補充的介面範圍：apps/desktop 的 WorkspaceSettingsProvider 與 Tauri 新增專用 convert_spec_dir_to_speclink，沿用 SettingsSnapshot.specDir 並加選填的未完成轉換狀態；不新增 CLI 子指令、旗標、stdin、HTTP 或 Node SDK API，既有 CLI 人眼／JSON 輸出不變。本機 .speclink.yaml 仍僅以既有 spec_dir 記錄路徑，不新增 spec_mode 設定鍵。
