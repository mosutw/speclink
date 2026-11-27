## Purpose

提供以 Node SDK 內嵌 Speclink 的公開入口，定義宿主供應的身分、範圍、掛鉤、HTTP 入口與移轉包，讓宿主能用自己的資料庫服務既有用戶端。

## ADDED Requirements

### Requirement: 內嵌宿主的授權與事件

createHost SHALL 提供 host.dispatch 與 host.handleRequest，使用呼叫端給的身分、範圍與 store；beforeCommand 可透過唯讀讀取器裁決常規動詞及直接讀取路由，afterCommit 只觀察已提交變更及各受影響資源的事件。

#### Scenario: 拒絕的唯讀請求
- **WHEN** beforeCommand 拒絕讀取某範圍的規格、搜尋或文件快照
- **THEN** HTTP 回 409 與拒絕原因，無文件內容，沒有提交及 afterCommit

#### Scenario: 跨資源提交
- **WHEN** 使用轉出動詞在同一提交建立變更並更新來源討論
- **THEN** 兩個資源各有可對應的事件，失敗時兩者都沒有成功事件

### Requirement: 範圍匯入模式

host SHALL 提供 exportScope、importScope 與對應 HTTP 入口，支援格式 2、空範圍建立、replace、dryRun、大小上限與穩定的 reason。dryRun 回報 added、changed、removed 的字串陣列。

#### Scenario: 預覽後執行取代
- **WHEN** 目標有 old.md，傳入只含 new.md 的有效包並依序呼叫 dryRun 和 replace
- **THEN** 前者分別回報 new.md 新增和 old.md 移除且儲存不變，後者讓該範圍只留下 new.md

#### Scenario: 衝突時保住範圍
- **WHEN** replace 的目標版本在列出後被另一筆提交改動
- **THEN** 回報 revision 衝突且不套用包內任何寫入

### Requirement: 宿主保留自由欄位

createHost 的 reservedFieldPrefixes SHALL 讓其指定的鍵只可經宿主提供的 fields 設定，使用者 argv 或 HTTP 請求不得設定或刪除這些鍵。

#### Scenario: 外部欄位被拒絕
- **WHEN** 請求嘗試寫入匹配保留前綴的鍵
- **THEN** 在掛鉤和儲存操作前拒絕，SDK 回 refused、HTTP 回 409
