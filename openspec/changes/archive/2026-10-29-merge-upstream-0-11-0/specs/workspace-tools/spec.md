## ADDED Requirements

### Requirement: 本機專案明寫預設規格資料夾

本機初始化、啟用既有專案及本 repo 的 .speclink.yaml SHALL 明寫 spec_dir: openspec。新專案在沒有使用者指定路徑時 SHALL 在 openspec/ 建立規格骨架；既有有效 spec_dir SHALL 原樣保留，包含使用者明寫的 speclink 或 docs/specs。既有 .speclink.yaml 缺少 spec_dir 且實際使用 openspec/ 時，update SHALL 在任何上游遷移判定之前補入 spec_dir: openspec；設定損壞或補值失敗 SHALL 非零結束、回報原因，SHALL NOT 搬移規格或開始技能同步。補值 SHALL 保留 tools、remote、未知鍵及原有設定內容，使用者環境變數不寫回檔案。已使用 speclink/ 的專案 SHALL 維持該實際路徑，不為補預設而切去另一棵目錄。

#### Scenario: 新專案的明確預設
- **WHEN** 使用者在空目錄以所選工具執行 speclink init
- **THEN** exit code 為 0，.speclink.yaml 含有效的 spec_dir: openspec，openspec/specs/ 與 openspec/changes/archive/ 存在，沒有建立 speclink/ 規格資料夾

#### Scenario: 更新先補值再判斷遷移
- **WHEN** .speclink.yaml 只有 tools 清單且 openspec/ 已有文件，執行 speclink update
- **THEN** exit code 為 0，設定補入 spec_dir: openspec，原規格文件與資料夾名稱不變，技能使用 openspec/ 路徑，再次執行不重複新增鍵

#### Scenario: 明寫路徑與已遷移專案不被覆寫
- **WHEN** 更新明寫 spec_dir: docs/specs 的專案，或未明寫 spec_dir 但僅有 speclink/ 規格的專案
- **THEN** 各自仍讀取 docs/specs/ 或 speclink/，不另建或改讀 openspec/

#### Scenario: 壞設定拒絕寫入
- **WHEN** .speclink.yaml 不能解析，或補入預設值的寫入失敗
- **THEN** exit code 非 0、stderr 指出設定或寫入原因，規格未搬移、技能未更新，既有設定不被不完整文字取代

## MODIFIED Requirements

### Requirement: 工作區補齊入口
<!-- BEFORE: 既有啟用與初始化不明寫預設路徑，亦未涵蓋上游的新佈局與雙內建流程。 -->

引擎 SHALL 提供冪等的工作區補齊（adopt）入口：對「已有 openspec/ 但無 `.speclink.yaml`」的目錄，補 openspec/ 骨架缺件（specs/ 與 changes/archive/ 目錄；config.yaml 僅在不存在時寫入範本）、於專案根寫入 `.speclink.yaml` 記錄所選 tools 與有效的 spec_dir: openspec、為每個所選工具生成技能檔，並確保專案根 `.gitignore` 涵蓋 `.speclink/` 工作資料夾。既有 openspec/ 文件（specs、changes、discussions 及既有 config.yaml）SHALL 零觸碰。入口 SHALL NOT 受 init 的「已初始化即擋下」限制；tools 空清單 SHALL 回錯誤。重複執行 SHALL 冪等收斂於相同結果。

#### Scenario: 補齊工作區檔且既有內容零觸碰

- **WHEN** 對含 openspec/（內有規格文件與自訂 config.yaml）但無 .speclink.yaml 的目錄以 tools=[claude] 執行 adopt
- **THEN** 專案根產生 .speclink.yaml（tools 含 claude）與 .claude/skills/ 技能檔，openspec/ 內既有文件與 config.yaml 位元級不變，且不生成 CLAUDE.md

#### Scenario: 骨架缺件補齊

- **WHEN** 對 openspec/ 內缺 specs/ 目錄與 config.yaml 的未啟用目錄執行 adopt
- **THEN** specs/、changes/archive/ 目錄建立，config.yaml 以範本寫入

#### Scenario: 工作資料夾納入版控忽略

- **WHEN** 對 .gitignore 不存在、或存在但未涵蓋 `.speclink/` 的未啟用目錄執行 adopt
- **THEN** 專案根 .gitignore 涵蓋 `.speclink/`，且該檔既有內容原樣保留

##### Example: 既有 .gitignore 追加而非覆寫

- **GIVEN** 專案根 .gitignore 內容為 `node_modules/\ndist/\n`
- **WHEN** 以 tools=[claude] 執行 adopt
- **THEN** .gitignore 仍含 `node_modules/` 與 `dist/` 兩行，並多出 `.speclink/` 條目

#### Scenario: 重複執行冪等

- **WHEN** 對同一目錄以相同 tools 連續執行 adopt 兩次
- **THEN** 第二次成功且所有生成檔內容與第一次相同

#### Scenario: tools 空清單拒絕

- **WHEN** 以空的 tools 清單執行 adopt
- **THEN** 回單行錯誤，目錄零寫入

#### Scenario: 補齊啟用不搬走原規格
- **WHEN** 既有 openspec/ 專案沒有 .speclink.yaml，從 CLI 或桌面確認啟用
- **THEN** 既有 openspec/ 與其文件保持原狀，新 .speclink.yaml 明寫 spec_dir: openspec，技能指向 openspec/，不將啟用當成搬到 speclink/ 的匯入
