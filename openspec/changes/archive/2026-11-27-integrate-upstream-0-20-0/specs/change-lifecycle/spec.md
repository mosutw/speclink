## ADDED Requirements

### Requirement: 就緒時間與工作樹讀取

最後一項任務經動詞或桌面勾選完成時，系統 SHALL 寫入 ready_at。仍有未完任務時，show 的 JSON SHALL 省略 readyAt。開啟 worktree 時，主工作區的 show 與 list SHALL 使用同一份變更副本。

#### Scenario: 最後一項任務完成
- **WHEN** task done 勾完最後一個任務後於主工作區執行 show --json
- **THEN** 成功碼為 0，stdout 的 readyAt 是 YYYY-MM-DD 字串，來源與 list 一致

#### Scenario: 完成後再取消勾選
- **WHEN** 有就緒時間的變更取消勾選一項任務
- **THEN** 保留時戳歷史，後續 show 不輸出 readyAt

### Requirement: 自由欄位與追蹤操作

CLI 與遠端 SHALL 提供 change／discuss field、discuss restore、claim --release、change landed 與證據讀取。自由欄位在 JSON 是 fields 物件；落地的 landedCommit 與章上的 commit 照原資料各自保存。

#### Scenario: 記錄落地與讀取封存證據
- **WHEN** 提交後透過 change landed 記錄，並查詢該變更的證據
- **THEN** 詳細查詢回報落地 commit，封存後仍能讀取任務證據與工單歷史

#### Scenario: 無效欄位或變更
- **WHEN** 欄位鍵不合規則或指定不存在的變更
- **THEN** 非零退出、stderr 說明原因、資料不變
