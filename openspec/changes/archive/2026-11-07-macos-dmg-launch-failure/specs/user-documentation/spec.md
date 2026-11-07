## MODIFIED Requirements

### Requirement: 開發者入口文件雙語對
<!-- BEFORE: 中英 development 文件描述五個 npm 入口及未簽章放行，未區分 macOS ad-hoc 產物與完整 Developer ID／公證交付。 -->

docs/development.md（英文）與 docs/development.zh-TW.md（繁體中文）SHALL 存在且章節結構與事實內容對等（僅語言不同），並受既有「中英文文件保持結構與事實對等」與「文件準確性具可重複驗證清單」需求約束。內容 SHALL 涵蓋：npm run dev、npm run dev:server、npm run dev:desktop、npm run dev:reset、npm run cli 五個入口各自的用途、前置條件與預期可觀察結果；下載安裝檔的未使用發行者憑證時放行步驟（macOS 系統設定對個別 App 明確允許、Windows SmartScreen 仍要執行），放行一節的產物名稱 SHALL 與 desktop-release 規格一致。macOS 說明 SHALL 區分完整 ad-hoc 簽章與 Developer ID 簽章加 Apple 公證：無 Apple 憑證的 Release 或 Desktop Build 產物 SHALL NOT 被描述為已經 Apple 公證，首次開啟說明 SHALL 指向 macOS 的個別 App 允許操作；完整 Developer ID 與公證模式 SHALL 依實際驗證結果標示。文件 SHALL NOT 將全域停用 Gatekeeper、永久移除原安裝 quarantine 或修改原安裝簽章作為一般交付方案。文件中出現的指令 SHALL 全部為已驗證可執行的入口。README.md 與 README.zh-TW.md SHALL 各含一處指向對應語言 development 文件的連結。

#### Scenario: 一鍵入口涵蓋完整

- **WHEN** 檢視 development 文件任一語言版
- **THEN** 五個 npm 入口各有一節，敘明用途、前置條件與預期可觀察結果（如 dev:server 會印出 /setup 連結、dev:desktop 會開啟本地模式視窗）

#### Scenario: 雙語結構對等

- **WHEN** 對照 docs/development.md 與 docs/development.zh-TW.md 的章節骨架
- **THEN** 兩檔章節一一對應、事實一致，僅語言不同

#### Scenario: README 導流

- **WHEN** 檢視 README.md 與 README.zh-TW.md
- **THEN** README.md 含指向 docs/development.md 的連結、README.zh-TW.md 含指向 docs/development.zh-TW.md 的連結

#### Scenario: 未簽章放行教學可依循

- **WHEN** 使用者依放行一節在 macOS 開啟無 Apple 發行者憑證、但完成完整 ad-hoc 簽章的 DMG 安裝版 App，或在 Windows 執行未簽章安裝器
- **THEN** 文件步驟足以完成個別放行（macOS：系統設定 > 隱私權與安全性的強制打開；Windows：SmartScreen 其他資訊 > 仍要執行），且步驟中的產物名稱與實際 Release assets 相符

#### Scenario: ad-hoc 下載產物不宣稱 Apple 公證

- **WHEN** 使用者閱讀任一語言的 macOS 安裝說明，且下載的產物來自無 Apple 憑證的 Release 或 Desktop Build
- **THEN** 說明明列產物採 ad-hoc、不具 Developer ID／Apple 公證，列出首次對個別 App 明確允許的步驟，不寫所有正式 DMG 都無需放行，也不要求停用全域安全檢查

#### Scenario: 完整 Developer ID 模式依證據說明

- **WHEN** 文件介紹完整 Apple 憑證模式，且該模式的簽章、公證票證與 Gatekeeper 驗證已通過
- **THEN** 說明將已完成的 Developer ID／公證結果限定於該模式，不延伸到 ad-hoc 或尚未實際驗證的產物
