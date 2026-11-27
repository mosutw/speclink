## ADDED Requirements

### Requirement: 匯出與宿主工具

SDK SHALL 提供 createEngine 的 exportScope、commandNames、eventNames、eventResource、agentToolDescription 及 skills.render 的 skills、checkout、gitHistory 選項，新渲染入口拒絕未知鍵。SDK 的共用動詞採 CLI 語意，舊 createEngine 與舊 dispatch 契約保留既有用法及形狀。

#### Scenario: 有 checkout 的本機匯出
- **WHEN** 本機規格目錄含變更、討論及正式規格且沒有符號連結
- **THEN** 回傳依 key 排序的 formatVersion 2 文件包，documents 內 key、content、digest 均為字串，舊 SDK 查詢照常可用

#### Scenario: 無法匯出的工作區
- **WHEN** 本機沒有規格資料夾、含符號連結或變更的 schema 不可用
- **THEN** 回結構化 Error，schema 不可用時帶 change 與 path，來源檔案保持原樣

#### Scenario: 裁剪宿主工具
- **WHEN** 宿主宣告 gitHistory 為 false 並從工具說明排除一個動詞
- **THEN** 渲染說明跳過 Git 歷史步驟，排除的動詞不出現在工具說明
