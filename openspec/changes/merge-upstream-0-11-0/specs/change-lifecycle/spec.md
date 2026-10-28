## ADDED Requirements

### Requirement: change metadata 時間欄位寫入時戳、對外輸出日期

系統新寫入 change metadata（`.meta.yaml`）的 `created`、`started_at`、`claimed_at`、`reviewed_at`、`verified_at` 與 `archived_at` SHALL 為帶本機時區偏移量、精確到秒的 RFC 3339 時戳（例 `2026-10-21T14:05:00+08:00`）。讀取 SHALL 同時接受純日期（`YYYY-MM-DD`）與 RFC 3339 時戳；既有的純日期記錄 SHALL NOT 被改寫。凡目前輸出這些欄位的地方——CLI 人眼輸出與 `--json`（例如 `speclink show`、`speclink list --json`、drift 報告）、server 回應與 desktop 協定——SHALL 只輸出日期部分，也就是時戳的前 10 個字元（寫入者時區的日期），所以同一天寫入的純日期記錄與時戳記錄產生位元級相同的輸出。依建立日期計算的天數（變更過期判定）SHALL 取日期部分。既不是純日期也不是 RFC 3339 時戳的值 SHALL 維持現行行為：原樣輸出，且不計算過期天數。

#### Scenario: 新變更寫入時戳

- **WHEN** 執行 `speclink new change demo`
- **THEN** exit code 為 0，`demo` 的 `.meta.yaml` 中 `created:` 的值符合 RFC 3339、帶時區偏移量、精確到秒

#### Scenario: 對外輸出只給日期部分

- **WHEN** change `demo` 的 `.meta.yaml` 含 `created: 2026-10-21T14:05:00+08:00`，執行 `speclink show demo` 與 `speclink show demo --json`
- **THEN** 兩者 exit code 為 0；人眼輸出為 `Created: 2026-10-21`，`--json` 的 `created` 為字串 `"2026-10-21"`

#### Scenario: 純日期記錄的輸出不變

- **WHEN** change 的 `.meta.yaml` 含 `created: 2026-07-29`，執行 `speclink show <change> --json` 與 `speclink list --json`
- **THEN** 輸出與本變更之前的版本位元級一致，exit code 為 0，沒有警告

#### Scenario: 無法辨識的時間值維持現行行為

- **WHEN** change 的 `.meta.yaml` 含 `created: someday`，執行 `speclink show <change>`
- **THEN** exit code 為 0，人眼輸出為 `Created: someday`，該 change 不計算過期天數

### Requirement: 封存寫入 archived_at

`speclink archive <change>` 成功且該 change 帶 metadata（`.meta.yaml`）時，封存目錄的 `.meta.yaml` SHALL 含 `archived_at`，值為 RFC 3339 時戳，且與封存目錄名的日期前綴、正式規格 `@trace` 的 `updated` 取自同一個「現在」。寫入順序 SHALL 為：守門全數通過後、把 change 移入封存目錄之前，先把 `archived_at` 寫入該 change 的 metadata（已有該鍵時取代，檔內只留一行）；移動失敗時，該 change 留在進行中並帶著 `archived_at`，之後重試封存成功時 SHALL 以新的「現在」取代。封存守門拒絕時 SHALL NOT 寫入 `archived_at`。沒有 metadata 的舊式 change 照常封存，SHALL NOT 為了 `archived_at` 新建 metadata。

#### Scenario: 成功封存帶封存時戳

- **WHEN** 在 2026-10-21 對守門全數通過的 change `demo` 執行 `speclink archive demo`
- **THEN** exit code 為 0，封存目錄名以 `2026-10-21-` 開頭，其 `.meta.yaml` 含一行 `archived_at`，值為 `2026-10-21T` 開頭、帶時區偏移量的 RFC 3339 時戳

#### Scenario: 封存守門拒絕不寫封存時戳

- **WHEN** change 的審查章已失效，執行 `speclink archive <change>`
- **THEN** exit code 非零，change 留在進行中，其 `.meta.yaml` 沒有 `archived_at`

#### Scenario: 重試封存只留一行封存時戳

- **WHEN** change 的 `.meta.yaml` 已帶著前一次失敗封存留下的 `archived_at`，再次執行 `speclink archive <change>` 且成功
- **THEN** exit code 為 0，封存目錄的 `.meta.yaml` 只有一行 `archived_at`，值為這次封存的時戳

## MODIFIED Requirements

### Requirement: in-progress 標記可自 change meta 移除(零工作痕跡守門)
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

執行 speclink in-progress remove 某 change 後,系統 SHALL 僅在該 change 為零工作痕跡——tasks.md 的已勾任務數為 0(tasks.md 不存在視為 0)且 touched 記錄的 v1 與 v2 兩清單皆空(記錄檔不存在視為空)——時,自該 change 的 .meta.yaml 移除 started_at、started_by、started_with 三欄位;其餘欄位與內容 SHALL 逐字保留,SHALL NOT 重新序列化整份文件。子指令不接旗標、不讀 stdin。成功移除 SHALL exit 0 並於 stdout 印出移除確認;對未開工(無任何 started_* 欄位)的 change SHALL 冪等成功——exit 0、零檔案寫入。已勾任務數 > 0 或 touched 記錄非空時 SHALL 拒絕:exit 非 0,stderr SHALL 列出已勾任務數與 touched 記錄的檔案清單(兩清單聯集、去重)及出路說明(已勾任務可取消後重試;touched 需以人工或 agent 判斷處理),且 SHALL NOT 修改任何檔案。指名不存在的 change SHALL exit 非 0 並於 stderr 報找不到——此行為與 in-progress add 對未知名稱的靜默成功刻意不對稱(add 受遷移前 parity 凍結,remove 為新動詞、修正動作打錯名字必須明確報錯)。change meta 損毀無法解析時 SHALL fail-closed 報錯且不動任何檔案。本指令 SHALL NOT 提供任何強制旗標或機械清理已勾任務/touched 記錄的路徑,SHALL NOT 影響 speclink in-progress add 的既有輸出與行為。

#### Scenario: 零痕跡的進行中變更成功退回

- **WHEN** 一個 change 曾執行 in-progress add(meta 含 started_at 與 started_by),tasks.md 無任何已勾任務且無 touched 記錄,對其執行 speclink in-progress remove
- **THEN** exit 0,stdout 印出移除確認;meta 的 started_at、started_by、started_with 消失,schema、created_*、from_discussion 等其餘欄位逐字不變

#### Scenario: 已勾任務時拒絕退回

- **WHEN** 一個進行中的 change 其 tasks.md 有 2 個已勾任務,對其執行 speclink in-progress remove
- **THEN** exit 非 0,stderr 說明有 2 個已勾任務並提示取消勾選後可重試;meta 與 tasks.md 皆不變

#### Scenario: touched 記錄非空時拒絕退回並列出檔案

- **WHEN** 一個進行中的 change 已勾任務數為 0,但 touched 記錄含檔案,對其執行 speclink in-progress remove
- **THEN** exit 非 0,stderr 列出 touched 記錄的檔案清單並說明需以人工或 agent 判斷處理;meta 與 touched 記錄皆不變

##### Example: 證據清單為兩版記錄的聯集去重

- **GIVEN** touched 記錄 v1 清單含 src/a.rs 與 src/b.ts,v2 清單含 src/b.ts 與 src/c.rs
- **WHEN** 對該 change 執行 speclink in-progress remove
- **THEN** stderr 的檔案清單恰為 src/a.rs、src/b.ts、src/c.rs 三項,無重複

#### Scenario: 未開工的變更冪等成功

- **WHEN** 對一個 meta 無任何 started_* 欄位的 change 執行 speclink in-progress remove
- **THEN** exit 0,不寫入任何檔案

#### Scenario: 不存在的變更明確報錯

- **WHEN** 對不存在的 change 名稱執行 speclink in-progress remove
- **THEN** exit 非 0,stderr 報找不到該 change

#### Scenario: meta 損毀時 fail-closed

- **WHEN** 一個 change 的 .meta.yaml 無法解析,對其執行 speclink in-progress remove
- **THEN** exit 非 0 報錯,不修改任何檔案

### Requirement: in-progress 標記真相存於 change meta
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

in-progress 標記 SHALL 以 change 目錄的 .meta.yaml 為唯一真相：執行 speclink in-progress add 某 change 後，該檔 SHALL 含 started_at（ISO 日期）；started_by 與 started_with SHALL 依 created_by／created_with 的同一身分機制寫入——呼叫端可歸屬者寫入、不可歸屬者缺席（CLI 以 git 身分供 started_by；CLI 現無 agent 識別來源，started_with 缺席，寫入縫留在引擎函式的 agent 參數，desktop／remote 通道屆時供給）。既有欄位（schema、created_*、from_discussion 等）SHALL 原樣保留。指令 SHALL NOT 建立 .git/speclink-app/ 目錄或其下任何檔案。重複執行 SHALL 冪等——已有 started_* 時欄位值不變。本需求為 parity 敏感：指令的 stdout、stderr 與 exit code SHALL 與遷移前的行為位元級一致（首次與重複執行皆然）；對不存在 change 的行為 SHALL 維持遷移前實測基線——靜默成功（無輸出、exit 0），且 SHALL NOT 寫入任何檔案。

#### Scenario: 標記後 meta 含三站中的開工欄位

- **WHEN** 對含 created_* 欄位的 change 執行 speclink in-progress add 該 change
- **THEN** 該 change 的 .meta.yaml 新增 started_at 與 started_by（git 身分可得時），created_* 與 schema 欄位逐字元保留，且 .git/speclink-app/ 未被建立；經引擎函式帶 agent 識別呼叫時另含 started_with

#### Scenario: 重複標記冪等

- **WHEN** 對已含 started_* 的 change 再次執行 speclink in-progress add
- **THEN** 三欄位值與首次標記後完全相同（保留首次開工蓋章），stdout 與 exit code 與首次執行一致

#### Scenario: 不存在的 change 行為不變

- **WHEN** 對不存在的 change 名執行 speclink in-progress add
- **THEN** 與遷移前版本一致地靜默成功——無輸出、exit 0（遷移前實測基線：名稱不驗證），且無任何檔案被寫入（無 meta 變動、無 .git/speclink-app/）

### Requirement: restale_from 記錄變更待重新反映的討論並經 CLI 觀測
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

變更 meta 檔（openspec/changes/<name>/.meta.yaml）MAY 帶 restale_from 欄位——逗號分隔的討論 slug 清單，語意為「本變更曾反映（seal）這些討論，其後這些討論被重新結論，內容相對新結論過期、待 re-ingest」。ChangeMeta SHALL 提供 restale_from() accessor 回傳 Vec<String>：欄位缺席時回空、逗號值 SHALL 各段 trim 後分割，行為平行既有 from_discussion／from_discussions()。此欄位由 discuss conclude 寫入、discuss seal 清除（見 discussion-docs 正式規格），本需求規範其讀取與觀測。speclink show <change> --json SHALL 於變更 payload 恆曝 restaleFrom（camelCase 字串陣列，無旗標為空陣列），平行既有 fromDiscussions。speclink list --json SHALL 於 restale_from 非空的變更 payload 曝 restaleFrom 陣列、為空時省略該欄位——以維持 list --json 對無旗標變更的既有輸出逐位元不變。speclink analyze <change> 於某變更 restale_from 非空時 SHALL 出一條資訊性 finding，指明該變更反映的討論已重新結論、需重新 ingest 以同步新結論。此欄位讀取 SHALL 為零 per-load 掃描——僅讀既存 meta 欄位，不掃描討論記錄。

#### Scenario: restale_from() accessor 讀取

- **WHEN** 變更 meta 含 restale_from: alpha-search, beta-cache
- **THEN** ChangeMeta::restale_from() 回傳 ["alpha-search", "beta-cache"]；meta 無該欄位時回傳空 Vec

#### Scenario: show 恆曝 restaleFrom

- **WHEN** 對 restale_from 含 alpha-search 的變更、以及無該欄位的變更，各執行 speclink show <change> --json
- **THEN** 前者 payload 的 restaleFrom 為 ["alpha-search"]；後者 payload 的 restaleFrom 為空陣列（欄位恆存在）

#### Scenario: list 曝 restaleFrom 且對無旗標變更輸出不變

- **WHEN** 對含一個 restale_from 非空變更與一個無該欄位變更的專案執行 speclink list --json
- **THEN** 非空變更的 payload 含 restaleFrom 陣列（如 ["alpha-search"]）；無旗標變更的 payload 省略 restaleFrom 欄位，其 list --json 輸出與本變更前逐位元一致

#### Scenario: analyze 對過期變更出資訊性 finding

- **WHEN** 對 restale_from 非空的變更執行 speclink analyze <change>
- **THEN** 輸出含一條資訊性 finding，指明該變更反映的討論已重新結論、需 re-ingest；restale_from 為空時無此 finding，且 analyze 輸出與本變更前逐位元一致

### Requirement: 任務完成蘊含開工標記
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

speclink task done 成功完成一項任務時，若該 change 的 .meta.yaml 尚無 started_* 欄位，SHALL 於同一操作內蓋開工章：started_at 為當日 ISO 日期；started_by 依 git 身分可得性寫入——可歸屬者寫入、不可歸屬者缺席；本指令無 agent 識別來源，started_with 缺席。meta 既有欄位 SHALL 逐字元保留。該 change 已有 started_* 時 SHALL 保留首章不變。touched-files 記錄（.speclink/ 下）行為 SHALL 維持現行語意。

本需求為輸出凍結敏感：指令的人眼輸出、--json payload（change、status、taskDesc、taskId 對應之既有欄位形狀）與 exit code SHALL 與現行位元級一致（既有輸出基線不變）；.meta.yaml 的開工章為刻意檔案效果變更——自我基線的檔案樹對照 SHALL 隨本需求更新並記載此差異。錯誤路徑（tasks.md 缺失、任務序號無效、任務已完成）SHALL 維持現行訊息與非零 exit code，且 SHALL NOT 寫入任何檔案。

#### Scenario: 首次完成任務蓋開工章

- **WHEN** 對 meta 含 created_* 而無 started_* 的 change 執行 speclink task done 完成一項未完成任務
- **THEN** tasks.md 該任務標記為 [x]，stdout 與 exit code 與現行一致，.meta.yaml 新增 started_at（git 身分可得時另含 started_by），schema 與 created_* 欄位逐字元保留

##### Example: meta 前後對照

- **GIVEN** .meta.yaml 內容為 schema、created、created_by、created_with 四欄，tasks.md 為 0/5
- **WHEN** speclink task done 1 --change demo 成功
- **THEN** .meta.yaml 於既有四欄之後新增 started_at: <當日> 與 started_by: <git 身分>，無其他變動；tasks.md 成 1/5

#### Scenario: 已開工的 change 完成後續任務不改章

- **WHEN** 對已含 started_* 的 change 執行 speclink task done 完成另一項任務
- **THEN** started_at、started_by、started_with 值與執行前完全相同（首章保留），tasks.md 正常勾章

#### Scenario: 任務已完成時無任何檔案效果

- **WHEN** 對已標記 [x] 的任務再執行 speclink task done
- **THEN** 指令以現行「already done」錯誤訊息與非零 exit code 結束，tasks.md、.meta.yaml 與 touched 記錄皆無變動

#### Scenario: tasks.md 缺失時不蓋章

- **WHEN** 對無 tasks.md 的 change 名執行 speclink task done
- **THEN** 指令以現行「tasks.md not found」錯誤結束，該 change 的 .meta.yaml（若存在）無任何變動

### Requirement: 壞 metadata 使生命週期寫入 fail closed
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

change 的 `.meta.yaml` 存在但 YAML 解析失敗時，讀寫該 change 生命週期狀態的動詞——in-progress add、claim、task done、task undone、new artifact、archive 與 discard——SHALL 以帶檔案位置與解析原因的錯誤拒絕，且 SHALL NOT 寫入、移動或刪除任何檔案。壞 metadata SHALL NOT 被解讀為未開工、預設 schema 或無來源討論。「檔案不存在」與「欄位缺席」SHALL 維持既有預設行為（「meta 新欄位向後相容」需求不變，其約束對象是缺欄位而非壞檔）。

#### Scenario: in-progress add 對壞 metadata 拒絕且不疊寫

- **WHEN** 對 `.meta.yaml` 為壞 YAML 的 change 執行 speclink in-progress add 該 change
- **THEN** 以非零 exit code 結束；該 `.meta.yaml` 逐位元不變（未被文字手術追加或代換 started_* 行）

#### Scenario: task done 因蘊含開工標記而拒絕

- **WHEN** 對壞 metadata 的 change 執行 task done 勾選任一任務
- **THEN** 以非零 exit code 結束；tasks.md 與 `.meta.yaml` 皆逐位元不變

#### Scenario: discard 不得把壞 metadata 當未開工

- **WHEN** 對壞 metadata 的 change 執行 speclink discard 該 change（未帶 --force）
- **THEN** 以非零 exit code 結束且 change 目錄完整保留；stderr 指出 metadata 損壞，而非以「未開工」放行刪除

#### Scenario: discard 帶 --force 仍拒絕

- **WHEN** 對壞 metadata 的 change 執行 speclink discard 該 change --force
- **THEN** 以非零 exit code 結束且 change 目錄完整保留（使用者修復 metadata 後方可廢棄）

#### Scenario: archive 對壞 metadata 拒絕

- **WHEN** 對壞 metadata 的 change 執行 speclink archive 該 change
- **THEN** 以非零 exit code 結束；正式規格未被併入、change 目錄未被移動

### Requirement: 歸檔保留完整生命週期歸屬
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

speclink archive 一個已標記開工的 change 後，封存目錄的 .meta.yaml SHALL 同時含 created／created_by（建立站）、started_at／started_by／started_with（開工站）與 archived_at／archived_by（歸檔站）——started_* 欄位 SHALL NOT 於歸檔時被剝除或改寫。

#### Scenario: 歸檔後三站欄位並存

- **WHEN** 對 meta 含 created_* 與 started_* 的 change 執行 speclink archive
- **THEN** changes/archive/ 下該 change 的 .meta.yaml 同時含三站全部欄位，started_* 的值與歸檔前逐字元一致
