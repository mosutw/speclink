## ADDED Requirements

### Requirement: 參考 Server 儲存升級

參考 Server SHALL 支援新 Store Port 與三種後端，遇到舊儲存時拒絕啟動並提示舊版備份、新版空目標還原；還原須保留身分與資料，也包含 fork 的安全及品質歷史。

#### Scenario: 從舊資料升級
- **WHEN** 依照提示以舊版產生有效備份並在新版空庫還原
- **THEN** 新版讀到原有工作區與品質及安全資料，可以用原授權登入及查詢

#### Scenario: 還原目標有資料
- **WHEN** 試圖將舊版備份還原到已有資料的目標
- **THEN** 還原非零退出，stderr 顯示原因，目標既有資料不變
