## ADDED Requirements

### Requirement: 新舊預設資料夾都可遷移到遠端

本機遷移到遠端（含採用伺服器內容）SHALL 接受明寫 spec_dir: openspec 的 Fork 預設專案與上游 speclink/ 佈局；不得僅因 spec_dir 有值就拒絕 openspec。明寫非這兩種預設的自訂路徑，以及兩種規格目錄並存時 SHALL 在寫入遠端前拒絕並說明原因。上傳已封存變更資料檔時 SHALL 以 .meta.yaml 為鍵；舊 .openspec.yaml 讀取相容，兩者並存以 .meta.yaml 為準，不上傳兩份權威資料。

#### Scenario: 明確 Fork 預設可遷移
- **WHEN** spec_dir: openspec 的專案完成遠端匯入
- **THEN** 原目錄備份為 openspec.migrated-<日期>，遠端已封存資料檔鍵為 .meta.yaml，本機轉為遠端專案

#### Scenario: 自訂或並存佈局拒絕
- **WHEN** 專案明寫 spec_dir: docs/specs，或 openspec/ 與 speclink/ 規格目錄並存
- **THEN** 遷移被拒絕並指出原因，本地資料與遠端內容均無寫入

## MODIFIED Requirements

### Requirement: 遷移成功後才轉換本地且一律備份
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

遷移流程 SHALL 於 import 成功後才觸碰本地：本機規格資料夾 SHALL 改名為帶日期的備份目錄 `<資料夾名>.migrated-<日期>`（例如 `speclink.migrated-2026-11-02`；舊佈局為 `openspec.migrated-2026-11-02`；同名既存則加序號）——SHALL NOT 存在刪除本地內容的路徑；隨後寫入 remote marker 並原地將分頁轉為 remote（checkoutRoot 為該資料夾）。import 失敗（scope 非空、網路、驗證）時本地 SHALL 分毫不動且 UI 原樣呈現錯誤。遷移確認對話 SHALL 指出目標 Project/Repo 與本地備份行為。

#### Scenario: 失敗零副作用

- **WHEN** 遷移於 import 步驟因 scope 非空失敗
- **THEN** 本地規格資料夾原樣存在、無 marker 寫入、分頁維持 local，錯誤原樣呈現

#### Scenario: 成功後資料夾轉為 checkout

- **WHEN** 遷移成功完成
- **THEN** 資料夾含帶日期的備份目錄與 remote marker、原分頁原地轉為 remote 分頁且 checkoutRoot 為該資料夾，CLI 於該資料夾以 remote 模式運作

#### Scenario: 備份目錄依實際資料夾命名

- **WHEN** 規格資料夾為 openspec/ 的專案在 2026-11-02 遷移成功，資料夾內已有 speclink.migrated-2026-11-02
- **THEN** 本機規格資料夾改名為 speclink.migrated-2026-11-02-2
