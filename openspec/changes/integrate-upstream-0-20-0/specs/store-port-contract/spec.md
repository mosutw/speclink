## Purpose

定義每個宿主範圍的文件儲存接口及可由 JavaScript 和 Rust 共用的行為驗證，涵蓋版本、原子提交、事件以及 SQLite、PostgreSQL、檔案系統後端。

## ADDED Requirements

### Requirement: 三個操作與單次提交

每個儲存實作 SHALL 提供 list、read、commit。list 回傳排序後的文件鍵與版本，read 批次取回內容，commit 依版本提交同一範圍的文件及事件。合格測試 SHALL 可驗證 JS store 與三個 Rust 後端。

#### Scenario: 一次修改多份文件
- **WHEN** 提交同一範圍的兩份文件和一筆事件
- **THEN** 兩份文件及事件一起出現，其他範圍不變

#### Scenario: 版本不符
- **WHEN** 任一文件的預期版本已過期
- **THEN** 整筆拒絕，既有內容及事件保持原狀
