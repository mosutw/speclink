---
topic: v0.12.0 打包後，mac 安裝 .dmg 檔但是無法開啟
slug: macos-dmg-launch-failure
status: promoted
created: 2026-11-06
created_by: mosutw <luke1974@gmail.com>
promoted_to: macos-dmg-launch-failure
---

# Discussion: v0.12.0 打包後，mac 安裝 .dmg 檔但是無法開啟

<!--
Document rules:
- Rounds are appended by `speclink discuss add-round`; never rewrite an earlier round.
  A changed position gets a new round that names what changed and why.
- Each round distills one focus question: **Focus** / **Position** / **Ruled out** / **Open**.
- The conclusion must resolve or explicitly defer every open question left by the rounds.
-->

## Context

使用者回報 macOS 以 DMG 安裝後無法開啟，進一步確認為 Dock 圖示持續跳動、主視窗未出現，而 npm run dev 正常。原始描述稱 v0.12.0；本機實際找到的下載檔為 /Users/mosu/Downloads/software/AI/Speclink_0.12.1_universal.dmg，/Applications/Speclink.app 的版號亦為 0.12.1，環境為 macOS 26.6.2／ARM64。本次先釐清失敗位置及已發佈安裝檔的啟動條件，尚不修改應用程式碼。
相關正式規格為 desktop-release，既有要求包含 universal DMG 可在兩種 Mac 晶片啟動、OS 簽章與公證可由 secrets 啟用，以及缺 OS 簽章時允許發布未簽章產物；目前沒有進行中的變更。先前討論分別決定 universal 安裝檔及正式發佈的簽章方向，現行規格與文件對「未簽章亦可發布」和「已簽章並公證」存在需要核對的落差。
Prior discussions: release-assets-trim, release-first-and-distribution

## Rounds

<!-- `### Round N — <mode> (<date>)` entries are appended here by the CLI. -->

### Round 1 — interview (2026-11-06)

**Focus**: 無法開啟發生於 DMG 映像掛載，還是安裝後 App 啟動？
**Position**: 使用者確認為安裝後 App 的 Dock 圖示持續跳動，主視窗未出現。
- 本機找到 Speclink_0.12.1_universal.dmg，hdiutil verify 回報映像核對碼 VALID；/Applications/Speclink.app 版號亦為 0.12.1。
- 問題應沿安裝版啟動路徑調查，尚不能把視窗未出現直接歸因於前端或 Rust 初始化。
**Ruled out**: 優先將 DMG 核對碼損壞視為根因——目前映像驗證通過。
**Open**: 原始描述的 0.12.0 與本機 0.12.1 是否指同一批整合；App 是否被系統啟動檢查攔住；原始 DMG 與已安裝副本是否一致。

### Round 2 — interview (2026-11-06)

**Focus**: npm run dev 正常，是否能縮小安裝版啟動失敗的範圍？
**Position**: 使用者確認開發模式正常，因此優先檢查安裝包與系統載入路徑，暫不判定應用程式初始化死鎖。
- ps 顯示目前唯一 speclink-desktop 程序為 repo/target/debug/speclink-desktop、父程序為 node；其 sample 堆疊是正常事件迴圈，不能當成安裝版卡住的證據。
- 安裝版主程式顯示 Signature=adhoc、TeamIdentifier=not set、Sealed Resources=none；codesign --verify --deep --strict 回報 code object is not signed at all。
- 排除 sandbox 干擾後，spctl --assess 回報 rejected、source=no usable signature；App 仍有來自 Chrome 的 quarantine 屬性。
- 發版流程的 signing-gate.mjs 允許 Apple 簽章組完全缺席時繼續發布；docs/development.zh-TW.md 卻描述正式 DMG 已簽章並經 Apple 公證，需釐清實際產物及交付承諾。
**Ruled out**: 使用開發程序的堆疊判定安裝版死鎖——程序來源不同；僅憑 Dock 跳動便修改前端或停用外掛——尚無相應證據。
**Open**: 原始 DMG 內 App 與已安裝副本的簽章及內容是否一致；系統啟動日誌能否直接對上 Dock 跳動；發布要求要維持可選 OS 簽章，或改為正式 macOS 安裝檔必須簽章與公證；原始版本描述仍待對齊。

### Round 3 — assumptions (2026-11-06)

**Focus**: 簽章異常是在 DMG 產物內就存在，還是安裝後才出現？
**Position**: 原始 DMG 內的 App 已缺少完整 bundle 簽章，安裝後的主程式與 Info.plist 沒有改變。
- 以 hdiutil -readonly -nobrowse 只讀掛載下載檔；DMG 內 App 同樣顯示 Signature=adhoc、Info.plist=not bound、Sealed Resources=none，codesign --verify --deep --strict 同樣失敗。
- DMG 與 /Applications 的主程式 SHA-256 均為 fbdfdbc29747e6a6e3237b2e112efd85d2620c667012e2872f8d6de70a5173fe，Info.plist SHA-256 均為 93b1cd2b8dffedbe9f69b18f19fc5629b804a1742daac59e14abc9b5795be72d。
- 系統日誌曾列出 /Applications/Speclink.app 啟動後終止，但不能僅憑退出狀態區分使用者關閉、系統中止或程式問題。目前可取樣的唯一程序仍是開發版，尚未取得卡住安裝版的有效堆疊。
- Apple 官方發佈文件要求在預設 Gatekeeper 設定下以 Developer ID 簽章並公證；現有流程另允許無 Apple secrets 時發布未簽章產物，交付政策與無需放行的文件承諾須後續討論。來源：https://support.apple.com/en-ie/guide/security/sec3ad8e6e53/web 、https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution 。
**Ruled out**: 只把這次簽章異常歸咎於安裝時複製破壞——原始 DMG 已呈現相同狀態；宣稱 Dock 卡住的所有根因已定位——尚缺獨立安裝版的啟動觀測。
**Open**: 關閉開發模式後單獨啟動安裝版是否仍持續跳動；若持續，需取得該次安裝版程序或系統啟動拒絕證據；正式 macOS 發佈是否要求完整 Developer ID 簽章與公證；版本描述差異仍待對齊。

### Round 4 — assumptions (2026-11-06)

**Focus**: 排除開發模式後，安裝版卡住的位置與此次 Release 的簽章模式為何？
**Position**: 安裝版單獨執行仍卡在程式啟動前的系統載入階段，無 Apple 憑證的 Release 簽章路徑是目前優先修正與驗證的方向。
- 使用者已關閉 npm run dev，單獨從應用程式開啟後仍持續跳動；因此不能用同時執行開發版解釋此症狀。
- ps 取得 PID 93960，執行檔為 /Applications/Speclink.app/Contents/MacOS/speclink-desktop、父程序為 launchd。三秒 sample 的 2574 個主執行緒樣本全在 _dyld_start +0，實體記憶體僅 112K、無 binary images 描述，尚未看到 Rust main 或 Tauri 初始化。
- codesign --verify --all-architectures --strict 對主程式亦回報 code object is not signed at all，不能把 codesign -dv 的 linker ad-hoc 資訊視為完整簽章驗證成功。
- GitHub Release run 42123457227 的 universal macOS desktop job 127581485310 成功，但實際日誌為 macOS 簽章：none；現行 release.yml 只在 full 模式注入 Apple 簽章與公證參數。
- 既有 desktop-build.yml 在 macOS 明確設定 bundle.macOS.signingIdentity 為 '-'，可作為無 Apple 憑證模式的既有實作參照；建議優先對齊這條路徑並加入完整產物簽章及啟動驗收。此方向符合 desktop-release 的可選 OS 簽章政策。
**Ruled out**: 開發版同時執行是必要觸發條件——使用者單獨啟動仍重現；把修正先落在前端或 Tauri 初始化——目前取樣未到該階段；僅憑 Release job 成功宣稱產物可啟動——本次成功 job 已產出無法通過簽章驗證的原始 App。
**Open**: 使用者是否保留無 Apple 憑證的交付模式（需明確首次手動允許說明），或將正式 macOS 產物改為強制 Developer ID 簽章與公證；重簽隔離副本後是否能正常載入，以驗證因果；若仍失敗，需繼續檢查載入器與執行檔問題，不能宣稱簽章是唯一根因。

### Round 5 — interview (2026-11-06)

**Focus**: 本次修正是否要求正式 macOS DMG 一律使用 Developer ID 簽章與公證？
**Position**: 使用者選擇先修無 Apple 憑證的打包路徑，保留目前可選 Apple 簽章的發佈政策。
- 無完整 Apple 憑證組時，目標是產出完整 ad-hoc 簽章且驗證通過的 App，並確認打包後真的能啟動；首次開啟仍可能需要使用者在 macOS 明確允許。
- 已有完整 Apple 憑證組時，仍沿用 Developer ID 簽章與公證；部分 secrets 設定必須繼續失敗，不降級為成功。
- 先用原 DMG 的隔離副本重簽及啟動重測來確認因果，再修改正式打包流程；若仍卡在系統載入器，繼續調查，不把目前簽章異常當成唯一根因。
**Ruled out**: 本次強制購置或提供 Apple Developer ID 憑證——使用者選擇先修無憑證路徑；以全域停用 Gatekeeper 或永久移除 quarantine 當作交付修正——未修正產物品質與驗收缺口。
**Open**: 重簽副本能否正常啟動、正式打包實作與各架構驗收留待變更設計及實作驗證；全面強制 Developer ID 與公證另案。

## Conclusion

**Decision**: 先修正無 Apple 憑證的 macOS 打包路徑，保留現有可選 Apple 簽章政策，並以打包產物可正常啟動作為驗收目標。
- 復現基準採本機實際下載的 Speclink_0.12.1_universal.dmg、/Applications/Speclink.app、macOS 26.6.2／ARM64；開發模式正常，安裝版單獨啟動仍持續在 Dock 跳動，實際樣本停於 _dyld_start，尚未進入應用程式主流程。
- 先將原 DMG 的 App 複製到隔離位置，對副本完成 ad-hoc 簽章後重測，確認能否解除系統載入卡住；原安裝與系統安全設定不作為實驗修改對象。若副本仍卡住，繼續調查載入器及執行檔問題，不能僅憑現有簽章異常宣稱唯一根因已確認。
- 無 Apple 簽章組時，參照現有 desktop-build.yml 的 bundle.macOS.signingIdentity='-' 做法，讓 Release 產出具有完整 ad-hoc 簽章的 App，涵蓋主程式、內附 CLI、巢狀程式與資源，並在 universal 組裝及內容修改完成後驗證。
- 完整 Apple 簽章組仍走既有 Developer ID 簽章與 Apple 公證路徑；部分設定繼續失敗，不能默默降級。Updater 的 minisign 簽章、更新端點與公鑰維持原契約，ad-hoc 不取代 updater 來源驗證。
- 增加打包完成後的簽章檢查：驗證完整 App 與 universal 各架構程式碼，避免 Release 成功但實際產物只有未完成 bundle 封裝的 linker 臨時簽章；失敗時阻止發布。
- 增加安裝包啟動驗收，至少以本次 ARM64 環境重現與回歸；保留既有 universal 支援 Apple Silicon 與 Intel 的要求。原生環境尚未實跑的部分須明確記錄，不能以靜態簽章或架構檢查冒充成功啟動。
- 無 Apple 憑證產物不宣稱已通過 Developer ID 簽章或 Apple 公證，使用說明須寫明首次開啟可能需要在 macOS 手動允許；同步修正現行中英文件對正式 DMG 一律已簽章並公證的錯誤承諾。
- 驗收例：Apple 憑證組完全未設定時，產出 universal DMG，內含 App 的完整 ad-hoc 簽章驗證通過；使用者拖入應用程式並完成首次明確允許後，App 能顯示主視窗，不再無限停留在 Dock 啟動狀態。
- 本次以發版打包與產物驗收為優先範圍；除非後續證據證明需要，不擴大修改前端、Tauri 外掛或業務功能。
**Rationale**: 使用者選擇保留不依賴 Apple 憑證的交付方式；實際原始 DMG 與已安裝副本存在相同簽章驗證缺口，成功的 Release 日誌亦顯示 macOS 簽章為 none，而安裝版實際停在應用程式執行前的載入階段。先以隔離副本驗證因果，再沿用現成 ad-hoc 打包方式補驗收，可避免把尚未證實的啟動假說擴大成應用程式重寫。
**Rejected alternatives**:
- 本次強制 Developer ID 簽章與公證 — 使用者明確選擇先修無 Apple 憑證路徑，完整憑證模式保留但不成為此次修正前提。
- 全域停用 Gatekeeper 或將移除 quarantine 作為永久交付方案 — 未修正產物本身及發佈驗收缺口，也掩蓋下載後的真實安裝條件。
- 先修改前端或停用啟動外掛 — 目前安裝版樣本尚未進入該階段，缺乏相應證據。
- 只補文件或只確認 CI 建置成功 — 已存在實際不可正常開啟且簽章驗證不通過的打包產物，需要可重現的產物與啟動驗收。
**Deferred**:
- 全面要求正式 macOS 發布必須 Developer ID 簽章及公證 — 本次維持可選政策，待憑證與交付政策另案決定。
- 簽章缺口是否完整解釋載入卡住 — 由後續變更的隔離副本重簽實驗與實際啟動驗收確認，現在不宣稱已證實。
- 原始描述所稱獨立 v0.12.0 安裝檔的問題 — 目前實際取得與觀測的是 v0.12.1，若另有不同產物需另行比對。
**Capture to**: proposal, design, spec, tasks
**Next**: speclink-propose --from-discussion macos-dmg-launch-failure
