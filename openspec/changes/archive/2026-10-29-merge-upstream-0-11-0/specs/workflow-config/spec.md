## MODIFIED Requirements

### Requirement: workflow-config show 動詞
<!-- BEFORE: show 未輸出專案預設 schema，Fork 已提供安全政策與系統語言預設。 -->

CLI SHALL 提供 speclink workflow-config show（旗標 --json 與 --no-color，無 stdin），顯示 openspec/config.yaml 的正典內容：schema（專案預設產出流程名稱，未設定為 spec-driven）、政策五欄（locale、spec_locale、tdd、audit、worktree——未設定的欄位標示為未設定與其預設語意）、context（有無與行數）、rules（各 artifact 節的條數）與 review（預設清單模式與三個嚴重度清單的條數）。show SHALL 顯示正典值，SHALL NOT 應用環境變數的覆寫（有效值的三層解析屬 instructions payload 職責）。帶 --json 時 SHALL 輸出 camelCase payload：schema（字串，未設定為 null）、locale、specLocale（未設定為 null）、tdd、audit、worktree（布林）、context（字串或 null）、rules（artifact id 對規則字串陣列的物件）、review（物件：baseline 為字串 "extend" 或 "replace"，critical、warning、suggestion 為規則字串陣列；檔案未寫 review 區塊時為 baseline "extend" 與三個空陣列）。人眼輸出 SHALL 於 locale 行之前新增 schema 行，值為設定的名稱，未設定時為 unset (spec-driven)；並於 rules 行之後、以與其他行相同的標籤欄寬印出 review 行：baseline 為 extend（含未設定）且三個清單皆空時值為 none，否則值為「<baseline>, critical <n>, warning <n>, suggestion <n>」。fs 模式讀取 openspec/config.yaml；remote 模式 SHALL 經既有連線讀取 server 端 config 文件，人眼與 --json 輸出形狀與 fs 模式一致。config 文件無法解析時 SHALL 沿用既有 fail-closed 行為：非零 exit code、stderr 指出檔案與解析原因。本動詞為周邊設定動詞（與 config 同類），SHALL NOT 進入命令執行層。成功 exit code 0、輸出至 stdout；--no-color 下無 ANSI 色彩。本次安全審查修訂 SHALL 保留 review 行與欄位，刻意於 review 行之後新增 security_verification 摘要行與 JSON securityVerification 物件；此次另刻意新增 schema 行與 JSON schema 欄位；除此之外的各行與既有欄位 SHALL 維持既有位元級輸出。安全物件 SHALL 含 enabled:boolean、scope:string、standards:object、reviews:object、disabledRules:string[]、excludeDirs:string[]，其預設與值域依安全政策區塊需求；四標準皆含 enabled:boolean 與 version:string|null，ASVS 另含 level:string，reviews 含 spec／code／apiBoundary 布林。

#### Scenario: fs 模式顯示正典值

- **WHEN** openspec/config.yaml 含 locale: tw、tdd: true 與 context，執行 speclink workflow-config show
- **THEN** stdout 顯示 locale 為 tw、tdd 開啟、audit 未設定（預設關閉）、worktree 未設定（預設關閉）、context 存在；exit code 0

#### Scenario: --json payload 形狀

- **WHEN** 執行 speclink workflow-config show --json
- **THEN** stdout 為 JSON，欄位 schema、locale、specLocale、tdd、audit、worktree、context、rules、review 一律 camelCase；未設定的 specLocale 為 null、未設定的 tdd 為 false、未設定的 worktree 為 false、未寫 review 區塊時 review 為 baseline "extend" 與 critical、warning、suggestion 三個空陣列

#### Scenario: show 不應用環境變數覆寫

- **WHEN** 設定 SPECLINK_TDD=false 且 openspec/config.yaml 含 tdd: true，執行 speclink workflow-config show --json
- **THEN** payload 的 tdd 為 true（正典值，非解析後有效值）

#### Scenario: remote 模式輸出形狀一致

- **WHEN** 於 remote 綁定的 workspace 執行 speclink workflow-config show --json
- **THEN** stdout 的 JSON 欄位名與 fs 模式完全一致，值來自 server 端 config 文件

#### Scenario: 壞 config fail-closed

- **WHEN** openspec/config.yaml 含 YAML 語法錯誤，執行 speclink workflow-config show
- **THEN** exit code 非 0，stderr 指出該檔與解析原因，stdout 無 payload

#### Scenario: 人眼輸出的 review 行

- **WHEN** 執行 speclink workflow-config show --no-color
- **THEN** stdout 於 rules 行之後恰有一行 review，標籤欄寬與 rules 行相同，值依 review 區塊內容而定；除新增 schema 行與 security_verification 摘要行外，其餘各行與本次修訂前逐位元一致

##### Example: review 行的值

| config.yaml 的 review 區塊 | review 行的值 |
| -------------------------- | ------------- |
| 未寫 review | none |
| critical 兩條、warning 一條、suggestion 一條，未寫 baseline | extend, critical 2, warning 1, suggestion 1 |
| baseline: replace、三個清單皆空 | replace, critical 0, warning 0, suggestion 0 |

#### Scenario: --json 帶出 review 內容

- **WHEN** openspec/config.yaml 含 review 區塊 baseline: replace 與 critical 一條「所有 SQL 一律用參數化查詢」，執行 speclink workflow-config show --json
- **THEN** payload 的 review 為 {"baseline":"replace","critical":["所有 SQL 一律用參數化查詢"],"warning":[],"suggestion":[]}；exit code 0

未設定 locale 與 spec_locale 的人眼標示 SHALL 為 unset (system language)；JSON 仍為 null，SHALL NOT 把偵測值寫回設定。官方新增的 review 行與 JSON 欄位 SHALL 保留。

#### Scenario: 舊設定與安全摘要

- **WHEN** 未寫安全政策，執行 workflow-config show --json 及 --no-color 人眼輸出
- **THEN** JSON securityVerification.enabled=false、scope=off、四標準停用、reviews 三項 true、兩清單空；人眼在 review 後有 security_verification: disabled，舊行內容不變

#### Scenario: 僅一個標準且保留版本

- **WHEN** enabled=true、只啟用 Top 10 2025、ASVS 停用，執行 show --json
- **THEN** securityVerification.standards.owaspTop10 為 enabled=true、version="2025"，ASVS 停用不影響 Top 10，Local／Remote JSON 同形

#### Scenario: 顯示專案預設 schema

- **WHEN** 執行 speclink workflow-config show --no-color 與 speclink workflow-config show --json
- **THEN** 人眼輸出於 locale 行之前恰有一行 schema，標籤欄寬與 locale 行相同；--json 的 schema 欄位值依 config 的 schema 鍵而定；exit code 0

##### Example: schema 的值

| config.yaml 的 schema 鍵 | 人眼 schema 行的值 | --json 的 schema |
| ------------------------ | ------------------ | ---------------- |
| 未寫 schema | unset (spec-driven) | null |
| schema: spec-driven | spec-driven | "spec-driven" |
| schema: my-flow | my-flow | "my-flow" |
