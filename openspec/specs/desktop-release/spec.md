# desktop-release Specification

## Purpose

桌面 app 的發版與更新交付：release 產出三平台安裝檔、隨版發布的更新描述檔與更新包簽章，以及 OS 程式碼簽章這把可插可拔的鑰匙開關。本 capability 保證自動更新的來源可驗證，且本機安裝帶版號新鮮度斷言——裝到舊版會當場被擋下，而非默默沿用。

## Requirements

### Requirement: Release 產出三平台桌面安裝檔

push 符合 v* 的 tag 後，release 管線 SHALL 產出桌面安裝檔並附於同一個 GitHub Release，且該 Release 的 assets SHALL 恰為下列七個檔案：macOS universal dmg（Apple Silicon 與 Intel 同一檔，檔名 Speclink_<版本>_universal.dmg）、macOS universal 更新包（Speclink_<版本>_universal.app.tar.gz）、Windows NSIS 安裝器（x86_64，Speclink_<版本>_x64-setup.exe）、Windows x64 免安裝包（Speclink_<版本>_x64-portable.zip）、Linux AppImage（x86_64 為 Speclink_<版本>_amd64.AppImage、aarch64 為 Speclink_<版本>_aarch64.AppImage）、更新描述檔 latest.json。Release assets SHALL NOT 含 .sig 簽章檔、.deb、CLI 壓縮檔、SHA256SUMS.txt 或 server 壓縮檔（CLI 通路見 cli-distribution，server 發布通路見 server-release）。Windows 免安裝 ZIP SHALL 包含同版桌面程式與 CLI，並通過解壓縮、CLI 設定寫入及搬移後設定仍隨資料夾的檢查。桌面安裝檔 SHALL 內含同版 speclink CLI binary，macOS 內含的 CLI SHALL 為含 arm64 與 x86_64 兩個架構的 universal binary。Windows NSIS 安裝器 SHALL 內建繁體中文與英文語系並跟隨系統顯示語言（繁中系統顯示繁中、其餘落回英文），安裝器圖示 SHALL 為 Speclink 產品圖示。

#### Scenario: tag 發布產出完整安裝檔集

- **WHEN** push tag v0.5.0 且 workflow 全部成功
- **THEN** 該 Release 的 assets 恰為 Speclink_0.5.0_universal.dmg、Speclink_0.5.0_universal.app.tar.gz、Speclink_0.5.0_x64-setup.exe、Speclink_0.5.0_x64-portable.zip、Speclink_0.5.0_amd64.AppImage、Speclink_0.5.0_aarch64.AppImage 與 latest.json 七個，無任何 .sig、.deb、CLI／server 壓縮檔或 SHA256SUMS.txt

#### Scenario: Windows portable ZIP 可解壓並可搬移

- **WHEN** Windows x64 portable ZIP 從 Release 下載、解壓縮並執行 CLI，再將資料夾搬到另一個位置
- **THEN** 桌面程式與同版 CLI 存在，CLI 設定寫入 ZIP 目錄內，搬移後仍從搬移後目錄讀取設定

#### Scenario: 任一形態失敗則不發布

- **WHEN** 桌面安裝檔任一 target 建置失敗
- **THEN** GitHub Release SHALL NOT 建立（與既有 Docker gating 同一 needs 閘門），不產生缺形態的 Release

#### Scenario: 繁中系統的安裝精靈顯示繁中

- **WHEN** 在顯示語言為繁體中文的 Windows 上執行 NSIS 安裝器
- **THEN** 安裝精靈以繁體中文顯示，安裝檔圖示為 Speclink 產品圖示；非內建語系的系統落回英文

#### Scenario: universal dmg 在兩種晶片皆可用

- **WHEN** 在 Apple Silicon 與 Intel 的 Mac 上分別以同一個 universal dmg 安裝並啟動 app
- **THEN** 兩台皆能啟動，且 app 內含的 speclink CLI 以 lipo -info 檢視含 arm64 與 x86_64 兩個架構


<!-- @trace
source: release-assets-trim
updated: 2026-09-14T21:53:29+08:00
-->

---
### Requirement: 更新描述檔隨 release 發布

release 管線 SHALL 組裝更新描述檔 latest.json 附於 Release assets：version 欄位等於 tag 去除 v 前綴、pub_date 為發布時間、platforms 物件至少含 darwin-aarch64、darwin-x86_64、windows-x86_64、linux-x86_64 四鍵，各鍵含 url（指向同一 Release 的對應更新包 asset）與 signature（該更新包的簽章內容）；darwin-aarch64 與 darwin-x86_64 兩鍵 SHALL 指向同一個 universal 更新包且 signature 相同。組裝腳本的輸入 SHALL 為「每個更新包目錄對應一或多個平台鍵」：必要目錄 darwin-universal（對應 darwin 兩鍵）、windows-x86_64、linux-x86_64，可選目錄 linux-aarch64；每個目錄恰含一個更新包與同名 .sig。缺任一必要目錄、缺更新包或缺簽章時 SHALL 以非零結束且不寫出描述檔。組裝邏輯 SHALL 有可獨立執行的單元測試。

#### Scenario: 描述檔欄位對齊發布內容

- **WHEN** push tag v0.5.0 且 workflow 成功
- **THEN** latest.json 的 version 為 0.5.0，且 platforms 每一鍵的 url 均指向本次 Release 的 asset 下載路徑

##### Example: v0.5.0 的 platforms 對應

| platforms 鍵 | url 指向的 asset |
| ------------ | ---------------- |
| darwin-aarch64 | v0.5.0 Release 中的 Speclink_0.5.0_universal.app.tar.gz |
| darwin-x86_64 | 與 darwin-aarch64 相同的 url，signature 亦相同 |
| windows-x86_64 | v0.5.0 Release 中的 Speclink_0.5.0_x64-setup.exe |
| linux-x86_64 | v0.5.0 Release 中的 Speclink_0.5.0_amd64.AppImage |

#### Scenario: 更新端點可匿名讀取

- **WHEN** 以未帶認證的 HTTP GET 請求 releases/latest/download/latest.json 路徑
- **THEN** 回應為可解析的 JSON 且內容為最新 Release 的更新描述檔

#### Scenario: 缺更新包目錄即不組裝

- **WHEN** 組裝腳本的輸入缺 darwin-universal 目錄，或該目錄有更新包但缺同名 .sig
- **THEN** 腳本以非零結束並點名缺失的目錄或簽章，不寫出 latest.json


<!-- @trace
source: release-assets-trim
updated: 2026-09-14T21:53:29+08:00
-->

---
### Requirement: 更新包簽章

每個平台的更新包 SHALL 以專案自產的 updater 金鑰（minisign）簽章；公鑰 SHALL 隨版本庫提交於桌面設定檔，私鑰與其密語 SHALL 只存於 CI secrets。CI 缺少私鑰 secrets 時桌面建置 SHALL 失敗（fail-closed），不得產出無簽章的更新包。

#### Scenario: 缺私鑰時建置失敗

- **WHEN** CI 環境沒有 updater 私鑰 secret 而執行桌面建置
- **THEN** 建置以非零結束，workflow 標紅，Release 不發布

#### Scenario: 簽章隨更新包產出

- **WHEN** 桌面建置成功
- **THEN** 每個更新包旁存在對應簽章，且其內容與 latest.json 中該平台的 signature 一致


<!-- @trace
source: desktop-installer-and-updater
updated: 2026-07-30
code:
  - .github/workflows/release.yml
  - Cargo.lock
  - apps/desktop/package.json
  - apps/desktop/src-tauri/Cargo.toml
  - apps/desktop/src-tauri/capabilities/default.json
  - apps/desktop/src-tauri/src/cli_install.rs
  - apps/desktop/src-tauri/src/lib.rs
  - apps/desktop/src-tauri/tauri.conf.json
  - apps/desktop/src-tauri/windows/hooks.nsh
  - apps/desktop/src/App.tsx
  - apps/desktop/src/__tests__/App.test.tsx
  - apps/desktop/src/__tests__/appSettingsView.test.tsx
  - apps/desktop/src/__tests__/cliInstall.test.ts
  - apps/desktop/src/__tests__/updateBanner.test.tsx
  - apps/desktop/src/__tests__/updater.test.ts
  - apps/desktop/src/adapter/cliInstall.ts
  - apps/desktop/src/adapter/updater.ts
  - apps/desktop/src/components/UpdateBanner.tsx
  - apps/desktop/src/core/cliInstall.ts
  - apps/desktop/src/core/updater.ts
  - apps/desktop/src/i18n/messages.ts
  - apps/desktop/src/main.tsx
  - apps/desktop/src/store.ts
  - apps/desktop/src/views/AppSettingsView.tsx
  - package-lock.json
  - scripts/desktop-sidecar.mjs
  - scripts/release-latest-json.mjs
  - scripts/release-latest-json.test.mjs
-->

---
### Requirement: OS 程式碼簽章為可插鑰匙開關

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


<!-- @trace
source: macos-dmg-launch-failure
updated: 2026-11-07T21:21:51+08:00
-->

---
### Requirement: 本機安裝的新鮮度斷言

repo SHALL 提供本機建置安裝入口（node scripts/desktop/desktop-install.mjs），把桌面 app 的本機建置與安裝收成單一指令並以版號斷言取代信任。執行時 SHALL 依序：(0) 進入任何步驟之前先行把關——帶 `--install` 而平台非 macOS，或簽章環境變數缺失時，SHALL 單行錯誤（點名平台或缺失的變數名）並以非零結束，SHALL NOT 進入任何建置；(1) 印出當前 HEAD、分支、工作樹是否乾淨與源碼的產物層版號；(2) 執行 sidecar 佈署（永遠重新建置，SHALL NOT 沿用 src-tauri/binaries/ 既有檔案）；(3) 前端建置與 tauri bundle；(4) 斷言 bundle 內 sidecar CLI 的引擎版號等於源碼產物層版號，不等時 SHALL 印出兩邊版號並以非零結束。帶 `--install` 時 SHALL 續行：(5) 確認 app 未執行——執行中 SHALL 單行錯誤停止，SHALL NOT 代為結束程序；(6) 覆蓋 /Applications 安裝，SHALL 先將新版完整佈到暫存路徑再換上，使拷貝失敗時既有安裝原封不動；(7) 斷言安裝版 CLI 的引擎版號同版，不等時 SHALL 印出兩邊版號並以非零結束。安裝步驟（5–7）僅支援 macOS；建置步驟（1–4）平台中立。任一步驟失敗 SHALL 以非零結束且 SHALL NOT 繼續後續步驟。

#### Scenario: 建置並通過 bundle 斷言

- **WHEN** 於簽章環境變數齊備的源碼樹執行 node scripts/desktop/desktop-install.mjs
- **THEN** 依序輸出 HEAD 與源碼產物層版號、重建 sidecar、完成 bundle，並以 bundle 內 CLI 的引擎版號等於源碼版號通過斷言，exit code 0

#### Scenario: bundle 版號不符即失敗

- **WHEN** bundle 內 sidecar CLI 的引擎版號與源碼產物層版號不等（如建置鏈沿用了過期 binary）
- **THEN** 印出兩邊版號、exit code 非零、不進行安裝

#### Scenario: 安裝後斷言安裝版同版

- **WHEN** 帶 --install 執行且 app 未執行、建置斷言通過
- **THEN** 覆蓋安裝後以安裝版 CLI 的引擎版號同版通過第二道斷言，exit code 0

#### Scenario: app 執行中拒絕安裝

- **WHEN** 帶 --install 執行而 Speclink app 程序仍在執行
- **THEN** 單行錯誤說明需先關閉 app、exit code 非零、/Applications 零變動

#### Scenario: 簽章環境變數缺失即停

- **WHEN** 簽章環境變數未設定時執行 node scripts/desktop/desktop-install.mjs
- **THEN** 單行錯誤指出缺失的變數名、exit code 非零、不進入建置（含 sidecar 重建）

#### Scenario: 非 macOS 帶 --install 即停

- **WHEN** 於非 macOS 平台執行 node scripts/desktop/desktop-install.mjs --install
- **THEN** 單行錯誤指出平台、exit code 非零、不進入建置

<!-- @trace
source: instruction-downgrade-guard
updated: 2026-08-06
-->

---
### Requirement: Release 說明含下載指南

release job SHALL 以腳本產生雙語的 Release 說明：開頭為英文下載指南，其後接該版的英文更新日誌片段；再其後為一個預設收合的 `<details>` 區塊，`<summary>` 為「繁體中文」，區塊內依序為繁中下載指南與繁中更新日誌片段；GitHub 自動產生的比較連結接續於最後。兩種語言的片段 SHALL 由 release-notes 的渲染腳本自 release-notes.json 畫出。每種語言的指南 SHALL 含：一般使用者安裝檔對照表（macOS 一列指向 universal dmg 並註明 Apple Silicon 與 Intel 同一檔、Windows NSIS 安裝器一列、Linux 桌面機依架構各一 AppImage、Linux 伺服器或無圖形介面一列指向 CLI 一行安裝而非任何檔案；檔名含該版號且與資產命名一致，兩種語言的檔名 SHALL 逐字相同）、CLI 的三條一行指令（npm 全域安裝、安裝腳本 curl 一行、Homebrew）、server 的一行啟動節（npx 指令與 Docker image 一行，寫法比照 CLI 安裝節），並 SHALL 註明更新包（.app.tar.gz）與 latest.json 為自動更新機制使用、毋須手動下載。指南 SHALL NOT 提及 .sig、.deb、SHA256SUMS.txt 或 CLI 壓縮檔。產生器 SHALL 於 tag 不符 vX.Y.Z 格式時以非零結束，SHALL 於 release-notes.json 沒有該版條目時以非零結束且 stdout 保持空白，且 SHALL 有驗證兩種語言輸出內容的自動化測試。

#### Scenario: 發布後 Release 頁開頭為下載指南

- **WHEN** push tag 完成發布後檢視該 GitHub Release 頁
- **THEN** 說明開頭為英文的四列安裝檔對照表、三條 CLI 安裝指令與 server 一行啟動節（npx 與 Docker），.app.tar.gz 與 latest.json 標註為自動更新機制使用，其後為英文更新日誌片段（`## X.Y.Z (YYYY-MM-DD)` 與分組條目）；再其後為收合的「繁體中文」區塊，展開後為繁中指南與繁中片段；自動產生的比較連結位於最後

#### Scenario: 指南檔名對齊版號與資產命名

- **WHEN** 以 tag vX.Y.Z 執行下載指南產生器
- **THEN** 英文與繁中兩段輸出中的每個檔名都含該版號、彼此逐字相同，且與 release 管線對應平台的資產命名一致

#### Scenario: JSON 缺該版時產生器失敗

- **WHEN** 以 tag v0.3.0 執行產生器而 release-notes.json 沒有 0.3.0 條目
- **THEN** 產生器以非零結束並於 stderr 說明，Release 說明不產生


<!-- @trace
source: bilingual-release-notes
updated: 2026-10-15T17:28:24+08:00
-->

---
### Requirement: 正式 Release 等待同 commit CI

一般 `v*` 正式 Release SHALL 在任何 build、資產上傳或發布工作開始前，等待 repo 預設分支上與 tag 指向完全相同 commit SHA 的 push CI 完成並成功。Release SHALL 重用既有 `scripts/desktop/desktop-ci-gate.mjs` 的成功判定；CI 失敗、SHA 不符或 40 分鐘內沒有成功結果時，gate SHALL 以非零結束並阻止 build、desktop、docker、engine-version 等獨立建置工作與其後的發布工作。通過 gate 後 SHALL 保留 `.github/workflows/release.yml` 原本的 `v*` 發布通路、DMG 與 updater 資產契約；SHALL NOT 改走只產出獨立桌面資產的 `desktop-v*` 通路。

#### Scenario: 同 commit CI 成功後才建置正式 Release

- **WHEN** `v*` tag 指向預設分支上的 commit，且該 SHA 的最新 push CI 已完成並成功
- **THEN** gate 成功，原有正式 Release jobs 才開始，DMG 與 updater 維持原資產通路

#### Scenario: CI 未成功時阻止 Release

- **WHEN** 同 SHA CI 失敗、回報其他 SHA，或等待 40 分鐘仍沒有成功結果
- **THEN** gate 以非零結束，打包與所有發布工作不執行，Release 不產生資產

<!-- @trace
source: macos-dmg-launch-failure
updated: 2026-11-07T21:21:51+08:00
-->

---
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

<!-- @trace
source: macos-dmg-launch-failure
updated: 2026-11-07T21:21:51+08:00
-->

---
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

<!-- @trace
source: macos-dmg-launch-failure
updated: 2026-11-07T21:21:51+08:00
-->

---
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

<!-- @trace
source: macos-dmg-launch-failure
updated: 2026-11-07T21:21:51+08:00
-->