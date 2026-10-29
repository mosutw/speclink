## ADDED Requirements

### Requirement: change schema 動詞切換變更的 schema

`speclink change schema <name> <schema> [--json]` SHALL 把作用中變更 `.meta.yaml` 的 `schema:` 值改為 `<schema>`，其餘位元組逐字保留、每行沿用自身行尾；metadata 沒有 `schema:` 行時 SHALL 新增一行；`schema:` 不是第 0 欄的單行 `schema: <值>`（例如 `schema : x`，或值寫在縮排續行）時 SHALL 拒絕且零寫入，訊息提示先改成單行的 `schema: <name>`。守門 SHALL 依序：`<name>` 不是作用中變更（以名稱逐字比對）即錯；`<name>` 的 metadata 無法解析即拒絕；`<schema>` 不是小寫 kebab-case 名稱（含路徑分隔符、`..`、空白、換行、大寫字母，或為空字串）即拒絕，在解析之前判定，訊息沿用 `schema init`／`schema fork` 的名稱不合法訊息；`<schema>` 在專案層、user 層與內建都解析不到即錯（沿用現行的 schema 找不到訊息）；`<schema>` 沒有 id 為 `specs` 的 artifact 而變更含 delta spec 時即錯，訊息列出這些 capability；變更有未結的驗證工單時即錯，訊息提示先執行 `speclink verify discard <name>`。任一守門失敗 SHALL 以非零 exit code 結束、stderr 一行原因且零寫入。`<schema>` 與現值相同時 SHALL 成功且不寫檔。切換寫入時，若 metadata 帶著驗證章，同一次寫入 SHALL 移除 `verified_at`、`verified_by`、`verified_with`、`verified_tasks_total` 與 `verified_scope`，`verified_rounds` 與審查章欄位 SHALL 保留。成功時 stdout SHALL 為一行 `✓ <name> schema: <舊值> → <新值>`（未寫檔時為 `✓ <name> schema: <值> (unchanged)`，移除驗證章時另一行 `  verify stamp cleared — re-run verify under <新值>`）；`--json` 時 stdout SHALL 為 `{ "change": string, "schema": string, "previous": string, "changed": boolean }`。在主 checkout 且 worktree 政策開啟時，`<name>` 已映射到 worktree 者 SHALL 寫入 worktree 副本，主副本不變。指令在本機與遠端模式 SHALL 同語意、同輸出形狀。

#### Scenario: 切到不改規格
- **WHEN** 作用中變更 tidy 的 `.meta.yaml` 為 `schema: spec-driven`、沒有 delta spec，執行 `speclink change schema tidy no-spec-change`
- **THEN** exit code 為 0，`schema:` 行變為 `schema: no-spec-change`，其餘行逐位元不變，stdout 為 `✓ tidy schema: spec-driven → no-spec-change`

#### Scenario: 切回 spec-driven 並輸出 JSON
- **WHEN** tidy 的 schema 為 no-spec-change，執行 `speclink change schema tidy spec-driven --json`
- **THEN** exit code 為 0，stdout 是 JSON（排版同其他 `--json` 輸出），內容為 `{"change":"tidy","schema":"spec-driven","previous":"no-spec-change","changed":true}`

#### Scenario: 有 delta 時拒絕切到不改規格
- **WHEN** 變更 c 含 `specs/auth/spec.md` 的 delta，執行 `speclink change schema c no-spec-change`
- **THEN** exit code 非零，stderr 說明 c 含 delta spec（列出 auth）而 no-spec-change 沒有 specs，`.meta.yaml` 逐位元不變

#### Scenario: 解析不到的 schema 拒絕
- **WHEN** 執行 `speclink change schema c no-such-flow`
- **THEN** exit code 非零，stderr 為現行的 schema 找不到訊息，`.meta.yaml` 逐位元不變

#### Scenario: 不合法的 schema 名稱在解析前拒絕
- **WHEN** 執行 `speclink change schema c ../evil` 或 `speclink change schema c Spec-Driven`
- **THEN** exit code 非零，stderr 為 schema 名稱不合法的訊息（`must be lowercase kebab-case`），`.meta.yaml` 逐位元不變

#### Scenario: 未結驗證工單時拒絕
- **WHEN** 變更 c 有未結的驗證工單，執行 `speclink change schema c spec-driven`
- **THEN** exit code 非零，stderr 提示先執行 `speclink verify discard c`，`.meta.yaml` 與驗證工單皆不變

#### Scenario: 切換時移除驗證章、保留審查章與輪數
- **WHEN** 變更 c 的 schema 為 no-spec-change，`.meta.yaml` 帶著審查章、驗證章與 `verified_rounds: 2`，沒有未結工單，執行 `speclink change schema c spec-driven`
- **THEN** exit code 為 0，`verified_at`、`verified_by`、`verified_with`、`verified_tasks_total` 與 `verified_scope` 已移除，`verified_rounds: 2` 與所有 `reviewed_*` 欄位不變，stdout 第二行說明驗證章已清除

#### Scenario: 同一個 schema 不寫檔
- **WHEN** c 的 schema 已是 spec-driven，執行 `speclink change schema c spec-driven`
- **THEN** exit code 為 0，stdout 為 `✓ c schema: spec-driven (unchanged)`，`.meta.yaml` 逐位元不變

#### Scenario: 變更不存在
- **WHEN** 執行 `speclink change schema ghost spec-driven`，沒有名為 ghost 的作用中變更
- **THEN** exit code 非零，stderr 說明找不到 ghost，沒有任何檔案被寫入

#### Scenario: 目標變更在 worktree 內時寫入副本
- **WHEN** 主 checkout 且 worktree 政策開啟，tidy 已映射到 worktree，執行 `speclink change schema tidy no-spec-change`
- **THEN** worktree 副本的 `.meta.yaml` 為 `schema: no-spec-change`，主副本不變

### Requirement: no-spec-change 內建 schema 的組成

內建 schema `no-spec-change`（中文「不改規格」）SHALL 給不改任何正式規格承諾行為的變更使用。它的 artifact SHALL 為 proposal（無前置）、design（前置為 proposal，選用）與 tasks（前置為 proposal），applyRequires SHALL 為 tasks，SHALL NOT 含 specs artifact。proposal 的 instruction SHALL 要求 Why 寫明「為什麼這個變更不改正式規格承諾的行為」並列出查過的正式規格；proposal 的 template SHALL 含 Why、What Changes、Non-Goals（選用）與 Impact，SHALL NOT 含 Capabilities。tasks 的 instruction SHALL 要求最後一個任務以既有測試確認相關正式規格的行為未變，SHALL NOT 要求任務引用 requirement 名稱。design 的 instruction 與 template SHALL 與 spec-driven 相同。schema 沒有 specs artifact 的變更執行 `speclink validate` 時 SHALL NOT 產生「No delta specs found」警告；schema 含 specs artifact 的變更維持現行警告。封存 `no-spec-change` 變更 SHALL 不需要 `--skip-specs`，正式規格不變。內建 schema 在本機與遠端工作區 SHALL 皆可解析。

#### Scenario: 建立不改規格變更
- **WHEN** 執行 `speclink new change tidy --schema no-spec-change`，接著執行 `speclink status --change tidy --json`
- **THEN** 兩者 exit code 為 0；`schemaName` 為 `no-spec-change`，`artifacts` 依序為 proposal、design、tasks，`applyRequires` 為 `["tasks"]`，沒有 specs

#### Scenario: proposal 範本不含 Capabilities
- **WHEN** 對 schema 為 no-spec-change 的變更執行 `speclink instructions proposal --change tidy --json`
- **THEN** `template` 含 `## Why`、`## What Changes`、`## Impact`，不含 `## Capabilities`；`instruction` 要求 Why 說明為什麼不改正式規格承諾的行為並列出查過的正式規格

#### Scenario: validate 不警告缺少 delta
- **WHEN** 對沒有 delta spec 的 no-spec-change 變更執行 `speclink validate tidy`，再對沒有 delta spec 的 spec-driven 變更執行 `speclink validate other`
- **THEN** `tidy` 的結果為有效且沒有「No delta specs found」警告；`other` 照現行輸出該警告

#### Scenario: 封存不需要 skip-specs
- **WHEN** no-spec-change 變更的任務全數完成，執行 `speclink archive tidy`（不帶 `--skip-specs`）
- **THEN** exit code 為 0，變更移入封存目錄，`openspec/specs/` 下的正式規格沒有任何變動

#### Scenario: 專案層同名 schema 遮蔽內建
- **WHEN** 專案的 `openspec/schemas/no-spec-change/schema.yaml` 存在且可載入，執行 `speclink schema which no-spec-change`
- **THEN** exit code 為 0，解析結果指向專案層的 schema.yaml、來源層級為 project；`speclink schema which --all` 同時列出被遮蔽的內建位置

### Requirement: schema 沒有 specs 產出時拒絕寫入 delta spec
對 schema 沒有 id 為 `specs` 的 artifact 的變更執行 `speclink new artifact spec <capability> --change <name>`（含 `--stdin` 與 `--force`）時，系統 SHALL 以非零 exit code 拒絕、零寫入，stderr SHALL 說明該變更的 schema 沒有 specs，並提示 `speclink change schema <name> spec-driven`。schema 含 specs artifact 的變更 SHALL 維持現行行為。

#### Scenario: 不改規格變更寫 delta 被拒
- **WHEN** 變更 tidy 的 schema 為 no-spec-change，執行 `speclink new artifact spec auth --change tidy --stdin` 並由 stdin 提供合法的 delta 內容
- **THEN** exit code 非零，stderr 說明 tidy 使用 no-spec-change、沒有 specs，並提示 `speclink change schema tidy spec-driven`；tidy 目錄下沒有出現 specs/

#### Scenario: spec-driven 變更照常寫入
- **WHEN** 變更 c 的 schema 為 spec-driven，執行 `speclink new artifact spec auth --change c --stdin` 並提供合法的 delta 內容（auth 為正式規格已收錄的 capability）
- **THEN** exit code 為 0，`specs/auth/spec.md` 建立

## MODIFIED Requirements

### Requirement: 單一正典載入
<!-- BEFORE: 只有 spec-driven 內建產出流程。 -->
每個內建 schema SHALL 只由一份內嵌的正典 YAML 定義：spec-driven 為 crates/engine/speclink-core/assets/schema/spec-driven/fork.schema.yaml，no-spec-change 為 crates/engine/speclink-core/assets/schema/no-spec-change/fork.schema.yaml。內建名稱 SHALL 只有這兩個；名稱解析順序維持 project → user → 內建。載入時 SHALL 走與自訂 schema 相同的解析與驗證路徑；template 內容由內嵌資產依 template 檔名附掛。引擎 SHALL NOT 另持手寫的內建 schema 定義。schema fork <內建名稱> 產出的 schema.yaml SHALL 與該內建的正典 YAML 逐位元組相同。內建 schema 顯示的 description SHALL 為正典 YAML 的字面值。

#### Scenario: 內建與 fork 同源
- **WHEN** 使用者執行 speclink schema fork spec-driven，以及 speclink schema fork no-spec-change
- **THEN** 兩份產出的 schema.yaml 各自與對應的內嵌正典 YAML 逐位元組相同

#### Scenario: 列表 description 來自正典
- **WHEN** 使用者執行 speclink schemas
- **THEN** spec-driven 的 description 顯示正典 YAML 的字面「Default OpenSpec workflow - proposal → specs → tasks (design optional)」，no-spec-change 的 description 顯示其正典 YAML 的字面「No-spec-change workflow - proposal → tasks (design optional), for changes that alter no promised behavior」

#### Scenario: template 內容附掛
- **WHEN** 引擎載入內建 schema
- **THEN** spec-driven 的四個 artifact 與 no-spec-change 的三個 artifact 各自帶有非空的 template 內容，與對應的內嵌 template 資產逐字相同
