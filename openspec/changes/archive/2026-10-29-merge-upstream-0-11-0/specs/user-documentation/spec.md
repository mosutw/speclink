## ADDED Requirements

### Requirement: workflow 文件載明蓋章前的預防規矩

中英文 workflow 文件的「品質關卡的共同規則」SHALL 載明兩條蓋章前的預防規矩，並各自寫出原因：先把最新的主分支合進來（rebase 或合併）再跑品質關卡——主分支前進時改到變更範圍內的檔，即使合併沒有衝突也會讓章失效；專案有 CI 時先等 CI 在所有平台通過再蓋章——章凍結範圍內檔案的內容指紋，蓋章後為了 CI 再修任何一行都會讓章失效。同一節 SHALL 說明章失效後重新蓋章要從首輪重跑兩道關卡（工單在蓋章時已刪除），SHALL NOT 寫成只要「再跑一輪」。

#### Scenario: 讀者在共同規則看到兩條預防規矩

- **WHEN** 讀者打開 docs/workflow.zh-TW.md 或 docs/workflow.md 的「品質關卡的共同規則」
- **THEN** 可見「先合進主分支再跑關卡」與「先等 CI 綠再蓋章」兩條規矩，各自附原因，且中英兩版一一對應

#### Scenario: 重蓋成本寫實

- **WHEN** 讀者在同一節查「蓋章後檔案又被改」怎麼辦
- **THEN** 說明寫出重新蓋章要從首輪重跑兩道關卡、原因是工單在蓋章時已刪除，沒有「再跑一輪」的說法

### Requirement: 使用者文件載明本地產物的純文字可攜性

README 與入門教學的中英文版本 SHALL 說明 Local 產物為 Markdown／YAML，可直接閱讀修改並由 Git 追蹤，預設位於 openspec/，Speclink 擴充含 discussions/ 與 .meta.yaml；遠端正式資料在 Store，本機為唯讀投影。文件 SHALL 明說本 Fork 預設明寫 spec_dir: openspec 而不自動搬走資料夾，但 .meta.yaml 等格式仍跟隨上游，SHALL NOT 據此宣稱 OpenSpec CLI 或所有相關工具完整互通，也不得照抄預設會把 openspec/ 搬成 speclink/ 的上游敘述。

#### Scenario: 中英文描述與實際設定一致
- **WHEN** 閱讀 README 與 getting-started 的中英文版本
- **THEN** 預設路徑均為 openspec/、新資料檔為 .meta.yaml，說明原始檔可攜性與尚未驗證的跨工具互通範圍

### Requirement: 入門教學與設定說明載明規格資料夾的遷移

中英文入門教學及設定說明 SHALL 寫明 .speclink.yaml 預設含 spec_dir: openspec、既有 openspec/ 專案會先補值再更新技能、明確自訂路徑不被覆寫，已使用 speclink/ 的專案仍可使用原路徑。文件 SHALL 區分「技能更新」「個別變更 metadata 改寫為 .meta.yaml」與「資料夾搬移」，不得把前兩者描述成必須搬走 openspec/。

#### Scenario: 升級說明不要求搬資料夾
- **WHEN** 既有 Fork 使用者依中英文升級說明執行 update
- **THEN** 說明可對照補入 spec_dir: openspec 的結果，沒有要求將既有 openspec/ 改名，且另說明新舊 metadata 的讀寫規則

### Requirement: 文件說明規格資料夾模式的單向轉換

README、getting-started 與 configuration 的中英文版本，以及操作手冊的規格資料夾與專案設定頁 SHALL 說明 OpenSpec（openspec/）為預設、Speclink（speclink/）為可選的單向轉換目標，入口是專案設定的 .speclink.yaml 頁。文件 SHALL 說明選取 Speclink 先提示無法轉回，明確確認才轉換、取消零寫入、成功後 OpenSpec 選項停用；同時交代一般 update 仍保留 OpenSpec 預設、自訂路徑與 remote 不使用此轉換入口。兩種模式 SHALL 僅指資料夾，不宣稱 OpenSpec CLI 全流程相容，不將選項操作描述為另造資料格式或雙份同步。

文件 SHALL 區分資料夾／設定已切換但技能尚未完成的狀態與轉換成功，說明未完成時同向重試；既有新舊 metadata 讀寫、品質歷史與安全政策保留的敘述 SHALL 維持。中英文 SHALL 有相同事實與章節對照，操作頁 SHALL 不要求使用者先手動刪除 spec_dir 來觸發搬移。

#### Scenario: 讀者找到模式選擇及不可逆提醒
- **WHEN** 閱讀 configuration 的中英文版本與操作手冊規格資料夾頁
- **THEN** 可找到專案設定入口、兩種模式與預設值，確認前警告無法轉回、取消不改檔、確認後單向轉換及未完成可續作的說明，與實際畫面一致

## MODIFIED Requirements

### Requirement: 使用者文件採漸進揭露與單一責任
<!-- BEFORE: 首頁定位及路徑敘述沿用既有版本。 -->

Speclink 使用者文件 SHALL 以 README、getting-started、workflow、product-status 與 roadmap 形成漸進揭露入口。README 第一個 H2 之前 SHALL 呈現品牌圖片、標語（繁中版「讓 AI 寫的每個變更，都對得上規格、查得到證據」，英文版「Every AI-built change, checked against its spec」）、徽章、語言切換、定位段、雙路徑段，以及一張 desktop 看板截圖。定位段 SHALL 說明 Speclink 是以 Rust 實作的 SDD 引擎，並以可驗收為主軸：每個變更先寫一小份規格、交給 AI agent 實作，封存前由審查與驗證兩道品質關卡檢查並留下證據，規格是 repo 裡的純文字；定位段 SHALL NOT 出現 PM 與 PO。雙路徑段 SHALL 說明 Local Repo／Remote Store 兩條路徑：一個人用 Local Repo 即足夠，團隊要共用規格再接 Remote Store；PM／PO／RD／AI Agent 共用 change／artifact／task／verify／archive 語意，以及 PM、PO 在團隊這一層參與討論與提案的說明，SHALL 放在雙路徑段。README SHALL 以 H2 提供：為什麼用 Speclink、快速開始、畫面一覽、特色、最短流程心智模型（工作流程）、安裝、自架 server、文件地圖、由 product-status 校正的目前狀態摘要、參與貢獻、致謝與授權；快速開始 SHALL 位於安裝之前，並以安裝 CLI、`speclink init` 與請 Agent 執行 propose 三步呈現 Local Repo 開始入口。設計之初以 Spectra App 2.3.1 CLI 為行為參考的歷史起源 SHALL 寫在致謝段，同段 SHALL 致謝 OpenSpec 的目錄結構；該起源 SHALL NOT 寫在第一個 H2 之前。getting-started SHALL 只承載可直接完成的 Local Repo 第一輪，workflow SHALL 作為完整使用流程正典，product-status SHALL 作為目前能力狀態正典，roadmap SHALL 作為尚未完成的方向的唯一落點。各文件 SHALL 以連結導向下一層細節，SHALL NOT 在 README 或 getting-started 複製完整架構與狀態矩陣。

#### Scenario: README 保留專案定位與起源

- **WHEN** 使用者開啟繁體中文或英文 README 判斷 Speclink 是什麼及為何存在
- **THEN** 第一個 H2 之前可見品牌圖片、可驗收標語、語言切換、以 Rust 實作的 SDD 引擎與可驗收定位、Local Repo／Remote Store 說明（含 PM、PO 參與的團隊情境）及 desktop 看板截圖；致謝段可見 Spectra App 2.3.1 行為參考起源與 OpenSpec；目前狀態段清楚區分已可運作與規劃中內容，並連到專案路線圖
- **AND** 文件整理只校正過時事實、術語與連結，不得將第一個 H2 之前的內容改成僅含導覽連結的入口

#### Scenario: 定位段以可驗收開場、團隊角色放在雙路徑段

- **WHEN** 讀者只讀繁體中文或英文 README 的標語與第一段
- **THEN** 標語為「讓 AI 寫的每個變更，都對得上規格、查得到證據」（英文版「Every AI-built change, checked against its spec」），第一段依序可見「先寫一小份規格」「交給 AI agent 實作」「封存前由審查與驗證兩道品質關卡檢查並留下證據」
- **AND** 第一段不出現 PM 與 PO；PM、PO 與共用語意的說明出現在緊接其後的雙路徑段，且與 Remote Store 的團隊情境寫在一起

#### Scenario: 首次使用者由 README 到完成第一輪

- **WHEN** 首次使用者從繁體中文或英文 README 尋找安裝與第一輪 Local Repo 操作
- **THEN** README 的快速開始段以三步呈現安裝 CLI、`speclink init` 與請 Agent 執行 propose，並提供可見的 getting-started 入口；getting-started 以目前存在的 CLI／skill 完成 init、提案、實作檢查與封存，遇到選用分支時可連到 workflow

#### Scenario: 進階使用者查詢目前能力與目標

- **WHEN** 使用者要判斷 Desktop Remote Workspace 或 Server 某項能力目前是否可用及最終目標
- **THEN** README 導向 product-status 取得目前狀態、證據與限制，並由該列導向專案路線圖取得之後的方向，兩者 SHALL NOT 混成同一狀態描述

## REMOVED Requirements

### Requirement: 使用者文件載明本地產物的 OpenSpec 結構相容性

**Reason**: 上游 0.11.0 改為新 metadata 名稱，不能繼續承諾舊檔名不變或未驗證的工具完整相容。

**Migration**: 改依「使用者文件載明本地產物的純文字可攜性」；本 Fork 保留 openspec 預設，讀取舊 .openspec.yaml、新寫入 .meta.yaml，既有資料不整批搬走。
