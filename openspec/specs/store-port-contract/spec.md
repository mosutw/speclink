# store-port-contract Specification

## Purpose

定義每個宿主範圍的文件儲存接口及可由 JavaScript 和 Rust 共用的行為驗證，涵蓋版本、原子提交、事件以及 SQLite、PostgreSQL、檔案系統後端。

## Requirements

### Requirement: 三個操作與單次提交

每個儲存實作 SHALL 提供 list、read、commit。list 回傳排序後的文件鍵與版本，read 批次取回內容，commit 依版本提交同一範圍的文件及事件。合格測試 SHALL 可驗證 JS store 與三個 Rust 後端。

#### Scenario: 一次修改多份文件
- **WHEN** 提交同一範圍的兩份文件和一筆事件
- **THEN** 兩份文件及事件一起出現，其他範圍不變

#### Scenario: 版本不符
- **WHEN** 任一文件的預期版本已過期
- **THEN** 整筆拒絕，既有內容及事件保持原狀

#### Scenario: 驗收讀到的版本已改變
- **WHEN** 政策或證據文件在驗收讀取後被另一筆提交更新，而且封存提交未寫入該文件
- **THEN** Rust 內建後端 SHALL 回報版本衝突，整筆不寫入章、封存資料或事件。以現行版本重試成功時，待核對文件的版本與修改時間 SHALL 保持原樣

<!-- @trace
source: integrate-upstream-0-20-0
updated: 2026-11-27T16:06:32+08:00
-->