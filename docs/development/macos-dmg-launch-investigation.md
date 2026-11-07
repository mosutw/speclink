# macOS DMG 啟動問題調查

## 調查狀態

Dock 問題的唯一根因仍未證實。使用者要求代理不得在本開發電腦做測試實驗，因此代理不執行本機 DMG 掛載、App 重簽或啟動。已確認的 Release 缺口由 GitHub Actions 的 CI 與正式 Release 產物檢查處理；使用者其後自行在同一台 Mac 執行 GUI 啟動驗收，並決定接受該結果。

## 環境與產物

- 環境：macOS 26.6.2（25G83），Apple Silicon（arm64）。
- 實際檢查的下載檔：`/Users/mosu/Downloads/software/AI/Speclink_0.12.1_universal.dmg`，版號為 0.12.1；這不是 0.12.0 的原始產物。
- `hdiutil verify` 回報 DMG checksum 有效。先前唯讀檢查中，DMG 內 App 與 `/Applications/Speclink.app` 的主程式、`Info.plist` 雜湊相同；未見容器損壞證據。
- 安裝版主程式為 universal（x86_64、arm64），`codesign -dv` 顯示 `Signature=adhoc`、`linker-signed`、無 Team ID。完整嚴格簽章驗證失敗；這項結果表示 linker 簽章不等於完整 App 簽章，不能單獨證明 Dock 問題的根因。
- 在 `/private/tmp/speclink-dmg-debug.IwLw7o/` 建立兩份暫存副本並保留 quarantine：`original/Speclink.app` 為原樣副本，`resigned/Speclink.app` 以 `codesign --force --deep --sign -` 重簽。重簽副本的 `codesign --verify --deep --strict --all-architectures` 通過；主程式雜湊與原樣副本不同。
- 沒有啟動這兩份暫存 App，也沒有取得重簽副本的主視窗觀察。使用者明確選擇停止實驗；之後不得把靜態驗證當成啟動成功。
- 唯讀掛載的權限升級請求遭拒，故本輪沒有重新掛載 DMG。原安裝、quarantine 與 macOS 安全設定未被修改。

## CI 與 Release 流程檢查

- `.github/workflows/desktop-build.yml` 的 prepare job 會呼叫 `scripts/desktop/desktop-ci-gate.mjs`，等候預設分支上同一 commit SHA 的 push CI 成功，再建 universal `app,dmg`；但它只接受 `desktop-v*` tag，發布獨立桌面資產且不產生正式 Release updater。
- `.github/workflows/release.yml` 的一般 `v*` workflow 原本沒有等待同一 SHA 主 CI；目前分支已新增前置 gate，並讓 build、desktop、docker、engine-version 依賴它。正式 Release、npm、Docker 與其他發布步驟沿既有依賴鏈等待建置完成。
- 目前分支已讓正式 Release 與 Desktop Build 共用打包設定：沒有 Apple 憑證時明確傳 `signingIdentity: '-'`，完成 ad-hoc 簽章；有憑證時不覆寫 Tauri 設定。macOS runner 在上傳前會檢查 App、DMG 內容及正式 Release updater archive 內的 App。
- Tauri 官方說明指出 notarization/stapler 處理的是 App bundle，不能推定 `.dmg` 也附有公證票證；因此驗證器檢查 App 的 Gatekeeper 與 stapler 結果，並用 `hdiutil verify` 檢查 DMG 映像完整性。[Tauri macOS 簽章說明](https://v2.tauri.app/distribute/sign/macos/)。
- PR #18 的 CI run [42247712018](https://github.com/mosutw/speclink/actions/runs/42247712018) 在提交 `8aae41662165878422e285b21af8f5c3c20ea70d` 上通過；macOS 工作於第二次嘗試通過，Windows、Linux 與其他工作亦通過。這次 PR CI 執行的是腳本測試與一般建置測試，並未執行正式 Release 的 DMG 與 updater 原生產物驗證。
- 補充的容器異常與清理測試，以及 Linux 的 Speclink 分析和嚴格驗證，由 PR #19 的 [CI run 42338204607](https://github.com/mosutw/speclink/actions/runs/42338204607) 執行。原規劃的實作前失敗紀錄未留存，後補測試不能視為該歷史紀錄。
- 正式 [v0.12.2 Release](https://github.com/mosutw/speclink/releases/tag/v0.12.2) 的 [GitHub Actions run 42331853628](https://github.com/mosutw/speclink/actions/runs/42331853628) 通過；macOS runner 的驗證紀錄為 `channel=release mode=none version=0.12.2 artifacts=app,dmg,updater`。它使用正式建置的 App、DMG 與 updater 產物，並完成原生簽章、架構及容器內容檢查。
- 使用者回報從正式 v0.12.2 Release 下載 `Speclink_0.12.2_universal.dmg`，在同一台開發電腦（Apple M1 Pro、macOS 26.6.2、arm64）關閉 `npm run dev` 後安裝；首次於系統設定按「強制打開」，之後可看到可操作主畫面且 Dock 不再持續跳動。使用者自行執行並勾選驗收，且明確接受同機結果。真實 Developer ID／公證模式及 Intel 原生啟動尚無驗證紀錄；代理仍不得在本機執行測試、掛載 DMG、重簽或啟動實驗。

## 同類位置盤點與處理範圍

| 位置 | 本次處理 |
| --- | --- |
| `.github/workflows/release.yml` | 已加入同 SHA CI gate、共用打包設定及 macOS 產物上傳前驗證；完整 Apple 憑證路徑維持原樣。 |
| `.github/workflows/desktop-build.yml` | 改為使用共用 ad-hoc 打包設定及上傳前驗證；保留不產生 updater 的行為與來源資訊。 |
| `scripts/release/signing-gate.mjs` | 保留完整／缺席／部分憑證的既有決策；部分設定仍須失敗，updater 簽章仍獨立處理。 |
| `scripts/desktop/desktop-sidecar.mjs` | 保留 CLI sidecar 建置、universal 合併及部署規則；新驗證器逐架構檢查並比對 sidecar。 |
| `scripts/desktop/desktop-install.mjs` | 不改本機安裝入口既有 `TAURI_SIGNING_PRIVATE_KEY` 與密碼前置檢查、版號斷言及安裝流程。 |
| `README.md`、`README.zh-TW.md` | 本次盤點未發現 macOS 下載簽章或公證承諾；既有 Windows 未簽章說明不在本次範圍。 |
| `docs/development.md`、`docs/development.zh-TW.md` | 已依 ad-hoc 與 Developer ID／公證模式區分說明首次個別允許，沒有宣稱所有 macOS 下載檔都已公證。 |

不擴大 updater、更新端點、公鑰、發佈資產或跨平台交付範圍。CI 產物驗證確認打包與簽章契約；使用者的同機啟動驗收成功，但簽章缺口是否為原問題唯一根因仍未證實。
