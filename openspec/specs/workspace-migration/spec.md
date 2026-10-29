# workspace-migration Specification

## Purpose

本地 workspace 遷移到 remote store 的語意：bundle 組裝涵蓋本地 workspace 全集、import 端點僅接受 CreateNew 且整批原子，以及遷移成功之後才把本地轉為 remote 模式並一律先備份。本 capability 保證遷移不會停在半套狀態，也不走本地與遠端並存後再合併那條路——兩個遷移入口都是如此。

## Requirements

### Requirement: import 端點僅限 CreateNew 且原子

server SHALL 提供綁定 scope 的 import 端點：接受 wire Bundle（format version、文件全集），模式固定 CreateNew——wire SHALL NOT 暴露 Overwrite。目標 scope 已持有任何文件時 SHALL 拒絕且不寫入任何內容；reader SHALL 拒於 403；未知 format version 或缺欄位 SHALL 拒收。成功時整包 SHALL 以單一原子提交落庫並回報 project revision 與逐文件結果——部分寫入狀態 SHALL NOT 存在。

#### Scenario: 空 scope 匯入後全面可讀

- **WHEN** 對空 scope 匯入含 changes、canonical specs、討論與 archived 的 Bundle
- **THEN** 回報逐文件 Created，隨後清單、文件、討論與 archived 各端點回傳與 Bundle 一致的內容

#### Scenario: 非空 scope 拒絕且零寫入

- **WHEN** 對已持有文件的 scope 呼叫 import
- **THEN** 回應衝突錯誤且訊息含 create-new 語意，scope 內容與 revision 皆未改變


<!-- @trace
source: local-remote-migration
updated: 2026-07-21
code:
  - Cargo.lock
  - apps/desktop/src-tauri/Cargo.toml
  - apps/desktop/src-tauri/src/lib.rs
  - apps/desktop/src-tauri/src/remote.rs
  - apps/desktop/src-tauri/tests/migration.rs
  - apps/desktop/src/App.tsx
  - apps/desktop/src/__tests__/App.test.tsx
  - apps/desktop/src/__tests__/migrationDialog.test.tsx
  - apps/desktop/src/__tests__/remoteOpen.test.ts
  - apps/desktop/src/__tests__/workspaceChooser.test.tsx
  - apps/desktop/src/adapter/migration.ts
  - apps/desktop/src/components/MigrationDialog.tsx
  - apps/desktop/src/components/RemoteConflictDialog.tsx
  - apps/desktop/src/components/WorkspaceChooser.tsx
  - apps/desktop/src/i18n/messages.ts
  - apps/desktop/src/store.ts
  - crates/speclink-protocol/src/query.rs
  - crates/speclink-remote/src/client.rs
  - crates/speclink-server/src/app.rs
  - crates/speclink-server/src/routes.rs
  - crates/speclink-server/tests/import_api.rs
-->

---
### Requirement: Bundle 組裝涵蓋本地 workspace 全集

desktop SHALL 以引擎讀取面把本地 workspace 組裝為 Bundle：active changes 的 meta 與全部 artifacts、canonical specs、live 與 archived 討論、archived changes 的全部文件、workflow config 與共用詞彙文件——與 store 契約的 DocumentId 全集一一對應。任一來源檔解析失敗 SHALL 中止組裝並點名該檔——SHALL NOT 部分遷移。

#### Scenario: 全集往返一致

- **WHEN** 對含各類文件的本地 workspace 組裝 Bundle 並匯入空 scope，再自 server 逐端點讀回
- **THEN** 每一份文件內容與本地原檔一致，無缺漏類別


<!-- @trace
source: local-remote-migration
updated: 2026-07-21
code:
  - Cargo.lock
  - apps/desktop/src-tauri/Cargo.toml
  - apps/desktop/src-tauri/src/lib.rs
  - apps/desktop/src-tauri/src/remote.rs
  - apps/desktop/src-tauri/tests/migration.rs
  - apps/desktop/src/App.tsx
  - apps/desktop/src/__tests__/App.test.tsx
  - apps/desktop/src/__tests__/migrationDialog.test.tsx
  - apps/desktop/src/__tests__/remoteOpen.test.ts
  - apps/desktop/src/__tests__/workspaceChooser.test.tsx
  - apps/desktop/src/adapter/migration.ts
  - apps/desktop/src/components/MigrationDialog.tsx
  - apps/desktop/src/components/RemoteConflictDialog.tsx
  - apps/desktop/src/components/WorkspaceChooser.tsx
  - apps/desktop/src/i18n/messages.ts
  - apps/desktop/src/store.ts
  - crates/speclink-protocol/src/query.rs
  - crates/speclink-remote/src/client.rs
  - crates/speclink-server/src/app.rs
  - crates/speclink-server/src/routes.rs
  - crates/speclink-server/tests/import_api.rs
-->

---
### Requirement: 遷移成功後才轉換本地且一律備份

遷移流程 SHALL 於 import 成功後才觸碰本地：本機規格資料夾 SHALL 改名為帶日期的備份目錄 `<資料夾名>.migrated-<日期>`（例如 `speclink.migrated-2026-11-02`；舊佈局為 `openspec.migrated-2026-11-02`；同名既存則加序號）——SHALL NOT 存在刪除本地內容的路徑；隨後寫入 remote marker 並原地將分頁轉為 remote（checkoutRoot 為該資料夾）。import 失敗（scope 非空、網路、驗證）時本地 SHALL 分毫不動且 UI 原樣呈現錯誤。遷移確認對話 SHALL 指出目標 Project/Repo 與本地備份行為。

#### Scenario: 失敗零副作用

- **WHEN** 遷移於 import 步驟因 scope 非空失敗
- **THEN** 本地規格資料夾原樣存在、無 marker 寫入、分頁維持 local，錯誤原樣呈現

#### Scenario: 成功後資料夾轉為 checkout

- **WHEN** 遷移成功完成
- **THEN** 資料夾含帶日期的備份目錄與 remote marker、原分頁原地轉為 remote 分頁且 checkoutRoot 為該資料夾，CLI 於該資料夾以 remote 模式運作

#### Scenario: 備份目錄依實際資料夾命名

- **WHEN** 規格資料夾為 speclink/ 的專案在 2026-11-02 遷移成功，資料夾內已有 speclink.migrated-2026-11-02
- **THEN** 本機規格資料夾改名為 speclink.migrated-2026-11-02-2


<!-- @trace
source: merge-upstream-0-11-0
updated: 2026-10-29T05:48:54+08:00
-->

---
### Requirement: 遷移入口雙路且不經並存合併

遷移流程 SHALL 可自兩處進入：chooser 本機路徑開到含 openspec/ 的專案時的次要動作，與並存衝突對話。SHALL NOT 提供「上傳合併至非空 scope」的任何路徑——CreateNew 為唯一遷移語意。

#### Scenario: chooser 次要動作進入遷移

- **WHEN** 於 chooser 本機路徑選到含 openspec/ 的專案並選擇遷移動作
- **THEN** 進入同一遷移流程（選 connection 與空 scope、確認、上傳、轉換）

<!-- @trace
source: local-remote-migration
updated: 2026-07-21
code:
  - Cargo.lock
  - apps/desktop/src-tauri/Cargo.toml
  - apps/desktop/src-tauri/src/lib.rs
  - apps/desktop/src-tauri/src/remote.rs
  - apps/desktop/src-tauri/tests/migration.rs
  - apps/desktop/src/App.tsx
  - apps/desktop/src/__tests__/App.test.tsx
  - apps/desktop/src/__tests__/migrationDialog.test.tsx
  - apps/desktop/src/__tests__/remoteOpen.test.ts
  - apps/desktop/src/__tests__/workspaceChooser.test.tsx
  - apps/desktop/src/adapter/migration.ts
  - apps/desktop/src/components/MigrationDialog.tsx
  - apps/desktop/src/components/RemoteConflictDialog.tsx
  - apps/desktop/src/components/WorkspaceChooser.tsx
  - apps/desktop/src/i18n/messages.ts
  - apps/desktop/src/store.ts
  - crates/speclink-protocol/src/query.rs
  - crates/speclink-remote/src/client.rs
  - crates/speclink-server/src/app.rs
  - crates/speclink-server/src/routes.rs
  - crates/speclink-server/tests/import_api.rs
-->

---
### Requirement: 新舊預設資料夾都可遷移到遠端

本機遷移到遠端（含採用伺服器內容）SHALL 接受明寫 spec_dir: openspec 的 Fork 預設專案與上游 speclink/ 佈局；不得僅因 spec_dir 有值就拒絕 openspec。明寫非這兩種預設的自訂路徑，以及兩種規格目錄並存時 SHALL 在寫入遠端前拒絕並說明原因。上傳已封存變更資料檔時 SHALL 以 .meta.yaml 為鍵；舊 .openspec.yaml 讀取相容，兩者並存以 .meta.yaml 為準，不上傳兩份權威資料。

#### Scenario: 明確 Fork 預設可遷移
- **WHEN** spec_dir: openspec 的專案完成遠端匯入
- **THEN** 原目錄備份為 openspec.migrated-<日期>，遠端已封存資料檔鍵為 .meta.yaml，本機轉為遠端專案

#### Scenario: 自訂或並存佈局拒絕
- **WHEN** 專案明寫 spec_dir: docs/specs，或 openspec/ 與 speclink/ 規格目錄並存
- **THEN** 遷移被拒絕並指出原因，本地資料與遠端內容均無寫入

<!-- @trace
source: merge-upstream-0-11-0
updated: 2026-10-29T05:48:54+08:00
-->