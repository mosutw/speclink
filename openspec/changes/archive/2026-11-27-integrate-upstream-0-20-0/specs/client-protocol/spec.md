## ADDED Requirements

### Requirement: 登入發現與連線狀態

遠端用戶端 SHALL 依 Server 宣告選用瀏覽器、裝置或存取金鑰登入，以作業系統信任庫驗證 TLS。憑證存入 OS 金鑰圈，登出移除同一 Server 的憑證。SSE 使用 Server 宣告的同 origin 位址。

#### Scenario: 多 repo 綁定
- **WHEN** 同專案對應兩個 repo，執行 speclink link 且未指定 repo
- **THEN** CLI 非零退出並在 stderr 提示 --repo 及可用 repo，原綁定保持不變

#### Scenario: 服務錯誤與認證失效
- **WHEN** 遠端查詢收到 5xx 或登入已失效
- **THEN** CLI 非零退出且 stderr 顯示可辨認的失敗原因，未確認的寫入不得當成成功

### Requirement: 詳細查詢與舊版容錯

遠端的可讀資料 SHALL 包含已封存清單、詳細清單與工單、證據、trace 與 plan 排程；新增欄位為選填，包括 whyExcerpt、stage、createdById、readyAt、modifiedAt、landedCommit（字串）、review／verify（物件）與 promotedTo（字串陣列）。

#### Scenario: 本機與遠端詳細清單一致
- **WHEN** 兩個工作區含相同變更，執行 list --json --detail
- **THEN** 成功碼為 0，stdout 含同語意的欄位與 camelCase 拼法，--no-color 不加 ANSI

#### Scenario: 舊版回應缺欄位
- **WHEN** 新用戶端讀到舊 Server 的資料
- **THEN** 解析成功，新增欄位維持缺席；沒支援的動詞顯示能力不足或服務端錯誤，原資料不變
