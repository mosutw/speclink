# spec-dir-migration Specification

## Purpose

規格資料夾探索、舊資料檔相容與技能同步的共用契約，涵蓋上游 0.11.0 的遷移機制、本 Fork 明寫 openspec 預設與使用者確認後單向轉為 speclink 的接點。此能力處理實際路徑與新舊格式的讀寫邊界，不保證 OpenSpec CLI 或其生態工具的完整互通。

## Requirements

### Requirement: init 匯入 OpenSpec 專案

已有 openspec/ 但沒有 .speclink.yaml 的本機專案 SHALL 由既有啟用入口就地補齊：寫入所選 tools 與 spec_dir: openspec、補骨架缺件並安裝 Speclink 技能，不改名 openspec/、不批次改名變更資料檔、不覆蓋既有規格或 OpenSpec 技能。啟用與初始化不得繞過設定和工具選擇驗證；已有 .speclink.yaml 的重複 init 維持既有 Already initialized 語意。

#### Scenario: 啟用保留既有工具文件
- **WHEN** 使用者啟用含規格、舊 .openspec.yaml 與 OpenSpec 技能的 openspec/ 專案
- **THEN** 新設定含 spec_dir: openspec，原規格及 OpenSpec 技能逐位元不變，另生成所選 Speclink 技能，不宣稱兩套 CLI 的全流程已驗證互通


<!-- @trace
source: merge-upstream-0-11-0
updated: 2026-10-29T05:48:54+08:00
-->

---
### Requirement: update 自動遷移舊佈局

speclink update（無 stdin）SHALL 先完成 workspace-tools 所定的 openspec 預設補值，再呼叫上游遷移規劃。明確 spec_dir（包括 openspec）及遠端模式 SHALL 不自動搬移規格資料夾或批次改名資料檔；符合上游其他遷移條件的專案沿用原有安全檢查、延後原因、部分改名失敗可續作及不得覆蓋較新 .meta.yaml 的規則。沒有 .speclink.yaml 的 OpenSpec 專案 SHALL 不由 update 擅自啟用或搬移，提示使用 init。成功 exit code 為 0；寫入或改名失敗為非 0，指出失敗路徑；不自動 commit。技能同步使用完成後的實際路徑。

#### Scenario: 明確預設阻止資料夾與批次資料檔遷移
- **WHEN** 有 spec_dir: openspec 的專案含舊 .openspec.yaml，執行 speclink update
- **THEN** openspec/ 與舊資料檔保持原名，只有受管技能依實際路徑更新；個別變更之後的 metadata 寫入仍遵循 .meta.yaml 新格式

#### Scenario: 遷移失敗不覆蓋已完成部分
- **WHEN** 上游允許遷移的專案在部分資料檔改名後失敗
- **THEN** 錯誤指出失敗路徑，已改名的內容保留，舊檔可讀，再次執行可繼續，既有 .meta.yaml 不被舊檔覆蓋


<!-- @trace
source: merge-upstream-0-11-0
updated: 2026-10-29T05:48:54+08:00
-->

---
### Requirement: 技能檔依實際規格資料夾渲染

`speclink init`、`speclink update` 與其他同步技能檔的入口 SHALL 以專案實際使用的規格資料夾（依「規格資料夾的預設位置與舊佈局相容」決定）渲染技能原稿裡的 `{{SPEC_DIR}}`。技能原稿 SHALL NOT 寫死 `openspec/`，變更資料檔一律寫作 `.meta.yaml`。

#### Scenario: 遷移後的技能路徑

- **WHEN** 專案以 `speclink/` 為規格資料夾，執行 `speclink update`
- **THEN** 生成的 speclink-commit 技能檔中的範例路徑以 `speclink/changes/<name>/` 開頭，沒有 `openspec/`

#### Scenario: 暫緩中的專案仍渲染舊路徑

- **WHEN** 專案因有其他 worktree 而暫緩遷移，仍只有 `openspec/`，執行 `speclink update`
- **THEN** 生成的技能檔中的規格路徑為 `openspec/`

#### Scenario: 自訂 spec_dir

- **WHEN** `.speclink.yaml` 含 `spec_dir: docs/specs`，執行 `speclink update`
- **THEN** 生成的技能檔中的規格路徑為 `docs/specs/`


<!-- @trace
source: merge-upstream-0-11-0
updated: 2026-10-29T05:48:54+08:00
-->

---
### Requirement: 技能足跡同步不遷移規格資料夾

切換 worktree 政策後的技能足跡同步（CLI `speclink workflow-config set worktree <true|false>`，以及桌面產出流程頁 worktree 開關的存檔）SHALL 只同步技能檔，SHALL NOT 移動規格資料夾或改名變更資料檔；符合遷移條件的舊佈局專案 SHALL 照常以 `openspec/` 同步技能檔，stdout SHALL NOT 出現遷移相關的提示。自動遷移 SHALL 只由 `speclink update` 與桌面技能檔提示的「更新」依原條件執行，兩者都會說明改名內容與提醒；使用者明確選取模式時另走「規格資料夾模式的明確單向轉換」，不得由 worktree 政策或工具勾選存檔代為觸發。

#### Scenario: CLI 切換 worktree 不遷移

- **WHEN** 已 commit、沒有其他 worktree、`.speclink.yaml` 沒有 `spec_dir` 的舊佈局專案執行 `speclink workflow-config set worktree true`
- **THEN** exit code 為 0，`openspec/` 原封不動、沒有建立 `speclink/`，兩顆 worktree 技能生成且其中的規格路徑為 `openspec/`，stdout 沒有遷移相關的提示

#### Scenario: 桌面切換 worktree 不遷移

- **WHEN** 使用者在符合遷移條件的舊佈局專案打開產出流程頁的 worktree 開關並存檔
- **THEN** `openspec/` 原封不動，技能足跡出現兩顆 worktree 技能，看板持續自動更新


<!-- @trace
source: merge-upstream-0-11-0
updated: 2026-10-29T05:48:54+08:00
-->

---
### Requirement: 規格資料夾的預設位置與舊佈局相容

系統 SHALL 沿用上游的集中路徑探索：有效的明確 spec_dir 優先，其次辨識已有的 speclink/ 規格資料夾、再辨識 openspec/。本 Fork 初始化及既有 openspec/ 專案更新 SHALL 依 workspace-tools 明寫 spec_dir: openspec，故預設實際路徑為 openspec/；上游的內部 fallback 不作第二套改寫。新舊資料夾並存時，明確 spec_dir SHALL 決定唯一來源，不合併兩份內容。命令警告、畫面與技能 SHALL 依解析結果顯示；明確 openspec 路徑 SHALL 不被標示為必須搬走的過期佈局。

#### Scenario: 預設值勝過並存目錄
- **WHEN** 專案有 openspec/ 與 speclink/，且 .speclink.yaml 為 spec_dir: openspec
- **THEN** 命令只讀 openspec/，不搬移或合併目錄，不提示必須搬至 speclink/

#### Scenario: 已使用上游目錄仍可開啟
- **WHEN** 未指定 spec_dir 的專案僅有作為規格資料夾的 speclink/
- **THEN** 系統讀取 speclink/，不因本 Fork 的新專案預設而改讀空的 openspec/


<!-- @trace
source: merge-upstream-0-11-0
updated: 2026-10-29T05:48:54+08:00
-->

---
### Requirement: 規格資料夾模式的明確單向轉換

系統 SHALL 提供本機 OpenSpec（openspec/）轉為 Speclink（speclink/）的專用操作，僅在使用者明確確認後執行。模式 SHALL 以實際規格資料夾與既有 .speclink.yaml 的 spec_dir 判定，不新增 spec_mode 鍵；新專案預設仍為 OpenSpec。此操作 SHALL NOT 透過暫時刪除 spec_dir 來觸發一般 update，SHALL NOT 提供 Speclink 轉回 OpenSpec 的操作，且 SHALL NOT 將自訂目錄強制歸類或將 remote 的資料搬到本機。

轉換 SHALL 先驗設定、工具／降級、來源／目的地與既有 worktree／未提交修改守門；拒絕 SHALL 在轉換寫入前完成。目的地已有任何項目（含空目錄、檔案、symlink）、來源 symlink、兩目錄並存、自訂來源、remote、linked worktree、其他 worktree 或 openspec/ 內有未提交／未追蹤內容時 SHALL 拒絕，指出原因且保留原內容。

確認且通過守門後 SHALL 保留原設定與最小轉換進度、搬整個 openspec/ 為 speclink/、只把 spec_dir 改為 speclink，保留 tools、註解、未知鍵、規格／討論／變更／品質工單／完成歷史與既有 metadata 檔內容，再按已儲存工具選集同步技能。此明確轉換 SHALL NOT 批次改 metadata 名稱，資料格式沿既有 .meta.yaml 優先／舊檔 fallback 與新寫入規則。完成後一般 update SHALL 保持 Speclink 路徑，重複轉換 SHALL 不重搬、不重複設定鍵。

搬移失敗 SHALL 留在原模式；設定寫失敗 SHALL 回復原路徑與設定，回復也失敗時 SHALL 保留資料與恢復記錄、回報實際路徑及未完成狀態，不回成功或顯示正常的空專案。新路徑與設定已生效後技能同步失敗，SHALL 留在 Speclink 模式，顯示可重試的未完成狀態；重試或中斷後重開 SHALL 能同向完成，成功才清除轉換記錄，SHALL NOT 自動轉回 OpenSpec。

#### Scenario: 明確確認後單向轉換並保留歷史
- **WHEN** 已啟用本機專案只有 openspec/、設定為 spec_dir: openspec、工具為 claude/codex/copilot/antigravity，通過守門後使用者確認轉為 Speclink
- **THEN** 規格資料夾變成 speclink/、設定為 spec_dir: speclink，四工具選集與未知鍵不變，原變更、討論、工單及 checks 歷史逐位元保留，技能改用 speclink/，沒有反向轉換入口

##### Example: 規格資料夾轉換結果
| 操作前 | 動作 | 操作後 |
| --- | --- | --- |
| openspec/，spec_dir: openspec | 確認轉換 | speclink/，spec_dir: speclink |
| speclink/，spec_dir: speclink | 再次請求同向完成 | 維持 speclink，不重複鍵或搬移 |
| speclink/，spec_dir: speclink | 請求轉回 OpenSpec | 不提供此操作，原資料不變 |
| docs/specs/，spec_dir: docs/specs | 嘗試用兩模式入口轉換 | 拒絕，保留自訂路徑 |

#### Scenario: 目的地與使用中 worktree 拒絕轉換
- **WHEN** openspec/ 專案的 speclink 目的地已有項目，或有其他 worktree／linked worktree／未提交的規格修改
- **THEN** 操作回報具體原因，規格資料夾、設定、技能與歷史均未改動

#### Scenario: 設定寫入失敗不留下假成功
- **WHEN** 已搬資料夾但 spec_dir 無法寫入
- **THEN** 回報失敗並回復原路徑與設定；回復也失敗則保留原設定備份與恢復記錄，顯示未完成和實際資料路徑，重開不被當成新的空專案

#### Scenario: 技能失敗或程序中斷可向前續作
- **WHEN** 新路徑與設定已生效，但技能同步失敗或轉換中程序中斷後重開
- **THEN** 顯示實際生效的路徑與未完成提示，僅提供同向重試，原歷史可保留／查閱，不提供切回 OpenSpec，續作完成後清除記錄且技能依 speclink/ 渲染

<!-- @trace
source: merge-upstream-0-11-0
updated: 2026-10-29T05:48:54+08:00
-->