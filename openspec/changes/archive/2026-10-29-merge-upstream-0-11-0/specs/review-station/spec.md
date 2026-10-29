## ADDED Requirements

### Requirement: 蓋章記錄審查輪數

`speclink review stamp <change>` 守門通過時，系統 SHALL 在寫入其他章欄位的同一次原子寫入中，把 `reviewed_rounds` 寫入該 change 的 `.meta.yaml`：值為原有 `reviewed_rounds`（缺席視為 0）加上本次蓋章的審查工單回合數，檔內 SHALL 只有一行 `reviewed_rounds`。帶 `--accept` 的蓋章 SHALL 同樣記錄。守門失敗時 SHALL NOT 改動 `reviewed_rounds`；放棄審查（只刪工單、未蓋章）SHALL NOT 改動 `reviewed_rounds`。原有 `reviewed_rounds` 不是非負整數時，metadata 視為無法解析，蓋章 SHALL 以非零 exit code 拒絕且零寫入。

#### Scenario: 首次蓋章記錄回合數

- **WHEN** change 的審查工單有 3 個回合、寫碼任務全數完成、末輪零必修 findings，執行 `speclink review stamp <change>`
- **THEN** exit code 為 0，`.meta.yaml` 含 `reviewed_rounds: 3` 與 `reviewed_at` 等章欄位，`review.md` 已刪除

#### Scenario: 章失效後重新蓋章累加輪數

- **WHEN** change 的 `.meta.yaml` 已有 `reviewed_rounds: 3`，章失效後重開的審查工單有 2 個回合，再次執行 `speclink review stamp <change>` 且守門通過
- **THEN** exit code 為 0，`.meta.yaml` 只有一行 `reviewed_rounds`，值為 5

#### Scenario: 守門失敗不改輪數

- **WHEN** 審查工單末輪有一條未解的 WARNING finding，執行 `speclink review stamp <change>` 且未帶 `--accept`
- **THEN** exit code 非零，`.meta.yaml` 的 `reviewed_rounds` 與其他欄位皆不變，`review.md` 仍存在

#### Scenario: 原有輪數不是整數時拒絕

- **WHEN** change 的 `.meta.yaml` 含 `reviewed_rounds: many`，執行 `speclink review stamp <change>`
- **THEN** exit code 非零，stderr 說明該 change 的 metadata 無法解析，`.meta.yaml` 與 `review.md` 皆不變

## MODIFIED Requirements

### Requirement: CLI 清單輸出的相容性釘住
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

`speclink list --json` 的輸出 SHALL 不因 metadata 含任何 reviewed 欄位而改變形狀：含全套 reviewed 欄位的 change 與不含者 SHALL 序列化出同形（位元級同構的欄位集合）。

#### Scenario: 帶章的 change 不外洩新欄位

- **WHEN** 某 change 的 `.meta.yaml` 含全套 reviewed 欄位時執行 `speclink list --json`
- **THEN** 該 change 的 JSON 項目與無 reviewed 欄位的 change 具有相同的欄位集合

### Requirement: 放棄審查
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

系統 SHALL 提供 `speclink review discard <change>`：刪除工單、不寫任何 metadata。

#### Scenario: 放棄既有工單

- **WHEN** 對有工單的 change 執行 `review discard`
- **THEN** exit code 0，`review.md` 不存在，`.meta.yaml` 不變

#### Scenario: 無工單可放棄

- **WHEN** 對無工單的 change 執行 `review discard`
- **THEN** exit code 非零，stderr 說明無工單

### Requirement: 蓋章守門與蓋章效果
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

系統 SHALL 提供 `speclink review stamp <change> [--accept]`。守門條件:(1) change 的寫碼任務全數完成(非 `[M]` 任務全數勾選;手動測試任務不計,見 manual-task-marker 的寫碼任務全完成預測子);(2) 工單末輪零未解必修 findings。必修 SHALL 以嚴重度界定:CRITICAL 與 WARNING 級為必修、擋乾淨蓋章;SUGGESTION 級 SHALL NOT 擋章——末輪僅含 SUGGESTION 級 findings 時蓋章照常放行。`--accept` SHALL 僅豁免條件 (2) 的必修部分。守門通過時系統 SHALL 於同一原子寫入內:將 `reviewed_at`/`reviewed_by`/`reviewed_with`/`reviewed_tasks_total`(蓋章時全任務總數,含 `[M]` 任務)/`reviewed_scope`(指紋清單)寫入該 change 的 `.meta.yaml`,並刪除 `review.md`。不得出現「章已寫入而工單仍存在」的中間狀態。活動工單刪除前 SHALL 將完整輪次、當時規則、接受情形、操作者及章指紋保存於checks/review/<YYYYMMDDHHmmss><ULID>.md，時間為審查開始的UTC時間，ULID仍為索引中的recordId，checks/index.json SHALL 登錄完整relativePath。舊checks/review/<ULID>.md紀錄 SHALL 仍可讀且不重新命名；完成紀錄不視為未結工單，蓋章後及保留紀錄的封存後仍可查詢。這是刻意新增的檔案效果，原成功stdout及JSON SHALL 維持；歷史、索引或章寫入失敗 SHALL 不回成功，本機journal恢復原工單／metadata／索引、Remote交易不提交；恢復未完成時 SHALL 明示history_recovery_required。

#### Scenario: 寫碼任務未全完成即拒絕

- **WHEN** change 的寫碼任務為 4/5 完成時執行 `review stamp`
- **THEN** exit code 非零,stderr 說明寫碼任務未全數完成並列計數(4/5),metadata 與工單皆不變

#### Scenario: 僅餘手動任務可蓋章

- **WHEN** change 的寫碼任務 4/4 全勾、一個 `[M]` 任務未勾,工單末輪 findings 為空時執行 `review stamp`
- **THEN** exit code 0,五個 reviewed 欄位寫入且 `review.md` 刪除

#### Scenario: 末輪有未解 findings 且未帶 --accept

- **WHEN** 工單末輪含至少一筆 CRITICAL 或 WARNING 級 findings 且執行 `review stamp`(無 `--accept`)
- **THEN** exit code 非零,stderr 點名未解必修數並提示 `--accept` 或先修正重審

#### Scenario: 僅 SUGGESTION 的末輪乾淨蓋章

- **WHEN** 寫碼任務全數完成且工單末輪僅含 SUGGESTION 級 findings 時執行 `review stamp`(無 `--accept`)
- **THEN** exit code 0,五個 reviewed 欄位寫入且 `review.md` 刪除;兩種模式均可從checks/review的完成紀錄查到SUGGESTION原文

#### Scenario: 帶保留蓋章

- **WHEN** 工單末輪含必修 findings 且執行 `review stamp --accept`
- **THEN** exit code 0,章寫入且工單刪除

#### Scenario: 乾淨蓋章

- **WHEN** 任務 5/5 完成且工單末輪 findings 為空時執行 `review stamp`
- **THEN** exit code 0,`.meta.yaml` 含五個 reviewed 欄位且 `review.md` 不存在

##### Example: 蓋章寫入的任務錨

- **GIVEN** change 有 5 個任務,其中 4 個寫碼任務全數勾選、1 個 `[M]` 任務未勾,工單 Round 2 的 findings 為空
- **WHEN** `review stamp` 成功
- **THEN** `.meta.yaml` 內 `reviewed_tasks_total` 為 5(全任務總數,含未勾的 `[M]` 任務)

#### Scenario: 歷史寫入失敗保全工單

- **WHEN** 蓋章時完成紀錄、索引或metadata寫入失敗
- **THEN** 命令非零退出，不顯示完成；原工單與章恢復原值，若恢復失敗則明示history_recovery_required且保留復原資料

#### Scenario: 審查紀錄檔名保留開始時間

- **WHEN** 審查於 2026-10-22 15:37:33 UTC 開始並成功蓋章，索引登錄該筆紀錄
- **THEN** 紀錄檔名以 `20261022153733` 開頭並接該紀錄 ULID，索引的 `recordId` 維持 ULID 且 `relativePath` 指向新檔名
