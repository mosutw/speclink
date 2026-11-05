---
topic: 原作者已釋出0.120, 以我目前的版本為主，將原作者的功能合併到我的版本
slug: merge-upstream-0-12-0
status: promoted
created: 2026-11-05
created_by: mosutw <luke1974@gmail.com>
promoted_to: merge-upstream-0-12-0
---

# Discussion: 原作者已釋出0.120, 以我目前的版本為主，將原作者的功能合併到我的版本

<!--
Document rules:
- Rounds are appended by `speclink discuss add-round`; never rewrite an earlier round.
  A changed position gets a new round that names what changed and why.
- Each round distills one focus question: **Focus** / **Position** / **Ruled out** / **Open**.
- The conclusion must resolve or explicitly defer every open question left by the rounds.
-->

## Context

使用者要求以目前 Fork 為主，整合原作者新版本的功能；經 GitHub 正式 Release 核對，0.120 指 v0.12.0，發布於 2026-11-05。目標和主從原則明確，不需先追問目標。使用者已確認首輪提出的五項整合原則，並要求開發前先建立新分支。
Fork 起點為 main 的 86dd8c9af661827cb0e2615c098500048dfd5615，目前產品版號為 0.11.2；已完成上游 0.11.0 的功能整合。此次上游鎖定 v0.12.0 對應 commit 26de3b9fb3342da43b05a2dbb790ff2f942bcde8，本機參考為 upstream/release-v0.12.0；以前次整合的上游 0.11.0 commit 64623a6bfbe866f4ae33e37b918cb39b941f38cb 為新增差異基準，不能因 Git 共同祖先較早而重複套用已整合內容。上游實際差異涵蓋 plan 未轉出工作、共用討論分刀解析、路線圖、speclink 網址、Herdr 整合與測試穩定性修正。
相關正式規格為 change-plan、desktop-app、client-protocol、claude-code-workflow-panel、spec-dir-migration、desktop-release；目前無進行中的變更。既有規格涵蓋排程、桌面操作及 CLI 同語意、OpenSpec 路徑預設與 Fork 發佈契約，新增路線圖與 Herdr 能力須在後續提案補齊。前次整合文件 docs/development/upstream-0.11.0-inventory.md 與來源討論作為保留客製內容的依據。
Prior discussions: merge-upstream-0-11-0

## Rounds

<!-- `### Round N — <mode> (<date>)` entries are appended here by the CLI. -->

### Round 1 — assumptions (2026-11-05)

**Focus**: 如何以目前 Fork 為主整合原作者 v0.12.0，並安排開發分支。
**Position**: 使用者確認五項整合原則，並要求實作開始前先建立新分支；整合範圍與作業方式已談定。
- 基底維持 Fork 0.11.2 的 86dd8c9a，逐段納入上游 0.11.0 至 v0.12.0 的新增差異，不直接以整份上游檔案取代客製內容，不混入 v0.12.0 之後的 main。
- 沿用 merge-upstream-0-11-0 的裁定：保留 spec_dir: openspec、安全審查、品質歷史、Antigravity、語言設定、桌面客製、Windows 免安裝版、Fork 更新來源與簽章設定；依據為前次討論、盤點與目前設定及程式碼。
- 納入 plan 的 unpromotedCuts，讓 CLI、Server 與桌面共用 lifecycle/discuss/cuts.rs 的分刀解析，保留舊 Server 缺欄位時的相容處理。
- 納入路線圖與 speclink://project、change、discussion、roadmap 連結；沿用現有專案開啟、視窗、遠端選擇與系統匣流程。依據為上游 deepLink.ts、RoadmapView.tsx 與協定差異。
- 納入 Herdr 面板、技能選單、Agent 狀態、Claude Code 即時回報、相關文件與測試，並保留 Herdr 0.1.13 以上的需求；安裝及啟用外部 Herdr 留待另行操作。
- 後續開發先由保有目前 Fork 內容的基底建立 feat/merge-upstream-0-12-0 分支，確認已切換後才改應用程式碼；若該名稱已存在，先核對內容再決定復用或另取名稱，不重設既有分支。
- 驗收同時確認上游新增功能與 Fork 保留功能；發版號與公開發佈待整合完成後決定。
**Ruled out**: 直接覆蓋 Fork 檔案或將主分支換成上游——可能遺失客製；重新套用整批 0.11.0 內容——目前已整合；只更新桌面畫面——會使 CLI、Server 與桌面判定不一致；在 main 直接開發——使用者要求先建新分支；整合時自動安裝或啟用 Herdr——不屬此次程式整合範圍。
**Open**: none

### Round 2 — assumptions (2026-11-05)

**Focus**: 是否把 issue #15 的 Antigravity 目錄修正納入此次上游整合，以及必須保留的共用目錄行為。
**Position**: 使用者明確要求 issue #15 一併修正，此要求補入同一整合範圍，並取代保留 Antigravity 原有 .agent 路徑的舊行為。
- issue https://github.com/mosutw/speclink/issues/15 為 OPEN，回報 Fork 0.11.2 Windows 本機模式中，AI 工具設定顯示 antigravity [.agent] 且實際建立 .agent，預期均為 .agents。
- crates/engine/speclink-core/src/workspace/skills.rs 的 Tool::Antigravity.skills_dir() 為 .agent/skills；apps/desktop/src/builtinTools.ts 的顯示值為 .agent。修正須同時涵蓋 UI、CLI／桌面共用的生成與同步、過期探測、取消工具的清理，以及 SDK、測試、相關規格與文件。
- 目前 workspace-tools 正式規格仍承諾 Antigravity 使用 .agent/skills，且與 Codex 互不覆寫；此次以使用者新要求更改路徑承諾，保留互不覆寫與不得誤刪的行為要求。
- Codex 已使用 .agents/skills，Antigravity 與 Codex 的技能內容分別帶 /speclink-、$speclink- 及不同 --agent。SyncPlan::resolve 目前按工具各生成一份目標；單純把路徑常數改成相同值會讓生成目標碰撞，也會讓取消選取的清理誤刪仍被使用的技能，不能只改字串或讓最後寫入者決定內容。
- 後續提案設計須處理 codex、antigravity 各自單選與兩者同選，讓 .agents 下的有效技能與所選工具一致，更新可重複執行，不互相覆寫且不長期顯示過期；取消其中一種時不得刪除另一種仍需的技能。共用內容與工具身分的具體呈現留在 design 決定，不先另建抽象層。
- 既有專案更新或同步時須處理 .agent/skills 的舊受管技能，保留自有技能、.agent 下其他檔案及不屬此次修正的工具內容，沿用不跟隨目錄連結清理的界線；不直接刪除整個 .agent 資料夾。測試包含既有錯誤佈局升級、Codex 共存與取消選取。
- 開發前先建立並切換 feat/merge-upstream-0-12-0 的要求不變；本輪只補充討論範圍，尚未實作或關閉 issue。
**Ruled out**: 只改介面標籤——實際路徑仍錯；只改 skills_dir 常數——會與 Codex 生成與清理碰撞；為修正 Antigravity 而移走 Codex 既有技能——不符合保留 Fork 既有行為的原則；整個刪除 .agent——可能刪除使用者檔案。
**Open**: 共用 .agents/skills 時的技能內容與工具身分如何呈現，留待後續提案的 design 依現有渲染與同步機制決定；沒有新增待使用者裁定的範圍問題。

## Conclusion

**Decision**: 以目前 Fork 為底，完整納入原作者 v0.12.0 的新增功能與相關修正，並在開發前先建立新分支。
- Fork 基準為 main 的 86dd8c9af661827cb0e2615c098500048dfd5615，產品版號 0.11.2；上游鎖定 v0.12.0 的 26de3b9fb3342da43b05a2dbb790ff2f942bcde8，不混入後續 main。比較前次已整合的上游 0.11.0 commit 64623a6bfbe866f4ae33e37b918cb39b941f38cb 至目標的新增差異，逐段整合，避免重複套用舊功能。
- 實作前先從保有目前 Fork 內容的基底建立並切換至 feat/merge-upstream-0-12-0，確認所在分支後才修改應用程式碼，不在 main 直接開發。若名稱已存在，先核對分支內容，不強制重設。若日後建立 PR，固定以 mosutw/speclink 的 main 為目標。
- 保留 spec_dir: openspec 預設與既有自訂路徑決策，不自動將本 repo 規格樹搬成 speclink/；沿用上游 .meta.yaml 格式與現有明確單向轉換能力，不新增 OpenSpec 相容層或雙份目錄同步，不宣稱已保證 OpenSpec CLI 全流程互通。
- 保留 Fork 現有安全審查與安全標準資料流程、品質紀錄保存與查閱、Antigravity、系統／Server 語言設定、提案條列、桌面客製功能、Windows 免安裝版、Fork 更新來源、簽章與發佈設定。交會處以保留目前行為為主，補入上游能力；不以搬入上游檔案整份覆蓋客製內容。
- 納入 plan 列出討論結論中尚未轉出的工作與 plan --json 的 unpromotedCuts，CLI、Server、桌面共用引擎的討論分刀解析，保留上游對列點格式的判定與舊 Server 缺欄位時的相容行為。例如結論含 cut 2 的變更 b，但 promoted_to 尚未包含 b，plan 應列出 b；轉出後不再列為尚未轉出。
- 納入看板路線圖檢視：按波次呈現、顯示依賴、列出尚未轉出的工作及複製轉出指令；納入 speclink://change、speclink://discussion、speclink://roadmap、speclink://project 連結與 Windows／Linux 再次啟動時喚起既有視窗的能力。例如 speclink://project?project=<編碼後的本機絕對路徑> 應開啟該專案看板，不因網址額外寫入規格或執行任務。
- 桌面連結與路線圖沿用現有專案開啟、視窗、系統匣及遠端連線確認流程。分刀判定由既有 core lifecycle/discuss 下的純解析模組擁有，透過既有協定與本地／遠端 adapter 提供給消費端；Tauri 殼負責網址外掛註冊，前端處理解析與導航。新模組應承擔實際解析、排程呈現或導航行為，不另增只有轉送作用的包裝或儲存抽象。
- 納入 integrations/herdr/ 的面板、停靠與浮動模式、技能選單與工作啟動、Agent 狀態與追蹤、側欄階段與進度、系統語言支援，以及 Claude Code 對 Herdr 的即時回報，連同相關文件與測試。沿用 Herdr 0.1.13 以上的需求；程式整合不自動安裝 Herdr、不修改使用者的外部 Herdr 或 Claude Code 設定。
- 納入相關測試穩定性修正，核對 Fork 既有同類修正後再套用，避免回退近期 CI 修補。驗收涵蓋 plan、討論分刀、路線圖、網址導航、舊 Server 相容、Herdr 模型、CLI／Server／桌面資料流，以及保留的安全與品質歷史、路徑預設、工具與發佈來源；執行受影響測試與專案要求的完整檢查。
- 正式規格與手冊依本 repo 實際 openspec/ 路徑更新，不搬入上游整棵專案歷史來取代目前內容。保留 Fork 更新日誌歷史，整合完成後再決定新版本條目與版號，不直接採用上游發版號提交；技能、快照或衍生文件有變更時依整合後來源產生。
- 一併修正 Fork issue #15（https://github.com/mosutw/speclink/issues/15）：Antigravity 的 AI 工具設定顯示改為 antigravity [.agents]，生成與同步使用 .agents/skills，取代原先 .agent/skills 的路徑承諾；此修正是保留 Antigravity 功能原則的明確例外，並非保留錯誤路徑。同步更新 CLI／桌面共用生成、過期探測、取消清理、SDK、測試、受影響正式規格與文件。
- Antigravity 改用 .agents 後須和已使用 .agents/skills 的 Codex 正常共存。驗收至少包含 codex 單選、antigravity 單選、兩者同選、重複更新、取消其中一個及取消兩者；不得互相覆寫工具所需的有效技能、誤刪仍被選取工具的內容，或因內容反覆覆寫而持續顯示過期。維持 Codex 的既有技能目錄；共用技能內容及工具身分的具體做法由後續 design 裁定，不能只改路徑字串或採最後寫入者覆蓋。
- 修正既有錯誤目錄的升級流程：更新或同步後技能落於 .agents/skills，舊 .agent/skills 下的受管 Speclink 技能應妥善清理；保留使用者自有技能、.agent 下其他檔案及其他工具內容，不直接整個刪除 .agent，不跟隨目錄連結清理。以既有錯誤佈局的升級測試確認生成、清理與重複執行結果。
**Rationale**: 使用者另要求一併修正 issue #15，實際程式也確認目錄錯誤；因正確路徑與 Codex 相同，需將共存及清理行為一併納入，避免修正路徑卻破壞既有工具。其餘整合原則不變。目前 Fork 已整合 0.11.0，新增功能主要落在討論排程、桌面導航與 Herdr；從上次已整合版本比較新增差異，能保留客製行為並減少重複改動。先建新分支可使整合結果獨立檢查，再依使用者指定的 Fork 主分支流程落地。
**Rejected alternatives**:
- 用上游檔案整份覆蓋目前 Fork 或直接切換成上游主分支 — 會失去現有客製與發佈設定。
- 重新套入整批 0.11.0 或混入 v0.12.0 以後的提交 — 超出此次新增差異範圍。
- 只更新桌面畫面、各端各自解析討論分刀 — CLI、Server 與桌面可能產生不同結果。
- 在 main 直接實作 — 違反使用者要求先建立新分支。
- 整合時自動安裝或啟用外部 Herdr — 使用者確認的是功能納入，安裝與啟用另行處理。
- 新增完整 OpenSpec 相容層或雙份規格目錄同步 — 延續前次已排除的方向，增加與上游分歧。
**Deferred**:
- Antigravity 與 Codex 共用 .agents/skills 時的內容生成及工具身分呈現方式 — 由後續 design 依既有渲染與同步機制處理，路徑正確、共存與清理要求已確定。
- Herdr 的實際安裝、外部設定修改與啟用 — 留待使用者另行操作或指定，不作為程式整合的自動動作。
- 整合後版本號、公開發佈時間及發布 — 待整合驗收完成後決定。
- OpenSpec CLI 全流程與其他生態工具的完整互通保證 — 延續前次討論的未驗證範圍，本次不擴大承諾。
**Capture to**: proposal | design | spec | tasks
**Next**: $speclink-propose --from-discussion merge-upstream-0-12-0
