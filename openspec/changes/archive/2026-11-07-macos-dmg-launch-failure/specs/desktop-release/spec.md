## MODIFIED Requirements

### Requirement: OS 程式碼簽章為可插鑰匙開關
<!-- BEFORE: macOS 與 Windows 的 OS 簽章組完全不存在時跳過 OS 簽章並允許發布未簽章安裝檔，完整時簽章，部分時失敗。 -->

macOS 的 Developer ID 簽章與公證、Windows 的發行者憑證簽章 SHALL 以對應平台 secrets 組是否存在為條件。macOS 簽章組完全不存在時 SHALL 對完整 App、內附 CLI 與巢狀程式碼完成 ad-hoc 簽章並通過最終產物驗證，SHALL NOT 將只有 linker 臨時簽章而未完成封裝的 App 發布。此模式不代表 Developer ID 身分或 Apple 公證，首次開啟須依系統對個別 App 明確允許。Windows 憑證組完全不存在時 SHALL 保留未使用發行者憑證的安裝檔與既有 SmartScreen 說明。macOS 簽章組 SHALL 由憑證與公證兩半構成（憑證半組：憑證內容、憑證密碼、簽章身分；公證半組：Apple ID、App 專用密碼、Team ID），齊備時 SHALL 於簽章後執行公證並將票證 staple 至產物。Windows SHALL 於本機憑證 secrets 組存在時走憑證路徑完成 Authenticode 簽章。任一平台的簽章組部分存在時 workflow SHALL 以列出缺項 secrets 名稱的錯誤失敗，SHALL NOT 產出半套簽章的安裝檔，也 SHALL NOT 降級為 ad-hoc 交付。本開關 SHALL NOT 影響更新包的 updater minisign 簽章，且 macOS ad-hoc 或 Developer ID 簽章 SHALL 在 updater 簽章產生之前完成，使更新包簽章涵蓋的是完成 OS 程式碼簽章後的檔案。

#### Scenario: 無簽章金鑰照常發布

- **WHEN** repo 未設定任何 OS 發行者憑證 secrets 且 push tag
- **THEN** macOS 產出完整 ad-hoc 簽章且最終驗證通過的安裝檔，Windows 保留沒有發行者憑證的安裝檔，workflow 成功且 assets 清單與有憑證模式一致，macOS 產物不宣稱 Apple 公證

#### Scenario: 插入金鑰即啟用簽章

- **WHEN** repo 設定了完整 macOS 簽章組（憑證半組與公證半組共六項）後 push tag
- **THEN** macOS 安裝檔內的 App 完成 Developer ID 簽章與公證、票證已 staple，Gatekeeper 評估通過，workflow 其餘步驟不變且不被 ad-hoc 覆寫

#### Scenario: 簽章組部分設定即紅燈

- **WHEN** repo 只設定 macOS 憑證半組而缺公證半組任一項（或 Windows 憑證組部分存在）並 push tag
- **THEN** workflow 以非零結束，錯誤訊息列出缺少的 secrets 名稱，Release 不發布且不以 ad-hoc 降級通過

## ADDED Requirements

### Requirement: 正式 Release 等待同 commit CI

一般 `v*` 正式 Release SHALL 在任何 build、資產上傳或發布工作開始前，等待 repo 預設分支上與 tag 指向完全相同 commit SHA 的 push CI 完成並成功。Release SHALL 重用既有 `scripts/desktop/desktop-ci-gate.mjs` 的成功判定；CI 失敗、SHA 不符或 40 分鐘內沒有成功結果時，gate SHALL 以非零結束並阻止 build、desktop、docker、engine-version 等獨立建置工作與其後的發布工作。通過 gate 後 SHALL 保留 `.github/workflows/release.yml` 原本的 `v*` 發布通路、DMG 與 updater 資產契約；SHALL NOT 改走只產出獨立桌面資產的 `desktop-v*` 通路。

#### Scenario: 同 commit CI 成功後才建置正式 Release

- **WHEN** `v*` tag 指向預設分支上的 commit，且該 SHA 的最新 push CI 已完成並成功
- **THEN** gate 成功，原有正式 Release jobs 才開始，DMG 與 updater 維持原資產通路

#### Scenario: CI 未成功時阻止 Release

- **WHEN** 同 SHA CI 失敗、回報其他 SHA，或等待 40 分鐘仍沒有成功結果
- **THEN** gate 以非零結束，打包與所有發布工作不執行，Release 不產生資產

### Requirement: 共用桌面打包設定

Release 與 Desktop Build SHALL 使用同一個 repo 私有設定入口 node scripts/release/desktop-bundle-config.mjs，參數為 release 或 desktop-build，平台依執行環境決定。此入口 SHALL 不讀 stdin，成功 exit 0 並在 stdout 僅輸出一份 JSON 物件加換行；未知參數或 macOS Release 的未知簽章模式 SHALL 非零結束，說明走 stderr，不輸出可用設定，也不進入 Tauri 建置。輸出 SHALL 無 ANSI，不新增產品 CLI 的 --json 或 --no-color 介面。macOS Release 的 none 模式 SHALL 設定 signingIdentity 為 '-'；full 模式 SHALL 保留 Apple 憑證環境且不覆寫簽章身分。Desktop Build SHALL 保留停用 updater 產物，Windows／Linux SHALL 保留原有打包行為。腳本 SHALL NOT 輸出 secret 值。

#### Scenario: 打包模式轉換符合既有通路

- **WHEN** 打包流程取得不同 channel、平台與已由 secrets 閘門判定的模式
- **THEN** 共用入口輸出下列精確覆寫值，且成功 exit 0

##### Example: 共用覆寫值

| channel | 平台 | Apple 模式 | stdout JSON |
| --- | --- | --- | --- |
| release | macOS | none | {"bundle":{"macOS":{"signingIdentity":"-"}}} |
| release | macOS | full | {} |
| release | Windows／Linux | 不適用 | {} |
| desktop-build | macOS | 不使用 Apple 憑證 | {"bundle":{"createUpdaterArtifacts":false,"macOS":{"signingIdentity":"-"}}} |
| desktop-build | Windows／Linux | 不適用 | {"bundle":{"createUpdaterArtifacts":false}} |

#### Scenario: 無效打包模式不產生可用設定

- **WHEN** 呼叫者傳入未知 channel，或 macOS Release 的簽章模式不是 none／full
- **THEN** 入口 exit 非 0、stderr 說明無效值、stdout 不含可用 JSON，該 workflow 不繼續建置

### Requirement: macOS 最終產物驗證

兩條桌面打包流程 SHALL 在 macOS 打包完成後、上傳前驗證最後交付內容：完整 App、DMG 內 App，以及完整 Release 的 updater 包內 App。repo 私有驗證入口 node scripts/release/macos-artifact-verify.mjs SHALL 接受 --app、--dmg、--mode none|full、--channel release|desktop-build，Release SHALL 另提供 --updater；不讀 stdin，不提供產品 --json／--no-color，成功 exit 0 並以無 ANSI 的 stdout 列明模式及已驗證產物。缺少參數／檔案、未知模式、非 macOS 平台、native 命令逾時或驗證失敗 SHALL 非零結束，以 stderr 點名產物、階段與原因，不輸出環境或 secret。完整 App 與內附程式碼 SHALL 通過 strict／deep／all-architectures 簽章驗證，none 模式 SHALL 確認簽章身分為 ad-hoc，full 模式 SHALL NOT 接受 ad-hoc 簽章，主程式與內附 CLI 均 SHALL 含 arm64 及 x86_64。容器內與參考 App 的版號、主程式、CLI、Info.plist 及資源封裝校驗值 SHALL 一致。none 模式 SHALL NOT 把 Gatekeeper 接受或 Apple 公證當作 ad-hoc 驗證成功的含義；full 模式 SHALL 另通過 Gatekeeper 與公證票證檢查。任一驗證失敗 SHALL 阻止該平台上傳，既有 Release 依賴 SHALL 阻止缺形態的 GitHub Release 建立；驗證 SHALL NOT 補簽或修改待發布產物。

#### Scenario: 無 Apple 憑證的最終產物通過驗證

- **WHEN** none 模式的 App、DMG 內容與 Release updater 包內容都完成完整 ad-hoc 簽章，兩個執行檔都有兩種架構且內容一致
- **THEN** 驗證 exit 0，stdout 列明 ad-hoc 與已驗證產物，流程進入原有上傳階段，但不宣稱 Gatekeeper 或公證通過

#### Scenario: DMG 或 updater 內產物異常阻止交付

- **WHEN** 參考 App 有效，但 DMG 或 updater 內 App 的簽章無效、缺 CLI、少一種架構、版號或校驗值不同
- **THEN** 驗證 exit 非 0，stderr 點名對應容器及原因，該平台產物不上傳，GitHub Release 不以缺形態資產建立

#### Scenario: linker 臨時簽章不能取代完整 App 簽章

- **WHEN** 主程式顯示 ad-hoc 資訊，但 App 未完成資源封裝或任一架構簽章驗證失敗
- **THEN** 驗證非零結束，不將顯示簽章資訊或流程成功建置當成完整驗證通過

#### Scenario: Developer ID 模式的公證失敗不降級

- **WHEN** full 模式產物的程式碼簽章有效，但 Gatekeeper 評估或公證票證檢查失敗
- **THEN** 驗證非零結束並阻止上傳，不降級為 none 模式

#### Scenario: 驗證失敗後清理本次暫存資源

- **WHEN** 驗證需要掛載 DMG 或解開 updater，且任一 native 命令失敗或超過 120 秒期限
- **THEN** 工具回報非 0 並嘗試卸載本次只讀掛載與清理本次已卸載暫存目錄；卸載失敗亦回報失敗與掛載點，不刪除仍掛載的目錄，也不動其他磁碟或使用者安裝

#### Scenario: 無效輸入在交付前失敗

- **WHEN** 呼叫者缺少必要參數／產物、使用未知模式，或在非 macOS 執行私有驗證入口
- **THEN** 入口非零結束，stderr 說明無效輸入或平台，沒有產物修改且不進入上傳

### Requirement: 打包版啟動驗收

macOS 打包修正 SHALL 以真正的安裝包啟動作為接受證據：記錄產物版號／來源、macOS 版本、CPU 架構、首次安全允許條件與主視窗觀察。成功 SHALL 表示關閉開發模式後，下載 DMG 安裝的 App 可進入可操作的看板或專案選擇器主視窗，SHALL NOT 以程序存在、CLI 可執行或簽章有效替代。ad-hoc 交付 SHALL 允許使用者透過 macOS 對個別 App 首次明確允許，不改全域安全政策；每個未實跑的原生架構或 Developer ID 模式 SHALL 明列為未驗證，不宣稱通過。

#### Scenario: ARM64 安裝版完成首次允許後顯示主視窗

- **WHEN** 使用者在 ARM64 Mac 關閉開發版，從同 commit CI 成功後的正式 v* Release DMG 安裝 App，並依 macOS 完成首次個別允許；該 Mac 可以是開發電腦
- **THEN** App 顯示可操作的看板或專案選擇器主視窗，驗收紀錄包含產物、環境與是否為開發電腦，Dock 不持續停在啟動狀態；代理不得自行在開發電腦執行此測試

#### Scenario: 啟動持續跳動不能視為通過

- **WHEN** 簽章驗證成功，但打包 App 啟動後仍持續在 Dock 跳動、沒有主視窗
- **THEN** 驗收記為未通過，保留該安裝版的程序／系統證據並繼續調查，不宣稱問題已修復

#### Scenario: 未實跑架構如實揭露

- **WHEN** 只有 ARM64 原生啟動紀錄，Intel 僅完成架構與簽章檢查
- **THEN** 報告明列 Intel 原生啟動未驗證，不把 universal 產物的靜態檢查視為兩種 Mac 都已實機驗收
