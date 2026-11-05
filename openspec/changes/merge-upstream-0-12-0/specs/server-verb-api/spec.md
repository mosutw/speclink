## MODIFIED Requirements

### Requirement: plan 唯讀衍生查詢端點

<!-- BEFORE: 本次以前已存在的契約為底，新增上游 v0.12.0 的排程欄位或統一分刀呈現。 -->

server SHALL 提供 GET /plan（scope 層級）：經 Command gateway 執行與本地相同的引擎 plan 計算，rank 來源 SHALL 為該 scope 的 board resource 文件（缺席、無法解析或非物件時視為全員缺 rank），回應為 typed DTO（waves、changes、next、skipped、unpromotedCuts，欄位與 CLI plan --json 同形，changes 每項九鍵含 `requirementOverlap` 與 `archiveAfter`，自引擎報告逐欄搬運）附 scope ETag。端點 SHALL 對 reader 與 editor 皆可用，SHALL NOT 產生任何寫入或事件發布。宣告依賴成環時 SHALL 回 HTTP 409、reason 為 refused（沿用封閉 error reason registry，SHALL NOT 新增 reason 值）、message 為引擎的成環訊息（含成環路徑）；成環以外的引擎一般錯誤 SHALL 維持既有映射。同一 scope 內容與同一 rank 圖下，回傳的配置順序、波次、requirementOverlap、archiveAfter 與 unpromotedCuts SHALL 與本地 fs 模式對同一內容的結果一致；unpromotedCuts SHALL 由同一個引擎計算自該 scope 的未封存討論，SHALL NOT 另寫一份分刀解析。

#### Scenario: reader 可讀 plan

- **WHEN** 以 reader role 憑證呼叫 GET /plan，scope 內 add-b 的 depends_on 含 add-a
- **THEN** HTTP 200，changes 內 add-a 在 add-b 之前、add-b 的 blockedBy 為 ["add-a"]，每項含 requirementOverlap 與 archiveAfter 鍵，附 scope ETag，revision 不前進

#### Scenario: rank 來自 board resource

- **WHEN** board resource 的 changes 段為 {"add-b":"b","add-a":"f"}，兩者無依賴且不重疊
- **THEN** GET /plan 的 changes 順序為 add-b、add-a（rank 字典序），波次皆為 1

#### Scenario: 壞 board 內容視為空圖

- **WHEN** board resource 內容為非 JSON 文字
- **THEN** GET /plan 仍 HTTP 200，順序依 change-plan 規定的缺 rank 順序（全員缺 rank；無人依賴且 task 數相同時即 created 升冪），不回錯誤

#### Scenario: 成環回 409

- **WHEN** add-a 與 add-b 互相 depends_on，呼叫 GET /plan
- **THEN** HTTP 409，reason 為 refused，message 為 `dependency cycle: add-a -> add-b -> add-a`，revision 不前進

#### Scenario: 同 requirement 重疊帶 archiveAfter

- **WHEN** scope 內 add-a 與 add-b 皆 MODIFIED 同一 capability 的同一 requirement，無依賴、無 rank、add-a 配置在前
- **THEN** GET /plan 的 add-b 項 archiveAfter 為 ["add-a"]、requirementOverlap 含 change 為 add-a 且 conflict 為 false 的一項，兩者 wave 皆為 1

#### Scenario: 回應帶尚未轉出的刀

- **WHEN** scope 內一份未封存討論 talk-a 的結論 Decision 含 ``**cut 1 `add-a`**: 第一刀`` 與 ``**cut 2 `add-b`**: 第二刀``，其 promoted_to 為 add-a，呼叫 GET /plan
- **THEN** HTTP 200，unpromotedCuts 恰一項，discussion 為 talk-a、cut 為 `cut 2`、title 為 `第二刀`、changes 與 unpromoted 皆為 ["add-b"]，revision 不前進

## ADDED Requirements

### Requirement: 討論詳情回應攜帶分刀

GET /discussions/{slug} 的回應 SHALL 攜帶 `cuts`（形狀依 client-protocol「討論詳情回應的分刀 payload」），live 與已封存的記錄皆然。`cuts` SHALL 只由引擎的分刀解析計算，輸入為該次請求讀到的同一份記錄的結論與 `promoted_to`——與回應的 `content` 及 `info.promotedTo` 出自同一次讀取；server SHALL NOT 重讀檔、SHALL NOT 另寫一份分刀解析。記錄沒有結論或結論沒有刀時 `cuts` SHALL 為空陣列。既有的 `info` 與 `content` SHALL 不變；slug 不存在時照舊回 404、無 `cuts`。

#### Scenario: live 記錄帶分刀

- **WHEN** live 討論 panel 的 promoted_to 為 `add-y`，結論的 Decision 有 ``- **cut 2 `add-y`**: 引擎解析`` 與 ``- **cut 3 `add-z`**: 路線圖`` 兩條列點，client 以具讀取權限的身分呼叫 GET /discussions/panel
- **THEN** 回應 200，`cuts` 為兩項：第一項 cut 為 `cut 2`、changes 為 ["add-y"]、unpromoted 為 []；第二項 cut 為 `cut 3`、title 為「路線圖」、changes 為 ["add-z"]、unpromoted 為 ["add-z"]；`info.promotedTo` 為 ["add-y"]

#### Scenario: 已封存記錄帶分刀

- **WHEN** 討論 old-topic 已封存，其結論有一刀 ``- **cut 1 `add-a`**: 骨架``、promoted_to 為 `add-a`，client 呼叫 GET /discussions/old-topic
- **THEN** 回應 200，`info.archived` 為 true，`cuts` 為一項，cut 為 `cut 1`、unpromoted 為 []

#### Scenario: 沒有結論的記錄

- **WHEN** 討論 fresh-topic 只有 Context 與 Rounds、沒有結論，client 呼叫 GET /discussions/fresh-topic
- **THEN** 回應 200，`cuts` 為空陣列，`content` 為記錄原文
