## 0. 實作起點核對

- [ ] 0.1 驗證 D0「實作分支與起始點提交」已先完成：本次先建立 fix/macos-dmg-launch-failure，並以主旨「chore(macos): 記錄 DMG 啟動修正開發起始點」提交來源討論與完整規劃，之後才執行 D1 重簽實驗或改程式；已建立的分支與提交驗證沿用，不重設。範圍 openspec/discussions/macos-dmg-launch-failure.md、openspec/changes/macos-dmg-launch-failure/。驗證：git branch --show-current 確認不在 main，git log 取得起始點 SHA、git merge-base 確認其為目前 HEAD 祖先，git status 確認規劃文件已受版控且未混入既有封存 lock 檔；把實際 SHA 記入任務完成證據，再進行 1.1。 <!-- speclink-task:tsk_01M6RJZ98SCSMNWWZ9QFSZGWQV -->

## 1. 因果對照與同類位置盤點

- [ ] 1.1 先完成 D1「隔離副本因果驗證」：在實作分支核對原 0.12.1 DMG 與 App 雜湊，只對獨立暫存副本做完整 ad-hoc 重簽，對照相同啟動方式、quarantine／允許條件下的原樣卡住與重簽後主視窗結果；不改原安裝、其他程序或系統政策。範圍 docs/development/macos-dmg-launch-investigation.md、.github/workflows/release.yml。驗證：記錄 hdiutil、codesign、sample 與視窗觀察的命令／摘要、環境及來源；無有效對照或重簽仍卡住即保留結果並停止後續程式碼工作，轉 ingest 更新原因與範圍，不能把未驗證算成完成。 <!-- speclink-task:tsk_01M6RJJBGF9P3H4VKFN3QEX3QC -->
- [ ] 1.2 依 D5「文件與回歸邊界」排查同類位置：rg 搜尋 signingIdentity、SPECLINK_MACOS_SIGNING、codesign、notariz、Gatekeeper、簽章及公證；盤點 .github/workflows/release.yml、.github/workflows/desktop-build.yml、scripts/release/signing-gate.mjs、scripts/desktop/desktop-sidecar.mjs、scripts/desktop/desktop-install.mjs、README.md、README.zh-TW.md、docs/development.md、docs/development.zh-TW.md。驗證：在 docs/development/macos-dmg-launch-investigation.md 逐項寫明採修或保留理由，標明本機安裝既有簽章前置不改、updater 與發佈通路不擴大。 <!-- speclink-task:tsk_01M6RJJBGGJ9KE2JCZHZYJVGJ0 -->

## 2. 共用打包設定的 Red／Green

- [ ] 2.1 先為 D2「共用桌面打包設定」與「OS 程式碼簽章為可插鑰匙開關」建立失敗案例：Release macOS none／full、非 macOS、Desktop Build 三平台、未知 channel／模式、JSON camelCase 欄位、stdout／stderr／exit code，以及 full 不被 '-' 覆寫、partial secrets 仍拒絕、secret 不洩漏。範圍 scripts/release/desktop-bundle-config.test.mjs、scripts/release/signing-gate.test.mjs、scripts/desktop/desktop-release.test.mjs。驗證：node --test scripts/*.test.mjs scripts/*/*.test.mjs，先確認失敗來自缺少共用設定或工作流程接線。 <!-- speclink-task:tsk_01M6RJJBGG6BFGAMF58K8GP7N5 -->
- [ ] 2.2 實作 D2 共用設定產生器並讓兩條 workflow 使用同一份政策，保留 Desktop Build 的來源資訊輸出與 createUpdaterArtifacts=false、Release updater 預設及 full 的 Apple 環境路徑；未知模式失敗且不進 Tauri。範圍 scripts/release/desktop-bundle-config.mjs、.github/workflows/release.yml、.github/workflows/desktop-build.yml。驗證：前項案例轉綠，再跑 node --test scripts/*.test.mjs scripts/*/*.test.mjs，確認生成 JSON 只包含設計表的覆寫，不增加產品設定或套件相依。 <!-- speclink-task:tsk_01M6RJJBGGW6WPRFTAZP4W97W7 -->

## 3. 最終產物驗證的 Red／Green

- [ ] 3.1 先為 D3「macOS 最終產物驗證」的 App 與模式判定建立失敗案例：完整 ad-hoc、只有 linker 資訊但驗證失敗、任一程式碼簽章失敗、缺 CLI、主程式／CLI 少一種架構、full 公證或 Gatekeeper 拒絕、缺參數／檔案、未知模式及非 macOS。範圍 scripts/release/macos-artifact-verify.test.mjs。驗證：以 native 命令 fixture 保留真實成功／失敗語義，執行 node --test scripts/*.test.mjs scripts/*/*.test.mjs 並確認新案例先紅；不得將 fixture 當作實機啟動證據。 <!-- speclink-task:tsk_01M6RJJBGGARKWHTGR017FWE2M -->
- [ ] 3.2 實作 D3 的唯讀 App 驗證與錯誤契約，native 呼叫使用 argv、每次最長 120 秒，檢查整包與主程式／sidecar 的所有架構，none 不誤要求 Apple 公證、full 不降級；診斷走 stderr，失敗非 0，不補簽、不輸出 secrets。範圍 scripts/release/macos-artifact-verify.mjs。驗證：前項 App 與模式案例轉綠，node --test scripts/*.test.mjs scripts/*/*.test.mjs 通過，確認不新增測試專用的產品旗標。 <!-- speclink-task:tsk_01M6RJJBGGT3WJZGCVAEN69S4G -->
- [ ] 3.3 先為 D3 最終容器與清理建立失敗案例：參考 App 有效但 DMG／updater 簽章或架構無效、版本／主程式／CLI／Info.plist／CodeResources 不一致、解包失敗、native 逾時、掛載後驗證失敗、卸載失敗，以及兩條流程上傳前必須經過驗證。範圍 scripts/release/macos-artifact-verify.test.mjs、scripts/release/signing-gate.test.mjs、scripts/desktop/desktop-release.test.mjs。驗證：node --test scripts/*.test.mjs scripts/*/*.test.mjs，確認未檢查最後容器或漏清理會造成新案例失敗。 <!-- speclink-task:tsk_01M6RJJBGGY56REG0NEE8ZE34A -->
- [ ] 3.4 完成 D3 的獨立暫存目錄、只讀 DMG 掛載與 updater 解包、相同 App 內容核對、finally 卸載與安全清理，並在兩條 workflow 的 macOS Tauri 打包後／任何上傳前接上共用入口；Release 必帶 updater，Desktop Build 不加 updater。範圍 scripts/release/macos-artifact-verify.mjs、.github/workflows/release.yml、.github/workflows/desktop-build.yml。驗證：前項容器與接線案例轉綠，node --test scripts/*.test.mjs scripts/*/*.test.mjs 通過；確認卸載失敗時明確失敗且不刪仍掛載目錄，驗證後不改待發布內容。 <!-- speclink-task:tsk_01M6RJJBGGJYM485984GC5E10C -->

## 4. 文件與原生產物驗證

- [ ] 4.1 依 D5 修正「開發者入口文件雙語對」，讓 macOS 安裝表與步驟區分 ad-hoc、Developer ID 與 Apple 公證，無憑證流程說明首次個別允許，不宣稱所有正式 DMG 都免放行；保留五個 npm 入口、README 對應連結、實際檔名與 Windows／Linux 步驟。範圍 docs/development.md、docs/development.zh-TW.md。驗證：人工對照兩語章節及事實、重新搜尋簽章承諾無未解釋落差，並執行 node --test scripts/*.test.mjs scripts/*/*.test.mjs。 <!-- speclink-task:tsk_01M6RJJBGGNCV0BRE6YJ4CCFST -->
- [ ] 4.2 在本次 Mac 對修正分支新產物執行 D3 native 閘門，取得 App 與 DMG 的真實驗證結果；測試 updater archive 若由同一 App 建立須標為測試包，真實 Developer ID 或 Intel 原生環境未實跑須列明。依 speclink instructions --skill audit 的清單核對路徑／模式輸入、native 失敗／期限、只讀掛載與清理及 full 不降級；不為此改舊 tag 或系統政策。範圍 .github/workflows/release.yml、.github/workflows/desktop-build.yml、scripts/release/macos-artifact-verify.mjs、docs/development/macos-dmg-launch-investigation.md。驗證：原生 codesign／lipo 與容器驗證成功或明確失敗、記錄產物來源及限制；確認一般 CI 範圍判定與原發佈依賴不變。 <!-- speclink-task:tsk_01M6RJJBGGTXVZV8HK8SQV95XZ -->

## 5. 使用者啟動接受與整合檢查

- [ ] [M] 5.1 使用者依 D4「打包版啟動驗收」在 macOS 26.6.2／ARM64 關閉開發模式，從修正分支的新 DMG 安裝 App，依系統完成首次對該 App 明確允許，確認版本正確且可操作的看板或專案選擇器主視窗出現、不持續在 Dock 跳動。範圍 docs/development/macos-dmg-launch-investigation.md、docs/development.md、docs/development.zh-TW.md。驗證：由使用者回報產物版本／來源、允許條件及主視窗結果；代理不得代勾，未通過時保留安裝版程序／系統證據並回 ingest，不將靜態驗證當作成功啟動。 <!-- speclink-task:tsk_01M6RJJBGGPG40VZ6JFKRPJEPZ -->
- [ ] 5.2 完成 D2／D3／D5 整合檢查，確認程式碼工作僅落在已列路徑、兩條打包流程共用政策與驗證、完整 Apple／partial／updater／Windows／Linux 契約保留，報告 D4 手動接受及未實跑模式的狀態；此自動化檢查可在 5.1 尚未接受時完成，但不得代替 5.1。範圍 .github/workflows/release.yml、.github/workflows/desktop-build.yml、scripts/release/desktop-bundle-config.mjs、scripts/release/desktop-bundle-config.test.mjs、scripts/release/macos-artifact-verify.mjs、scripts/release/macos-artifact-verify.test.mjs、scripts/release/signing-gate.test.mjs、scripts/desktop/desktop-release.test.mjs、docs/development.md、docs/development.zh-TW.md、docs/development/macos-dmg-launch-investigation.md。驗證：最後執行 node --test scripts/*.test.mjs scripts/*/*.test.mjs，以及 speclink analyze macos-dmg-launch-failure --json、speclink validate macos-dmg-launch-failure --strict；產品 golden／CLI 基線不改，完整應用程式測試由既有 CI 守門。 <!-- speclink-task:tsk_01M6RJJBGGZ43JTFJQXAKJD1AW -->
