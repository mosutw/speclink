## Problem

- 透過 AI 代理跑 SDD 的開發者、PO 與 PM，下載 macOS DMG 安裝 Speclink 後，App 持續在 Dock 跳動且沒有主視窗，無法進入討論、提案及實作流程；同一台 Mac 的 npm run dev 正常。
- 本次實際觀測為 0.12.1 universal DMG、macOS 26.6.2／ARM64。DMG 核對碼正常，原始產物與安裝副本的主程式及 Info.plist 雜湊一致，但完整 App 與各架構程式碼簽章驗證失敗；安裝版單獨啟動的樣本停在系統載入器，尚未進入應用程式主流程。

## Root Cause

- 已確認的交付缺口：Release run 42123457227 成功，但 macOS 簽章模式為 none；`.github/workflows/release.yml` 未像 `.github/workflows/desktop-build.yml` 一樣，於無 Apple 憑證時明確要求完整 ad-hoc 簽章，也沒有對最後的 DMG 內容做簽章驗收。
- 簽章缺口是否完整解釋這次載入卡住仍未證實。正式改動前先以原 DMG 的隔離副本重簽及相同方式啟動重測；若未能形成有效對照或仍卡住，保留證據、停止套用尚未證實的修復，透過 ingest 更新調查及範圍。
- 中英開發文件宣稱下載的 DMG 一律已簽章並公證，與允許無 Apple 憑證發布的現況不符。

## Proposed Solution

- 依使用者要求，實作前先建立本變更的新分支，再把討論與完整規劃提交為開發起始點；該提交完成並確認後，才執行重簽實驗或修改程式。起始點不納入既有封存 lock 檔。
- 先完成隔離副本的簽章與啟動對照，記錄產物版本、雜湊、測試環境、啟動方式、quarantine 條件、簽章結果與視窗觀察；保留原安裝與系統安全政策。
- 讓 Release 與 Desktop Build 共用小型打包設定產生器。Release 在 Apple 憑證組完全缺席時採完整 ad-hoc 簽章，完整組合仍採 Developer ID 簽章與公證；Desktop Build 保留 ad-hoc 及不產出 updater 的既有行為。部分 secrets 的失敗規則不變。
- 在兩條流程的 macOS 打包後、上傳前共用產物驗證入口，核對完整 App、DMG 內 App 與 Release updater 包內 App 的簽章、主程式與 sidecar 架構。任何驗證失敗都阻止該平台交付，不把 linker 臨時簽章視為完整 App 簽章。
- 加入打包安裝後的主視窗啟動驗收，並同步修正中英文件對 ad-hoc、Developer ID、公證及首次手動允許的描述。

## Success Criteria

- 在本次 ARM64 復現環境取得可追溯的隔離副本對照；未取得有效因果證據時不宣稱已修復 Dock 問題。
- 無 Apple 憑證時，最後交付的 App 及其巢狀程式碼完整簽章驗證成功，主程式與內附 CLI 均保留 arm64／x86_64，DMG 與 updater 包的驗證失敗會使交付失敗。
- 使用者從修正後的 DMG 安裝 App，完成 macOS 首次明確允許後能看到主視窗；簽章靜態檢查不得代替實際啟動驗收，未實跑的原生架構須明確記錄。
- 完整 Apple 憑證模式、部分設定拒絕、updater minisign、七個 Release assets、Windows／Linux 交付及一般 CI 範圍判定維持既有契約。

## Capabilities

### New Capabilities

無。

### Modified Capabilities

- `desktop-release`: 釐清無 Apple 憑證仍須完整 ad-hoc 簽章，新增最終 macOS 產物驗證與可啟動性驗收，保留可選 Developer ID 與 updater 簽章契約。
- `user-documentation`: 讓開發者入口文件的 macOS 安裝說明依實際簽章模式區分，不再把所有 DMG 描述為已公證。

## Impact

- 已掃描 desktop-release、user-documentation 及相鄰的 cli-distribution；本次沿用前兩項能力，不新增能力，CLI 安裝通路不變。
- 影響 apps/desktop 的 macOS 交付與 scripts/、CI 工作流程；不更改任何 Rust crate、Tauri command、前端功能或 Local／Remote 資料契約。
- Modified: `.github/workflows/release.yml`、`.github/workflows/desktop-build.yml`、`scripts/release/signing-gate.test.mjs`、`scripts/desktop/desktop-release.test.mjs`、`docs/development.md`、`docs/development.zh-TW.md`。
- New: `scripts/release/desktop-bundle-config.mjs`、`scripts/release/desktop-bundle-config.test.mjs`、`scripts/release/macos-artifact-verify.mjs`、`scripts/release/macos-artifact-verify.test.mjs`、`docs/development/macos-dmg-launch-investigation.md`。
- 相容性影響：沒有新增產品 CLI 動詞、旗標、stdin 或設定鍵，既有人眼、JSON、no-color、SDK 與技能生成輸出不變；新增腳本僅供 repo 打包流程使用。無 Apple 憑證的產物會從未完成封裝的臨時簽章改為完整 ad-hoc，仍不宣稱 Apple 公證，首次開啟可能需要手動允許。
- 不新增套件相依，不變更產品版號、既有發布 tag、updater endpoint／pubkey、Windows 憑證政策或 npm／Docker 通路。
