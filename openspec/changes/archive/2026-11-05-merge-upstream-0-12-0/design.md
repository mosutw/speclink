## Context

目前 Fork main 為 86dd8c9af661827cb0e2615c098500048dfd5615、產品版號 0.11.2，已用逐段整合方式納入上游 0.11.0（64623a6bfbe866f4ae33e37b918cb39b941f38cb）。本次目標固定為 v0.12.0 的 26de3b9fb3342da43b05a2dbb790ff2f942bcde8，新增內容涉及排程與分刀、桌面導航與路線圖、Herdr 整合，以及 Fork issue #15 的 Antigravity 目錄錯誤。

上游變更沒有涵蓋 Fork 的安全與品質歷史、Antigravity、Proxy 診斷、外觀及發佈客製；不能直接把目前檔案換成上游版本。Codex 已使用 .agents/skills，Antigravity 目前使用 .agent/skills；兩者相同檔名的內容分別帶不同技能引用與 --agent 身分。只修改目錄常數會在同步與取消選取時造成覆寫和誤刪。

## Goals / Non-Goals

**Goals:**

- 在新分支保留 Fork 行為，整合上游指定版本的新增功能與測試修正，逐檔留下來源及取捨盤點。
- 排程、討論分刀、本地及遠端由同一引擎產生；網址、路線圖與 Herdr 沿用既有資料和生命週期，不另存一套文件真相。
- Antigravity 改用 .agents 並與 Codex 共存，舊錯誤佈局可安全升級、重試，取消工具不誤刪。

**Non-Goals:**

- 不整批重套 0.11.0、不追隨目標標籤以後的 main、不搬入上游規格歷史取代 Fork、不強制搬走 openspec/。
- 不新增 CLI 動詞、工作流設定鍵、Store 抽象或 OpenSpec 全流程相容層。
- 不安裝或啟用外部 Herdr、修改使用者全域設定、調整產品版號、公開發佈或將 PR 指向上游。

## Decisions

### D1 新分支與逐檔整合盤點

開發第一步核對工作目錄，從目前 Fork 基底建立並切換 feat/merge-upstream-0-12-0；若同名分支已存在，核對祖先與內容，不強制重設。先保留本提案和來源討論等既有未提交文件。以 git diff 與 git log 比較 64623a6b 至 26de3b9f，將新增／改動路徑與本地客製交會處記入 docs/development/upstream-0.12.0-inventory.md，每項記錄採入、已存在、不採用及理由。只對已比對的段落套入，不把整批上游分支 merge 到 main；此流程用 Git 原生路徑與參考，Windows 路徑不靠 shell 字串切割。

產品版號、updater endpoint、pubkey、Windows portable、發佈與 CI 守門保留。若之後建立 PR，base 固定 mosutw/speclink 的 main。未採直接覆蓋或批次套整段歷史，因上次整合並非共同祖先更新，會重套已納入功能並失去 Fork 客製。

### D2 共用分刀與排程協定

沿用上游 crates/engine/speclink-core/src/lifecycle/discuss/cuts.rs 的純解析：從 Decision 的頂層短橫線列點辨識分刀，排除程式區塊、註解及非該格式。plan 與討論詳情從同一份記錄內容及 promoted_to 算出欄位，前端只依 line 對應回文件並呈現，不另猜分刀。核心不含 ANSI、不綁定磁碟；透過既有 Store、Host／Protocol 路徑供 CLI、Server、Node 與桌面消費。移除這個解析模組會使分刀與未轉出工作無法計算，具有實際領域責任；不加轉送型包裝。

plan 新增 unpromotedCuts 陣列；討論詳情新增 cuts；桌面清單新增 next 及 unpromotedCuts。沿用上游 camelCase 與 serde default，缺欄位讀空值。討論記錄以同次讀取算欄位，避免 content 與 promotedTo 不一致；CLI discuss show 仍剔除新增 cuts，不改原人眼與 JSON。變更清單保留 Fork 安全、品質歷史及所有現有欄位，轉換時逐欄搬運；遠端不走 raw JSON 旁路。拒絕平行實作多套解析，因會使路線圖、詳情與 CLI 判定分歧。

### D3 路線圖與網址導航

packages/ui/src/components/RoadmapView.tsx 與 packages/ui/src/roadmapModel.ts 承擔排程資料到路線圖的實際呈現與相依路線計算，使用同次看板載入的資料。沿用 @speclink/ui 的 Button、Badge、Tooltip、ScrollArea、空狀態及既有詳情頁；需要的視覺變體定義於 packages/ui，apps/desktop 不另建同名元件、不用 className 蓋掉共用變體顏色。返回、Esc、詳情前後移動與本頁目錄繼續走既有共用狀態及文件元件。保留看板掛載但使隱藏視圖不可見、不可互動，返回時恢復搜尋、篩選及捲動。

apps/desktop/src/deepLink.ts 為唯一網址解析、訂閱及派送入口，嚴格驗證 scheme、種類、單一 project 參數、百分比編碼、控制字元與本機絕對路徑。OS 接入採上游 Tauri deep-link 與 single-instance 相依及註冊；Rust 殼只註冊與委派，檔案邏輯仍在 apps/desktop/core，前端導航仍在 store。依到達順序排隊，等待分頁還原，再呼叫 openProjectAt；遇到遠端標記一律由既有選擇器接手，不由網址跳過認證或確認。不可取消浮層開啟時不切頁。缺名稱顯示現有 toast，無效網址不建立專案、不執行任務、不寫規格。Windows／Linux 重開喚起既有視窗；保留 Fork 系統匣、portable 與 updater 註冊。

### D4 Herdr 面板與 Claude Code 焦點

把 integrations/herdr/ 與 scripts/herdr/ 的目標版本來源及測試納入本 repo，保留上游 model.js、shared.js、main.js、skill-picker.js、i18n.js、column-sync.js 的職責；面板查詢既有 CLI JSON，沒有自建工作流 API。以現行 agent／worktree／焦點標籤尋找工作者，先確認目標再交付技能，不把一般文字提及當成工作歸屬；一個 archive+commit 一次只送一份，避免同時提交。worktree 收尾先建議合併，兩站有效章才建議 archive+commit；Fork 的安全驗收政策仍由既有命令守門，不讓面板推薦繞過。

Claude Code 的 withVerb 與 commandChange 沿用上游辨識邏輯：discuss、propose、improve、audit、trace、manual、baseline、config 先清掉舊變更焦點；propose 後續建立變更時可由 CLI 回報綁定，其他非變更技能忽略背景查詢。只有處於 Herdr 且確認當前變更時透過既有 mod API 回報 speclink.focus；缺 Herdr 時維持原本技能列及標題流程。面板及內建畫面的語系沿用上游系統弱中文判定，中文環境中文，其餘英文；不把工作流 tw／ja／en 偷換成裝置語言。不實際執行安裝器或寫使用者 Herdr／Claude Code 設定。

### D5 Antigravity 與 Codex 共用技能渲染

將 Tool::Antigravity.skills_dir 改為 .agents/skills、桌面 builtinTools 顯示 .agents。兩種工具在單選與同選時一律產生完全相同的共用內容，避免切換選集又改寫整組檔案。沿現有 skills.rs 增加共用渲染分支，使用非 Claude frontmatter；技能互相引用寫作 speclink-<skill> 的名稱，開頭簡短說明 Codex 以 $speclink-、Antigravity 以 /speclink- 呼叫，這些是宿主技能入口而非新增 CLI 動詞。實際 speclink CLI 範例維持原命令。

工具身分不固定成其中一種：共用內容的 --agent 範例使用 <current-agent>，開頭明訂執行者依當前宿主填 codex 或 antigravity，執行前須替換，不得照抄佔位字或以另一工具冒名。這是技能既有 <name> 類執行參數，不是漏換的生成模板；產物仍不得留下 {{TOOL}} 等模板。與本機 plan mode、互動或技能工具能力相關的宿主分歧寫在同一前言，Codex 正常詢問、Antigravity 使用其可用介面。共用前言由引擎渲染一次，不能只修改生成檔。

SDK 的 skills.render 維持原 target 聯集及 options，target codex 和 antigravity 改走同一共用分支，與兩種 CLI 單選／同選的生成位元一致；neutral、Claude、Copilot 不變。更新資產版本鎖、渲染 golden 及四工具生成；因本 repo 同時選取兩者，.agents 僅生成一份，舊 .agent 依升級流程處理。不採優先權覆蓋、不移動 Codex、不另造嵌套目錄或兩套技能名稱，因會使宿主發現方式和使用者既有入口變動。

### D6 共用同步目標與舊目錄升級

沿 SyncPlan::resolve 將 Codex／Antigravity 合為一個實體 .agents/skills 目標，保留兩種邏輯選取和對外工具摘要；生成、降級守門、預期內容比對與不同檔案清單共用同一份目標，差異路徑去重，逐工具方向仍各自回報。取消工具的清理按「該實體目錄是否還有被選取的工具」判斷：只取消其中一種不刪 .agents；兩者都取消才清受管技能。worktree 政策關閉只移除該目標的受管 worktree 技能，自有技能不動。

寫入順序固定：先驗證設定、自訂描述子、實體目錄邊界與降級守門；生成完整 .agents 受管內容成功後，才清理舊 .agent/skills 的受管目錄，最後保存既有同步足跡。所有寫入沿既有檔案寫入函式，YAML 純量若需變更用 util.rs 的 yaml_scalar；不增加設定鍵或旁路 writer。保留 .agent/skills 作為舊受管目錄的保留名稱，避免自訂描述子先佔用它又被升級清理刪掉，撞名仍在寫入前拒絕。

接受既有逐檔同步的可續作狀態，不新增跨目錄交易抽象：.agents 寫到一半失敗則非零退出、指出路徑，.agent 原內容仍在；新目錄全數完成、清舊目錄途中失敗則新目錄已可用，回報失敗路徑，重跑收斂並清理剩餘受管內容。對舊目錄或技能子目錄為連結、重解析點或父目錄會導向專案外者，不跟隨、不清理、不寫入外部目標，保留原內容並回報跳過；需要寫入的新目錄不安全時在任何技能寫入前拒絕。清理限受管 speclink-*，保留自有技能、規則與 .agent 其他檔案；不直接刪整個 .agent。無選取 shared 工具時清理其受管舊／新技能仍須遵循同樣邊界。

### D7 文件與驗證分層

保留單一整合變更，因使用者要求同次納入版本功能及 issue #15，且跨端 payload 與畫面須一起交付；任務按分支、引擎、協定、UI、Herdr、工具修正、文件順序拆小，避免一項任務跨整個產品。維持更新日誌歷史；正式規格只在封存由 delta 合併，不預先覆寫；手冊及中英文件使用本 repo 的 openspec/，保留既有歷史。

先跑受影響 crate、Vitest、腳本及 SDK 測試，golden 只有已明記的 plan 新欄位和共用技能輸出能變；Node binding 先建置再測試，保留 86dd8c9a 的型別測試逾時修補。npm run test:all 仍由既有 CI 守門，不為此改發佈或擴充本機反覆完整測試。無外部 Herdr 的環境以模型、腳本測試驗證程式整合，實際安裝與啟用是已延後的工作，不偽稱已實測該外部應用程式。

## Implementation Contract

| 範圍 | 成功結果 | 失敗或退回 | 驗證依據 |
| --- | --- | --- | --- |
| 分支與保留矩陣 | 修改程式前已在新分支，來源固定兩個版本；安全、品質、路徑、更新來源仍在 | 同名分支先核對、不重設；使用者修改保留 | Git 狀態、逐檔盤點與既有 Fork 回歸測試 |
| plan | JSON 多 unpromotedCuts；人眼有資料才多 Unpromoted cuts，無 ANSI 模式無控制碼；無 stdin、成功 0 | 成環非零且 stdout 空；remote strict-overlap 仍拒絕；讀不到單份討論略過 | core 分刀、CLI local／remote 與 no-color 測試 |
| 詳情與清單 | cuts 各項 cut/title/note/line 字串、changes/unpromoted 陣列、changesInBold 布林；清單保留既有欄位 | 缺 cuts 或 unpromotedCuts 用空陣列、缺 next 用 null；discuss show 原鍵集合不變 | Protocol、remote、Server 及桌面 adapter 測試 |
| 路線圖 | 同一次載入、正確波次與依賴、詳情返回路線圖、複製指令不執行 | 壞 metadata 顯示未排程數；遠端斷線／認證失效沿用現有錯誤且不改資料 | packages/ui 路線圖模型、元件及 App／store 測試 |
| 網址 | 四種類導航，佇列與啟動還原有順序，喚起既有視窗 | 無效編碼、重複參數、相對路徑、找不到目標提示；遠端須經選擇器 | URL／store／Tauri 殼建置；三平台自動測試涵蓋路徑 |
| Herdr | 面板與側欄反映 CLI，技能送給正確 Agent、worktree 合併優先、保留所有既有安全守門 | 找不到 CLI／壞設定顯示錯誤；未初始化才顯示 init；等待回答不誤送 | scripts/herdr/panel-model.test.mjs 與 Claude Code hook 測試 |
| issue #15 | 介面顯示 .agents；單選／同選生成相同技能；更新後探測現版；取消其中一個不刪共用內容 | 設定壞、較新版或不安全路徑在寫入前拒絕；失敗可重跑且不刪自有檔案 | core／CLI／desktop-core／前端／SDK 與 golden |

遠端的所有新增資料均為唯讀查詢，不新增 revision 寫入；認證、離線與衝突沿既有 Protocol 傳遞。桌面清單在可讀 board 但 plan 端點不可用時，沿既有相容契約退回 board overlay，next 為 null、unpromotedCuts 為空；plan 的 409 成環另保留 planError。主資料本身讀取失敗仍顯示既有錯誤，不把失敗當成功空清單。路線圖或面板啟動技能後的寫入仍由既有 Host 裁決認證、revision、安全驗收與封存政策。

## Risks / Trade-offs

- 回歸對照：plan 新鍵與共用技能為刻意變更，其他輸出仍保護；用舊／新 Server fixture、CLI parity、golden、Fork 安全與品質紀錄測試辨別意外退步。
- 跨平台：URL 的 Windows 磁碟路徑、CRLF、macOS 大小寫、Windows reparse point 與 single-instance 行為不同；以跨平台 fixture 和現有 CI 架構驗證，不以本機通過代替三平台結果。
- 共用技能的身分由執行宿主選定，文件需明確且 golden 鎖定，避免固定成 Codex 或 Antigravity；不為此擴大 CLI actor 自動偵測。
- 多檔同步途中失敗接受可續作狀態，先寫新位置再清舊位置以保留可恢復來源，不宣稱跨目錄原子交易。
- Herdr 隨外部版本變動；鎖定已核對的 0.1.13 以上需求及上游行為，安裝後實際互動未執行時須如實列出。

## Migration Plan

1. 開工先建立新分支，記錄目前 HEAD、未提交文件與 Fork 保留矩陣，再按任務整合指定差異。
2. 新引擎發布前以 fixture 驗證舊 .agent 到 .agents、Codex 共存與重試；既有專案以原有 update 或桌面工具存檔同步升級，tools 與 spec_dir 不改格式。
3. 新 client 可先接舊 Server，以空欄位退回；Server 升級後開始提供欄位；舊 client 繼續忽略新增欄位。不變更儲存結構或規格目錄。
4. 回退以整合分支提交和已保存工作目錄內容為單位，不強制重設使用者修改；若需恢復舊版工具生成，先恢復相應設定／技能備份，不用新清理流程刪掉使用者內容。產品發佈及外部 Herdr 安裝另行決定。

## Open Questions

無影響本次實作的待決項目。外部 Herdr 安裝啟用、最終版號及公開發佈、OpenSpec 生態完整互通仍在範圍外。
