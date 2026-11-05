## MODIFIED Requirements

### Requirement: plan 回應 payload

<!-- BEFORE: 本次以前已存在的契約為底，新增上游 v0.12.0 的排程欄位或統一分刀呈現。 -->

protocol SHALL 以 Rust 型別定義 GET /plan 的回應：`waves`（陣列，每項 `index` 整數與 `changes` 字串陣列）、`changes`（陣列，每項 `name`、`wave`、`stage`、`dependsOn`、`overlaps`（每項 `change` 與 `capabilities`）、`blockedBy`、`ready`、`requirementOverlap`（每項 `change`、`capability`、`requirement`、`ownOperation`、`otherOperation`、`conflict`）、`archiveAfter`）、`next`（字串或 null）、`skipped`（每項 `change` 與 `reason`）、`unpromotedCuts`（每項 `discussion`、`cut`、`title` 字串與 `changes`、`unpromoted` 字串陣列）。欄位一律 camelCase，陣列欄位缺席 SHALL 讀作空陣列（含 `requirementOverlap`、`archiveAfter` 與頂層的 `unpromotedCuts`，讓新 client 讀舊 server）。remote client SHALL 提供型別化的 plan 呼叫；CLI remote 模式 SHALL 把回應轉回引擎的 plan 報告型別後共用 fs 模式的渲染，轉換 SHALL 逐欄搬運 changes 每項的九鍵與 unpromotedCuts 每項的五鍵、SHALL NOT 以空值填任何欄位，SHALL NOT 另寫第二份渲染；`stage` 為 proposed／in-progress／ready 以外的值、或 `ownOperation`／`otherOperation` 為 ADDED／MODIFIED／REMOVED／RENAMED 以外的值時，轉換 SHALL 失敗並回報錯誤，SHALL NOT 猜測；`waves` 列出 `changes` 沒有的名稱時轉換同樣 SHALL 失敗並回報錯誤，SHALL NOT 使 CLI 崩潰。JSON Schema 匯出 SHALL 隨 Rust 型別更新。

#### Scenario: 回應反序列化

- **WHEN** client 收到 `{"waves":[{"index":1,"changes":["a"]}],"changes":[{"name":"a","wave":1,"stage":"proposed","dependsOn":[],"overlaps":[],"blockedBy":[],"ready":true,"requirementOverlap":[{"change":"b","capability":"desktop-app","requirement":"看板與任務","ownOperation":"MODIFIED","otherOperation":"MODIFIED","conflict":false}],"archiveAfter":["b"]}],"next":"a","skipped":[]}`
- **THEN** 反序列化成功，next 為 "a"，changes[0].ready 為 true，changes[0].requirementOverlap[0].conflict 為 false，changes[0].archiveAfter 為 ["b"]

#### Scenario: 缺陣列欄位容忍

- **WHEN** client 收到 `{"waves":[],"changes":[{"name":"a","wave":1,"stage":"proposed","dependsOn":[],"blockedBy":[],"ready":true}],"next":"a","skipped":[]}`（change 項缺 overlaps、requirementOverlap、archiveAfter 鍵）
- **THEN** 反序列化成功且 changes[0].overlaps、requirementOverlap、archiveAfter 皆為空陣列

#### Scenario: wave 成員不在 changes 時轉換失敗

- **WHEN** client 收到 `{"waves":[{"index":1,"changes":["add-a"]}],"next":"add-a"}`（缺 changes 鍵）並轉回引擎的 plan 報告
- **THEN** 轉換回報錯誤、訊息含 `add-a`，不崩潰

#### Scenario: 轉回引擎報告逐欄搬運

- **WHEN** client 收到 changes[0] 帶一項 requirementOverlap 與 archiveAfter ["b"] 的回應並轉回引擎的 plan 報告
- **THEN** 引擎報告的 changes[0].requirement_overlap 有一項且 archive_after 為 ["b"]

#### Scenario: 不認得的操作轉換失敗

- **WHEN** client 收到的 requirementOverlap 項 ownOperation 為 `MOVED` 並轉回引擎的 plan 報告
- **THEN** 轉換回報錯誤、訊息含 `MOVED`，不崩潰

#### Scenario: 舊 server 缺尚未轉出的刀

- **WHEN** client 收到 `{"waves":[],"changes":[],"next":null,"skipped":[]}`（缺 unpromotedCuts 鍵）並轉回引擎的 plan 報告
- **THEN** 反序列化與轉換皆成功，引擎報告的 unpromoted_cuts 為空

#### Scenario: 尚未轉出的刀逐欄搬回

- **WHEN** client 收到 unpromotedCuts 為 `[{"discussion":"talk-a","cut":"cut 2","title":"第二刀","changes":["add-b","add-c"],"unpromoted":["add-c"]}]` 的回應並轉回引擎的 plan 報告
- **THEN** 引擎報告的 unpromoted_cuts 恰一項，discussion、cut、title 與回應相同，changes 為 ["add-b","add-c"]、unpromoted 為 ["add-c"]

### Requirement: 變更清單的排程欄位

<!-- BEFORE: 本次以前已存在的契約為底，新增上游 v0.12.0 的排程欄位或統一分刀呈現。 -->

desktop 協定的 local 變更清單回應 SHALL 以引擎 plan 的配置順序排列 change 項，並增列 `wave`（整數）、`blockedBy`（字串陣列，只含作用中宣告前置）、`dependsOn`（字串陣列）、`overlaps`（陣列，每項 `change` 字串與 `capabilities` 字串陣列升冪）、`requirementOverlap`（陣列，每項 `change`、`capability`、`requirement`、`ownOperation`、`otherOperation` 字串與 `conflict` 布林）、`archiveAfter`（字串陣列）六欄，值 SHALL 取自引擎 plan 計算的同一入口，SHALL NOT 於呈現層另算；回應頂層 SHALL 增列 `planError`（字串或 null）、`next`（字串或 null，為引擎 plan 的 `next`）與 `unpromotedCuts`（陣列，每項 `discussion`、`cut`、`title` 字串與 `changes`、`unpromoted` 字串陣列，與 plan 回應的 `unpromotedCuts` 同形同序），三者 SHALL 取自同一次 plan 計算。plan 因依賴成環失敗時，項目順序 SHALL 退回既有 board 排序、`wave`、`blockedBy`、`overlaps`、`requirementOverlap`、`archiveAfter` 五欄 SHALL 缺席、`dependsOn` SHALL 仍為各項 meta 的宣告原文（與 plan 列同一來源；詳情抽屜靠它讓使用者移除成環的前置）、`planError` SHALL 為引擎的成環訊息、`next` SHALL 為 null、`unpromotedCuts` SHALL 為空陣列。壞 metadata 的 change 項 SHALL 照舊列出且六欄缺席。CLI `speclink list --json` SHALL NOT 包含六欄與 `planError`、`next`、`unpromotedCuts`；remote 變更清單的排程欄位 SHALL 依「remote 變更清單的排程欄位」需求；本需求只規定 local 回應。

#### Scenario: 清單項帶排程欄位

- **WHEN** desktop 載入 local 變更清單，add-b 的 depends_on 含 add-a，add-b 與 add-c MODIFIED 同一個 requirement 且 add-c 配置在 add-b 前
- **THEN** 回應 changes 內 add-a 在 add-b 之前，add-b 項含 wave=2、blockedBy=["add-a"]、dependsOn=["add-a"]、overlaps=[]（或目錄級重疊清單）、requirementOverlap 含 change 為 add-c 的一項、archiveAfter=["add-c"]，頂層 planError 為 null、next 為引擎 plan 的 next，既有欄位不變

#### Scenario: 清單回應帶尚未轉出的刀

- **WHEN** desktop 載入 local 變更清單，討論 panel 的結論有 cut 2 `add-y` 與 cut 3 `add-z`，其 promoted_to 只含 add-y，plan 的 next 為 add-y
- **THEN** 回應頂層 next 為 "add-y"，unpromotedCuts 為一項：discussion "panel"、cut "cut 3"、changes ["add-z"]、unpromoted ["add-z"]，與同一專案 `speclink plan --json` 的 unpromotedCuts 相同

#### Scenario: 成環退回

- **WHEN** add-a 與 add-b 互相 depends_on
- **THEN** 回應 changes 為既有 board 排序、各項無 wave、blockedBy、overlaps、requirementOverlap、archiveAfter 五欄，add-a 的 dependsOn 為 ["add-b"]、add-b 的為 ["add-a"]，planError 為 `dependency cycle: add-a -> add-b -> add-a`、next 為 null、unpromotedCuts 為空陣列

#### Scenario: CLI 清單不含排程欄位

- **WHEN** 執行 speclink list --json
- **THEN** change 項不含 wave、blockedBy、dependsOn、overlaps、requirementOverlap、archiveAfter，頂層無 planError、next 與 unpromotedCuts，輸出與本需求引入前逐位元一致

### Requirement: remote 變更清單的排程欄位

<!-- BEFORE: 本次以前已存在的契約為底，新增上游 v0.12.0 的排程欄位或統一分刀呈現。 -->

桌面 remote 資料源的變更清單 SHALL 以 GET /plan 的配置順序排列變更項（討論項維持 board resource overlay），並在每項疊 `wave`、`blockedBy`、`dependsOn`、`overlaps`、`requirementOverlap`、`archiveAfter` 六欄，頂層 `planError`（字串或 null）、`next`（字串或 null）與 `unpromotedCuts`（陣列，形狀同「變更清單的排程欄位」）；`next` 與 `unpromotedCuts` SHALL 逐欄搬運自 GET /plan 回應的同名欄位，SHALL NOT 於桌面另算；六欄 SHALL 為桌面側組裝，SHALL NOT 改動 server list 端點的回應與 ChangeSummary 型別；舊 server 的 plan 回應缺 `requirementOverlap` 或 `archiveAfter` 時該兩欄 SHALL 為空陣列而非缺席。清單內有 plan 未列出的變更項時 SHALL 排在末尾、維持 server 回傳序、六欄缺席。plan 端點回 409（成環）時順序 SHALL 退回 board overlay、六欄缺席、planError 為其 message、next 為 null、unpromotedCuts 為空陣列；plan 端點回其他錯誤（404 舊 server、連線失敗）時 SHALL 退回 board overlay、六欄缺席、planError 為 null、next 為 null、unpromotedCuts 為空陣列。舊 server 的 plan 回應缺 `unpromotedCuts` 時 SHALL 為空陣列。

#### Scenario: remote 清單帶排程欄位

- **WHEN** remote 分頁載入，server 的 plan 回 add-a 在 add-b 之前、add-b blockedBy ["add-a"]、add-b archiveAfter ["add-c"]
- **THEN** 桌面清單 add-a 在 add-b 之前，add-b 項含 wave=2、blockedBy=["add-a"]、archiveAfter=["add-c"]，看板顯示波次章與變淡

#### Scenario: remote 清單帶 next 與尚未轉出的刀

- **WHEN** remote 分頁載入，server 的 plan 回 next 為 "add-a"、unpromotedCuts 一項（discussion "panel"、cut "cut 3"、title "路線圖"、changes ["add-z"]、unpromoted ["add-z"]）
- **THEN** 桌面清單頂層 next 為 "add-a"，unpromotedCuts 為同一項且五鍵的值與 server 回應相同

#### Scenario: 舊 server 退回

- **WHEN** server 對 GET /plan 回 404
- **THEN** 清單順序與本需求引入前一致、各項無六欄、planError 為 null、next 為 null、unpromotedCuts 為空陣列、看板無章且無提示列

#### Scenario: remote 成環退回

- **WHEN** server 對 GET /plan 回 409、message 為 `dependency cycle: add-a -> add-b -> add-a`
- **THEN** 清單順序為 board overlay 序、各項無六欄、planError 為 `dependency cycle: add-a -> add-b -> add-a`，看板顯示成環提示列

#### Scenario: 舊 server 缺新欄位讀作空陣列

- **WHEN** server 的 plan 回應每項只有七鍵（無 requirementOverlap 與 archiveAfter）
- **THEN** 桌面清單項的 requirementOverlap 與 archiveAfter 皆為空陣列，wave 與 blockedBy 照常；plan 回應沒有 unpromotedCuts 鍵時桌面清單頂層 unpromotedCuts 為空陣列

### Requirement: 討論搜尋回應 payload

<!-- BEFORE: 本次以前已存在的契約為底，新增上游 v0.12.0 的排程欄位或統一分刀呈現。 -->

protocol SHALL 以 Rust 型別定義討論搜尋回應（JSON Schema 為匯出）：頂層 hits 陣列，每筆含與討論資訊 payload 相同的欄位（slug、topic、status、rounds、created、createdBy 選填、kind 選填、promotedTo、concluded 選填、path、archived）加 matches 陣列；每個 match 含 kind、where、text 三個字串欄位，kind 值為 topic、slug、ruled-out、decision、rejected、deferred 之一，where 值為 frontmatter、round-N 或 conclusion。欄位一律 camelCase。typed client SHALL 提供搜尋方法，以關鍵字清單組成空白分隔的 q 呼叫 GET /discussions/search，回應反序列化為 typed 型別，SHALL NOT 走 raw JSON 旁路。既有討論資訊 payload 與清單回應的欄位 SHALL 逐位元不變；詳情回應除「討論詳情回應的分刀 payload」增列的 `cuts` 外 SHALL 逐位元不變。

#### Scenario: typed client 讀取搜尋回應

- **WHEN** typed client 以關鍵字 golden 與 sse 呼叫搜尋方法，server 回一筆命中（matches 含 kind 為 deferred、where 為 conclusion 的項目）
- **THEN** 請求路徑為 /discussions/search 且 q 為 `golden sse`；回應反序列化成功，該筆的 slug、archived 與 matches 三欄位值與 JSON 一致，缺席的 createdBy 與 kind 為 None

#### Scenario: 選填欄位缺席時仍可反序列化

- **WHEN** server 回應的命中項目缺 createdBy、kind、concluded 三個鍵，promotedTo 為空陣列
- **THEN** typed client 反序列化成功，三個選填欄位為 None、promotedTo 為空清單，不報錯

## ADDED Requirements

### Requirement: 討論詳情回應的分刀 payload

protocol 的討論詳情回應（GET /discussions/{slug}）SHALL 在既有的 `info` 與 `content` 之外，於頂層增列 `cuts` 陣列，依刀在結論中的順序，每項含：`cut`（刀的識別籤原文）、`title`（範圍句，沒有時為空字串）、`note`（首行其餘的說明：範圍句取自粗體時為粗體之後去掉開頭冒號、外圍整組括號拿掉的文字，否則為第一個「——」之後的文字，都沒有時為空字串）、`line`（該刀列點去掉 `- ` 與頭尾空白的首行原文）四個字串，`changes`（變更名籤）與 `unpromoted`（變更名籤中不在該記錄 `promoted_to` 的名稱，依 `changes` 順序）兩個字串陣列，以及 `changesInBold`（變更名籤是否寫在粗體內）布林。欄位一律 camelCase。各欄的值 SHALL 由引擎自同一份記錄的結論與 `promoted_to` 計算，判定規則同 change-plan「plan 列出討論結論中尚未轉出的刀」；`changesInBold` 為 false 時 `unpromoted` SHALL 為空陣列。記錄沒有結論或結論沒有刀時 `cuts` SHALL 為空陣列。

typed client 反序列化時，`cuts` 缺席 SHALL 讀作空陣列，每項缺席的字串欄位 SHALL 讀作空字串、陣列欄位讀作空陣列、`changesInBold` 讀作 false，SHALL NOT 報錯；JSON Schema 匯出 SHALL 隨 Rust 型別更新。桌面 local 的討論記錄回應 SHALL 為 `{ content, cuts }`，`cuts` 每項與本需求同形同值；remote 的討論記錄回應 SHALL 自詳情回應逐欄搬運 `content` 與 `cuts`。CLI `speclink discuss show --json` 在 fs 與 remote 模式 SHALL NOT 包含 `cuts`，人眼與 `--json` 輸出 SHALL 與本需求引入前逐位元一致。

#### Scenario: 詳情回應帶分刀

- **WHEN** 討論 panel 的 promoted_to 為 `a`，結論的 Decision 有下表三條頂層列點，client 呼叫 GET /discussions/panel
- **THEN** 回應的 `cuts` 依序為三項，各欄如下表，每項的 `line` 為表中第一欄的原文；`info` 與 `content` 與本需求引入前相同

##### Example: 分刀各欄

| 列點首行（去掉 `- `） | cut | title | note | changes | changesInBold | unpromoted |
| --- | --- | --- | --- | --- | --- | --- |
| ``**cut 1 `a`**: 骨架——三種詳情頁改三欄`` | cut 1 | 骨架 | 三種詳情頁改三欄 | a | true | （空） |
| ``**cut 2 外殼**（拆兩個 change：`b` 外殼、`c` 選單）`` | cut 2 | 外殼 | ``拆兩個 change：`b` 外殼、`c` 選單`` | b、c | false | （空） |
| ``**cut 3 `d` `e`**: 收尾`` | cut 3 | 收尾 | （空字串） | d、e | true | d、e |

#### Scenario: 舊 server 缺 cuts 讀作空陣列

- **WHEN** server 的討論詳情回應只有 `info` 與 `content` 兩鍵
- **THEN** typed client 反序列化成功，`cuts` 為空陣列；桌面的討論記錄回應 `cuts` 為空陣列

#### Scenario: CLI 的 discuss show 不含 cuts

- **WHEN** 分別以 fs 模式與 remote 模式對同一份含刀的記錄執行 `speclink discuss show panel --json`
- **THEN** 兩次輸出都沒有 `cuts` 鍵，鍵集合彼此相同，且與本需求引入前逐位元一致
