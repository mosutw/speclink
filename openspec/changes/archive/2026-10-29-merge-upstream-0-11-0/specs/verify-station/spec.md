## ADDED Requirements

### Requirement: 蓋章記錄驗證輪數

`speclink verify stamp <change>` 守門通過時，系統 SHALL 在寫入其他章欄位的同一次原子寫入中，把 `verified_rounds` 寫入該 change 的 `.meta.yaml`：值為原有 `verified_rounds`（缺席視為 0）加上本次蓋章的驗證工單回合數，檔內 SHALL 只有一行 `verified_rounds`。帶 `--accept` 的蓋章 SHALL 同樣記錄。守門失敗時 SHALL NOT 改動 `verified_rounds`；放棄驗證（只刪工單、未蓋章）SHALL NOT 改動 `verified_rounds`。原有 `verified_rounds` 不是非負整數時，metadata 視為無法解析，蓋章 SHALL 以非零 exit code 拒絕且零寫入。

#### Scenario: 首次驗證蓋章記錄回合數

- **WHEN** change 的驗證工單有 2 個回合、寫碼任務全數完成、末輪零必修 findings，執行 `speclink verify stamp <change>`
- **THEN** exit code 為 0，`.meta.yaml` 含 `verified_rounds: 2` 與 `verified_at` 等章欄位，`verify.md` 已刪除

#### Scenario: 驗證章失效後重新蓋章累加輪數

- **WHEN** change 的 `.meta.yaml` 已有 `verified_rounds: 2`，章失效後重開的驗證工單有 3 個回合，再次執行 `speclink verify stamp <change>` 且守門通過
- **THEN** exit code 為 0，`.meta.yaml` 只有一行 `verified_rounds`，值為 5

#### Scenario: 驗證守門失敗不改輪數

- **WHEN** 驗證工單末輪有一條未解的 CRITICAL finding，執行 `speclink verify stamp <change>` 且未帶 `--accept`
- **THEN** exit code 非零，`.meta.yaml` 的 `verified_rounds` 與其他欄位皆不變，`verify.md` 仍存在

#### Scenario: 原有驗證輪數不是整數時拒絕

- **WHEN** change 的 `.meta.yaml` 含 `verified_rounds: -1`，執行 `speclink verify stamp <change>`
- **THEN** exit code 非零，stderr 說明該 change 的 metadata 無法解析，`.meta.yaml` 與 `verify.md` 皆不變

## MODIFIED Requirements

### Requirement: CLI 清單輸出的驗證欄位釘住
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

`speclink list --json` 的輸出 SHALL 不因 metadata 含任何 verified 欄位而改變形狀，與審查欄位的釘住規則同構。

#### Scenario: 帶驗證章的 change 不外洩新欄位

- **WHEN** 某 change 的 `.meta.yaml` 含全套 verified 欄位時執行 `speclink list --json`
- **THEN** 該 change 的 JSON 項目與無 verified 欄位的 change 具有相同的欄位集合

### Requirement: 放棄驗證
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

系統 SHALL 提供 `speclink verify discard <change>`：刪除驗證工單、不寫任何 metadata，並依「驗證 frozen scope 與續輪 snapshot」清理 verify snapshots；無工單時非零 exit code。

#### Scenario: 放棄既有工單

- **WHEN** 對有驗證工單的 change 執行 `verify discard`
- **THEN** exit code 0，`verify.md` 不存在，`.meta.yaml` 不變

### Requirement: 驗證蓋章守門與蓋章效果
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

系統 SHALL 提供 `speclink verify stamp <change> [--accept]`,守門與審查站同一條:寫碼任務全數完成(manual-task-marker 預測子)＋工單末輪零未解必修 findings。必修 SHALL 以嚴重度界定:CRITICAL 與 WARNING 級為必修、擋乾淨蓋章;SUGGESTION 級 SHALL NOT 擋章——末輪僅含 SUGGESTION 級 findings 時蓋章照常放行。`--accept` SHALL 僅豁免既有驗證工單的必修條件。安全政策enabled及required_for_acceptance皆true時 SHALL 另依安全驗收判定通過後才蓋章；--accept不得豁免缺報告、過期、必要未驗證或未處理中高風險。通過時 SHALL 於同一原子寫入內:將 `verified_at`/`verified_by`/`verified_with`/`verified_tasks_total`(蓋章時全任務總數,含 `[M]` 任務)/`verified_scope` 寫入 `.meta.yaml` 並刪除 `verify.md`,不得出現「章已寫而工單仍在」的中間狀態;canonical mutation 成功後 SHALL 依「驗證 frozen scope 與續輪 snapshot」清理 verify snapshots。活動工單刪除前 SHALL 將完整輪次、當時規則、接受情形、操作者及章指紋保存於checks/verify/<YYYYMMDDHHmmss><ULID>.md，時間為驗證開始的UTC時間，ULID仍為索引中的recordId，checks/index.json SHALL 登錄完整relativePath。舊checks/verify/<ULID>.md紀錄 SHALL 仍可讀且不重新命名；完成紀錄不視為未結工單，蓋章後及保留紀錄的封存後仍可查詢。這是刻意新增的檔案效果，原成功stdout及JSON SHALL 維持；歷史、索引或章寫入失敗 SHALL 不回成功，本機journal恢復原工單／metadata／索引、Remote交易不提交；恢復未完成時 SHALL 明示history_recovery_required。

#### Scenario: 寫碼任務未完成即拒絕蓋章

- **WHEN** 對寫碼任務 4/5 的 change 執行 `verify stamp`
- **THEN** exit code 非零,stderr 說明寫碼任務未全數完成,metadata 與工單皆不變

#### Scenario: 僅餘手動任務可蓋章

- **WHEN** 寫碼任務全勾、一個 `[M]` 任務未勾且驗證工單末輪 findings 為空時執行 `verify stamp`
- **THEN** exit code 0,五個 verified 欄位寫入且 `verify.md` 刪除

#### Scenario: 末輪有未解 findings 且未帶 --accept

- **WHEN** 驗證工單末輪含至少一筆 CRITICAL 或 WARNING 級 findings 時執行 `verify stamp`
- **THEN** exit code 非零,stderr 點名未解必修數並提示 `--accept` 或先修正重驗

#### Scenario: 僅 SUGGESTION 的末輪乾淨蓋章

- **WHEN** 寫碼任務全數完成且驗證工單末輪僅含 SUGGESTION 級 findings 時執行 `verify stamp`(無 `--accept`)
- **THEN** exit code 0,五個 verified 欄位寫入且 `verify.md` 刪除;兩種模式均可從checks/verify的完成紀錄查到SUGGESTION原文

#### Scenario: 乾淨蓋章

- **WHEN** 寫碼任務全數完成且末輪 findings 為空時執行 `verify stamp`
- **THEN** exit code 0,`.meta.yaml` 含五個 verified 欄位且 `verify.md` 不存在

##### Example: 蓋章寫入的任務錨

- **GIVEN** change 有 8 個任務,其中 7 個寫碼任務全數勾選、1 個 `[M]` 任務未勾,驗證工單 Round 1 的 findings 為空
- **WHEN** `verify stamp` 成功
- **THEN** `.meta.yaml` 內 `verified_tasks_total` 為 8(全任務總數)

#### Scenario: 必修驗收不能用一般接受略過

- **WHEN** required安全政策生效且安全報告缺失、過期或有未處理HIGH，執行verify stamp --accept
- **THEN** exit1，stderr列安全阻擋原因；活動工單、章、歷史索引皆不變


#### Scenario: 歷史寫入失敗保全工單

- **WHEN** 蓋章時完成紀錄、索引或metadata寫入失敗
- **THEN** 命令非零退出，不顯示完成；原工單與章恢復原值，若恢復失敗則明示history_recovery_required且保留復原資料

#### Scenario: 驗證紀錄檔名保留開始時間

- **WHEN** 驗證於 2026-10-22 15:37:33 UTC 開始並成功蓋章，索引登錄該筆紀錄
- **THEN** 紀錄檔名以 `20261022153733` 開頭並接該紀錄 ULID，索引的 `recordId` 維持 ULID 且 `relativePath` 指向新檔名
