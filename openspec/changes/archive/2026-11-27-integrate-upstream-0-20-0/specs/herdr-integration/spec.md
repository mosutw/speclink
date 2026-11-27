## ADDED Requirements

### Requirement: 工作標籤、分頁與等候原因

Herdr SHALL 顯示等候允許、回答或登入的原因，透過 hc run 在專案對應空間開分頁並帶著工作標籤。成功的 CLI 變更與討論操作只在所在終端機回報標籤。

#### Scenario: 終端機回報停用
- **WHEN** 設 SPECLINK_TERMINAL_REPORT=false 或在 Herdr 外執行同一條成功的 CLI 指令
- **THEN** 沒有終端回報，stdout、stderr、退出碼照原契約

#### Scenario: 派工有明確歸屬
- **WHEN** 從甲專案空間派工給乙專案的變更
- **THEN** 新分頁屬乙專案，面板依標籤顯示對應工作與可取得的等候原因

### Requirement: 可見性、指令面板與詳情連結

面板 SHALL 在 Herdr 0.1.18 的視窗不可見時暫停拉取，恢復可見時補抓一次。指令面板提供技能與面板入口，至少 0.1.17 的終端機可透過路徑開啟詳情，worktree 路徑回到主專案。

#### Scenario: 隱藏後恢復視窗
- **WHEN** 先將 Herdr 視窗完整遮住再顯示，接著點 worktree 的變更路徑
- **THEN** 隱藏期間不拉取，顯示時補抓一次，桌面打開主專案下的正確變更
