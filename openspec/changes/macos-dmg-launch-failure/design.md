## Context

此變更落在 repo 的 Node 打包工具與 GitHub Actions，不屬於 speclink-core 領域演算法、speclink-fs 儲存、speclink-host 裁決或 Desktop 的互動功能。沿用 Tauri bundler、signing-gate.mjs 的 full／none 決策、Desktop Build 已有 ad-hoc 作法及 Node 內建測試工具，不新增套件、crate、IPC、儲存抽象或 UI 元件。

已觀測的 0.12.1 App 在 ARM64 Mac 單獨啟動時停於 _dyld_start；原 DMG 與安裝副本的主程式／Info.plist 一致，完整簽章驗證失敗，Release 日誌顯示 none。這證明產物驗收有缺口，不足以直接宣稱簽章是 Dock 問題的唯一根因。使用者選擇先修無 Apple 憑證路徑；有憑證模式、updater 與跨平台交付維持原契約。

## Goals / Non-Goals

**Goals:**

- 先取得隔離副本的有效因果對照，再修改交付流程。
- 無 Apple 憑證時完成真正的 App ad-hoc 簽章，並驗證最後交付容器內的內容。
- 以實際打包版主視窗出現驗收啟動；文件明確區分 ad-hoc 與 Developer ID／公證。

**Non-Goals:**

- 不要求本次提供 Apple 憑證，不全域停用 Gatekeeper，不以移除原安裝 quarantine 作為修正。
- 不修改 Rust／Tauri／前端、產品 CLI 或 Local／Remote 契約；因果對照指向其他問題時先 ingest，不直接擴大實作。
- 不更改版號、既有 tag、更新端點／公鑰、updater minisign、發佈通路、資產檔名與 CI 範圍判定。

## Decisions

### D0 實作分支與起始點提交

實作順序固定為先建立本變更的新分支、先做一次起始點 commit，再進行 D1 實驗及程式碼工作。本次分支為 fix/macos-dmg-launch-failure，建立時基底為 e6dfcf30；起始點提交使用主旨「chore(macos): 記錄 DMG 啟動修正開發起始點」，保存來源討論、proposal、design、兩份 delta specs、tasks 與變更 metadata，既有封存 lock 檔不納入。

開始 apply 時先從 Git 記錄取得此提交的實際 SHA，確認它是目前實作分支 HEAD 的祖先、起始規劃文件已受版控，而且目前不在 main。若之後使用獨立 worktree，也必須包含此起始點提交。已存在且符合條件的分支／起始提交直接驗證沿用，不重設分支、不重做或覆寫起始提交。新增的前置驗證由任務 0.1 記錄，確認完成後才進行 D1；這不改產品規格或對外契約。

### D1 隔離副本因果驗證

在實作分支先把原 DMG 只讀掛載，將 App 複製至本次獨立暫存目錄。記錄來源雜湊與環境、原樣及完整重簽副本的驗證結果、相同啟動方式與觀察期限、quarantine／使用者允許條件。不得把不同啟動方式或不同安全條件下的結果直接比較成簽章因果。

只操作暫存副本；保持原下載檔、原安裝、其他程序及系統安全政策。重簽先處理巢狀程式，再封裝 App；實際命令與 native 工具輸出寫入簡潔的 docs/development/macos-dmg-launch-investigation.md，長篇系統日誌留在暫存位置，文件只保留必要摘要。

成功對照須能說明原樣重現、重簽副本進入 App 並顯示主視窗。若對照無效、無法重現原樣、需要尚未授權的安全設定變更，或重簽後仍卡住，記錄結果並停止後續程式碼修正，以 speclink-ingest 更新原因與範圍；不能略過此閘門或把測不到當通過。此處不以修改業務初始化作為預設替代方案。

### D2 共用桌面打包設定

新增 scripts/release/desktop-bundle-config.mjs，兩個 workflow 共用同一個小型設定產生器。私有介面為 node scripts/release/desktop-bundle-config.mjs release 或 desktop-build；不讀 stdin，成功 exit 0、stdout 僅一份 JSON 加換行，錯誤 exit 非 0、說明走 stderr 且無可用 JSON。channel 之外不增加可配置選項，平台使用實際 process.platform；Release 的 macOS 模式來自既有 SPECLINK_MACOS_SIGNING，未知模式失敗。

| 呼叫條件 | 產出覆寫 JSON |
| --- | --- |
| release、darwin、none | {"bundle":{"macOS":{"signingIdentity":"-"}}} |
| release、darwin、full | {}，保留既有 Apple 環境簽章與公證 |
| release、Windows／Linux | {} |
| desktop-build、darwin | {"bundle":{"createUpdaterArtifacts":false,"macOS":{"signingIdentity":"-"}}} |
| desktop-build、Windows／Linux | {"bundle":{"createUpdaterArtifacts":false}} |

Release 仍先執行 signing-gate.mjs，任何部分 secrets 都在建置前失敗；none 僅表示沒有 Apple 發行者憑證，不再代表 App 可以沒有完整 ad-hoc 簽章。產生器不讀取／輸出 secret 值，不改既有 full／none 的環境輸出契約。Desktop Build 不導入完整 Release 的 secrets 依賴。

各 workflow 將 stdout 寫到自己的 runner 暫存設定檔，再以絕對路徑傳給 Tauri 的既有 --config；設定產生失敗則不進行建置。原 Desktop Build 的版號／來源資訊輸出保留，移走的設定組裝由共用產生器取代。避免在兩份 YAML 各維護一份簽章政策，也不新增一般化建置框架。

### D3 macOS 最終產物驗證

新增 scripts/release/macos-artifact-verify.mjs，由兩條 workflow 在 macOS 的 Tauri 打包完成後、任何安裝檔／更新包上傳前呼叫。私有參數固定為 --app 路徑、--dmg 路徑、--mode none|full；Release 另必須提供 --updater 路徑，Desktop Build 不提供 updater。所有參數以 argv 傳入 native 工具，不組 shell 字串；缺檔、未知參數／模式、非 macOS 執行皆失敗。

驗證入口只讀取產物，不在驗證階段補簽、重新打包或修改封裝。以 codesign 的 deep、strict、all-architectures 驗證完整 App，另核對 Contents/MacOS/speclink-desktop 與 Contents/MacOS/speclink 的各架構簽章，並以 lipo 確認兩個執行檔都有 arm64 與 x86_64；不能把 codesign 的顯示資訊或程序存在當驗收成功。

將 DMG 只讀、無 Finder 視窗地掛載到本次獨立暫存位置，對容器內 Speclink.app 做相同驗證；Release 的 updater archive 解開至另一個獨立暫存目錄後亦做相同驗證。三個 App 的版本及主程式、sidecar、Info.plist、CodeResources 雜湊須與 --app 參考產物一致，確保驗證的是最後發送內容。此工具只接受本 repo 產出的已知 App 封裝，不設計一般化 archive API。

full 模式另要求 Gatekeeper 評估與公證票證檢查通過；none 模式不以 Gatekeeper 接受作為自動化成功條件，因 ad-hoc 不等於 Apple 公證。原 updater 簽章流程照常運作，驗證成功後不再修改任何待發布內容。

native 子程序各有最長 120 秒期限，錯誤、逾時或驗證不符皆 exit 非 0，stderr 點名產物／階段／原因，且不印環境或 secret。成功 exit 0，stdout 印所驗證的模式與產物；沒有產品 --json 或 no-color 介面變動。

暫存寫入順序為建立唯一目錄、只讀掛載 DMG、解開 updater、逐項驗證、卸載本次掛載點、移除已卸載的本次暫存內容。任何錯誤都在 finally 嘗試卸載；卸載失敗也算驗證失敗，留下明確掛載點供 runner 清理，不強制卸載別的磁碟、不遞迴刪除仍掛載的目錄。驗證失敗不能上傳該平台產物，既有 Release 的 desktop 依賴會阻止缺形態的 GitHub Release 建立；不另外改 npm／Docker 的依賴圖。

### D4 打包版啟動驗收

CI 的 native 產物驗證與人手啟動驗收分開記錄。修正後以本變更分支的本機或 Desktop Build CI 新 DMG 驗收，不為測試移動已發布 tag。至少在目前 macOS 26.6.2／ARM64 環境，關閉開發版後安裝測試產物，由使用者在 macOS 對該 App 首次明確允許，確認看板或專案選擇器主視窗出現、可以操作；檢查產物版號，避免把舊 App 或開發程序算成成功。

這項使用者操作與接受結果以 [M] 任務記錄，代理不得代勾。ARM64／Intel 的架構與簽章在自動化閘門檢查；有 Intel 原生環境時補實際啟動，無環境時明列「Intel 原生啟動未驗證」，不把交叉編譯、lipo 或簽章檢查冒充實機啟動。

### D5 文件與回歸邊界

中英 development 文件維持章節及事實對等，將「macOS DMG 一律已簽章並公證」改為依交付模式說明：ad-hoc 產物需依系統設定對特定 App 明確允許；Developer ID 且公證成功的產物依其驗證結果標示。保留既有五個 npm 入口、README 對應語言連結、安裝檔名及 Windows／Linux 步驟。

同類位置排查涵蓋 Release、Desktop Build、sidecar、signing-gate、desktop-install 及中英下載文件。desktop-install 仍保留既有本機安裝的簽章環境與版號斷言，不在本次將它改成通用無憑證安裝工具；sidecar 的建置／lipo 規則不變，完成的 sidecar 納入最終驗證。

## Implementation Contract

- 範圍內：D1 的診斷紀錄；D2 的兩條 workflow 及共用設定產生器；D3 的共用 native 產物驗證與其測試；D4 的真實安裝啟動驗收；D5 的中英文件與既有打包接線測試。具體路徑以 proposal Impact 為清單。
- 範圍外：應用程式與核心 crate、UI、IPC、Local／Remote 行為、新套件相依、產品設定 schema、生成技能、發版號／tag 與更新來源金鑰。沒有 serde 或資料儲存格式遷移。
- 設定產生器以純政策轉換測試所有 channel／平台／模式組合，並以 CLI 子程序確認 JSON、exit code 與 stderr；共用兩條實際 workflow 呼叫，不加測試專用產品旗標。
- 產物驗證測試以暫存命令 fixture 模擬 native 成敗、缺檔、缺架構、簽章錯誤、包內內容不一致、full 公證拒絕、逾時與卸載失敗，斷言不補簽、不漏清理及非 0 錯誤；另以真正 Mac 原生工具檢查新產物。命令 fixture 通過不得宣稱原生啟動通過。
- TDD 順序：D0 的分支與起始點提交先完成，再完成 D1；D2／D3 先有對應失敗案例，再實作與接線；D4 的人工接受不阻止品質站檢查已完成程式，但封存仍等待手動任務。
- 所有 scripts 驗證使用 node --test scripts/*.test.mjs scripts/*/*.test.mjs。此變更不改產品輸出，既有 golden／CLI 基線維持，不因只改打包而重跑整個 Rust／前端套件；完整應用程式回歸由既有 CI 決定。

## Risks / Trade-offs

- [golden／CLI 輸出意外漂移] → 不修改產品原始碼或生成來源，確認 Git 差異範圍；新增私有腳本輸出不冒充產品 API。
- [Windows／Linux 打包行為變動] → D2 明確保留 updater 覆寫值，native 閘門僅 macOS 執行，維持原 CI 範圍判定及各平台產物集合。
- [ad-hoc 驗證通過但 macOS 仍拒絕首次開啟] → D4 保留實際下載／手動允許／主視窗的人工驗收，不宣稱 Apple 公證，D1 無有效對照就停止擴大修正。
- [有 Apple 憑證的路徑被 '-' 覆寫] → full 的設定必須為空覆寫，fixture 覆蓋 full／none／partial，原生 full 閘門再要求公證結果。
- [掛載或子程序卡住] → D3 使用命令期限、唯一暫存目錄與失敗清理；退出失敗阻止產物上傳。
- [缺 Intel 原生機或真實 Apple 憑證] → 如實記錄未執行範圍，使用架構／native 簽章檢查與政策 fixture 保護既有契約，不把模擬結果當成實機驗證。

## Migration Plan

完成 D1 對照後，在實作分支落地腳本、兩條 workflow 接線及文件；先驗證新分支打包產物再進行後續發版。既有 v0.12.1 不移 tag、不覆寫；修正發布使用後續由使用者決定的新版本。若 native 閘門失敗，該平台沒有上傳產物且 Release 不建缺形態資產；回滾以還原這次 workflow／工具變更的新提交處理，不變動使用者安裝或 updater 金鑰。

## Open Questions

簽章缺口是否完整解釋載入卡住由 D1 實驗裁定，這是明確的實作前置閘門。Intel 原生啟動及真實 Developer ID 產物若缺環境，在驗證報告中列為未實跑，不能宣稱完成該部分；本次不要求購置憑證或變更交付政策。
