## ADDED Requirements

### Requirement: 已封存頁的流程成本摘要

已封存頁 SHALL 在頁標題區與列表卡之間顯示「流程成本」摘要，以全部已封存變更（不含已封存討論，且不受頁內搜尋影響）的流程成本欄位計算，最多四項：

- 文件行數：`docLines` 的中位數與 P90
- 任務數：`tasksTotal` 的中位數
- 建立到封存：`leadDays` 的中位數（天）；有 `leadHours` 樣本時，同一項另顯示 `leadHours` 的中位數（小時）
- 品質關卡輪數：`reviewRounds` 與 `verifyRounds` 各自的中位數，分別標為審查與驗證

每個數字 SHALL 標出樣本數，也就是帶有該欄位的封存變更筆數。樣本數為 0 的數字 SHALL NOT 顯示；一項內所有數字都沒有樣本時，該項 SHALL NOT 顯示；沒有任何封存變更帶 `docLines`、`leadDays`、`leadHours`、`reviewRounds` 或 `verifyRounds` 時（包含沒有任何封存變更，以及清單只帶 `tasksTotal`、不帶這些欄位的遠端工作區），整個摘要 SHALL NOT 顯示，即使任務數有樣本。中位數 SHALL 為由小到大排序後的中間值，筆數為偶數時取中間兩值的平均；P90 SHALL 為由小到大第 ⌈0.9 × n⌉ 個值。數字 SHALL 以最多一位小數顯示，整數不帶小數點。摘要文案 SHALL 跟隨 UI 語言：繁中為「流程成本」「文件行數」「任務數」「建立到封存」「品質關卡輪數」「審查」「驗證」「中位數」「P90」「天」「小時」「筆」；英文為 "Process cost"、"Doc lines"、"Tasks"、"Created to archived"、"Quality gate rounds"、"Review"、"Verify"、"median"、"P90"、"days"、"hours"、"changes"，數量為 1 時用單數形 "day"、"hour"、"change"。摘要 SHALL 為唯讀，SHALL NOT 提供任何寫入動作。

#### Scenario: 顯示中位數、P90 與樣本數

- **WHEN** 本機工作區有 3 筆封存變更，`docLines` 分別為 100、200、400，`tasksTotal` 分別為 4、6、9，`leadDays` 分別為 0、0、3
- **THEN** 已封存頁的流程成本摘要顯示文件行數中位數 200、P90 400、3 筆，任務數中位數 6、3 筆，建立到封存中位數 0 天、3 筆

#### Scenario: 偶數筆取中間兩值平均

- **WHEN** 本機工作區有 2 筆封存變更，`leadDays` 分別為 0 與 1，`leadHours` 分別為 6 與 30.5
- **THEN** 建立到封存顯示中位數 0.5 天、2 筆，以及中位數 18.3 小時、2 筆

#### Scenario: 舊記錄沒有輪數時不顯示該項

- **WHEN** 所有封存變更都沒有 `reviewRounds` 與 `verifyRounds`
- **THEN** 流程成本摘要不顯示品質關卡輪數這一項，其他有樣本的項目照常顯示

#### Scenario: 沒有資料時整個摘要不出現

- **WHEN** 已封存頁沒有任何封存變更，或處於清單只帶 `tasksTotal`、不帶流程成本欄位的遠端工作區
- **THEN** 頁標題區與列表卡之間沒有流程成本摘要，列表卡與分頁照常顯示

#### Scenario: 搜尋不改變摘要數字

- **WHEN** 使用者在已封存頁的搜尋框輸入只命中 1 筆封存變更的字串
- **THEN** 列表只顯示命中的那一筆，流程成本摘要的數字與樣本數維持全部封存變更的計算結果

### Requirement: 技能檔提示承載規格資料夾遷移

桌面技能更新提示 SHALL 依共用路徑與遷移規劃結果顯示；明寫或待補 spec_dir: openspec 的專案更新 SHALL 不被描述為將搬到 speclink/。使用者觸發更新時 SHALL 經共用 update 流程先補預設，再決定遷移並同步技能；更新後 watcher、快取與開啟的文件以實際規格路徑繼續運作。確實會遷移的其他佈局沿用上游改名前說明及完成提示；失敗顯示原因，不能顯示成功。

#### Scenario: 預設專案更新不中斷閱讀
- **WHEN** 桌面開啟缺少 spec_dir、實際為 openspec/ 的專案並執行技能更新
- **THEN** .speclink.yaml 補入 spec_dir: openspec，資料夾未搬移，提示不聲稱改名，原看板、文件與外部檔案變動仍可更新

#### Scenario: 設定無法補值時顯示失敗
- **WHEN** 更新前設定無法解析或補值失敗
- **THEN** 畫面顯示失敗原因、不顯示更新完成，仍使用原專案與原資料夾

## MODIFIED Requirements

### Requirement: GUI 勾任務與 CLI 完成語意一致
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

桌面看板勾選任務為完成 SHALL 產生與 speclink task done 相同的檔案效果：tasks.md 該任務勾章、touched-files 記錄（有未被先前任務認領的 git dirty 檔時追加，無則不追加——與 CLI 同語意）、該 change 首次有任務完成且 meta 無 started_* 時蓋開工章（started_at 為當日、started_by 依 git 身分可得性、started_with 缺席）。對已完成任務的重複完成請求 SHALL 視為冪等成功，SHALL NOT 寫入任何檔案、SHALL NOT 對使用者報錯。取消勾選與拖曳排序 SHALL 僅寫 tasks.md，SHALL NOT 寫入 meta 或 touched 記錄。

#### Scenario: 勾選首任務蓋章並移欄

- **WHEN** 使用者於看板勾選某 meta 無 started_* 的 change 的第一項任務
- **THEN** tasks.md 該任務成 [x]，.meta.yaml 新增 started_at（git 身分可得時含 started_by），看板刷新後卡片移入「進行中」欄且詳情頁顯示開工列

#### Scenario: 取消勾選不動 meta 與 touched

- **WHEN** 使用者取消勾選一項已完成任務
- **THEN** 僅 tasks.md 該行標記變為 [ ]；.meta.yaml 與 touched 記錄逐字元不變

#### Scenario: 拖曳排序不觸發完成語意

- **WHEN** 使用者拖曳任務改變順序（含跨群組重編號）
- **THEN** 僅 tasks.md 變動；.meta.yaml 與 touched 記錄逐字元不變

### Requirement: 詳情頁的排程分頁
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

change 詳情頁 SHALL 於左欄「文件」節點「規格」之後提供「排程」節點（tw 標籤「排程」、en「Plan」），點擊 SHALL 於閱讀卡呈現排程文件。清單項帶 `wave` 時閱讀卡 SHALL 依序呈現四張卡片（與審查／驗證文件同一種邊框卡片：卡頭與內容區以分隔線分格；卡頭為 16px 粗體標題帶本頁目錄錨點，重疊卡與封存順序卡的卡頭右端附筆數籤），data-plan-section 依序為 wave、depends、overlaps、archive：波次卡（「第 N 波」＋同波其他變更名稱清單，無其他時顯示「本波只有這個變更」）；前置卡（卡頭右側 SHALL 有狀態徽章——`blockedBy` 空時「可以開工」、非空時「等 N 項」且 title 列出前置名；內容為 dependsOn 每項一列，資料源 capability `setDepends` 為 true 時每列附移除鈕、底部附下拉選單列出候選變更供新增，選擇即寫入；空時「尚無前置」；`blockedBy` 含 `dependsOn` 以外的名稱（舊 server 以目錄級重疊判定阻擋）時 SHALL 另以一列唯讀列出這些名稱（tw「另需等待（server 依重疊判定）：」，不附移除鈕），讓徽章數字對得上卡片內容）；重疊卡（`requirementOverlap` 每項一列：對方變更名、`capability › requirement` 文字、雙方操作各一枚小標籤（用詞與配色同規格文件的 delta 區段，如 MODIFIED 顯示「修改」；title 為該操作所屬的變更名），`conflict` 為 true 時加「同名衝突」標籤；空時「無重疊」；SHALL NOT 顯示目錄級 `overlaps`）；封存順序卡（`archiveAfter` 每項變更名，下方一句「先封存它們，再對照正式規格重寫同名 requirement 後封存本變更」；其中有變更也把本變更列入它的 `archiveAfter`（互相等待）時 SHALL 另加一句警示（tw「與 {names} 互相等待：兩邊都要先等對方封存，須調整拆分」）；`requirementOverlap` 有 `conflict` 為 true 的列時 SHALL 加一句同名衝突警示（tw「與 {names} 同名衝突：先改掉其中一邊，較晚封存的一方才不會被拒」）；`archiveAfter` 為空且無同名衝突時「可直接封存」）。排程文件 SHALL NOT 渲染獨立的「阻擋」段。新增候選 SHALL 為該變更所在名冊中的其他作用中變更、扣除已宣告的前置：資料源提供名冊查詢時（local）SHALL 以查詢結果為準——該變更有 worktree 映射時名冊為其 worktree 副本、無映射時為主 checkout，與前置寫入作用的 store 相同；名冊查詢未完成時 SHALL NOT 渲染新增下拉；資料源未提供名冊查詢（remote，scope 只有一份名冊）或首次查詢（開啟詳情頁、切換變更）失敗時 SHALL 自作用中變更清單派生；看板刷新後的重新查詢失敗時 SHALL 沿用前一次的查詢結果。新增或移除前置 SHALL 經資料源 `setDepends` 寫回該變更 meta 的 depends_on，成功後看板與排程文件刷新；引擎拒絕（自依賴、不存在、已封存、成環）時 SHALL 以單行錯誤呈現引擎訊息、畫面不變。capability 為 false 時 SHALL NOT 渲染移除鈕與下拉選單。清單項缺 `wave` 且缺 `dependsOn`（remote 清單未取得 plan：舊 server、依賴成環或請求失敗）時閱讀卡 SHALL 只顯示一句說明（tw「此模式尚未提供排程資訊」），不渲染四卡；缺 `wave` 但帶 `dependsOn`（local plan 成環）時 SHALL 顯示一句說明（tw「依賴成環，先移除一條前置才有排程資訊」）並只渲染前置卡（無狀態徽章；`dependsOn` 為空時照樣顯示「尚無前置」；capability 為 true 時附移除鈕、SHALL NOT 渲染新增下拉），讓使用者解環。`requirementOverlap` 與 `archiveAfter` 缺席但 `wave` 存在（舊 server）時重疊卡與封存順序卡 SHALL 顯示各自的空態文案。判定 SHALL 收斂於 packages/ui 階段派生模組的 planWave／planBlockedBy／planRequirementOverlap／planArchiveAfter 單一入口。

清單項帶 `wave` 時閱讀卡 SHALL 另有：籤槽為前置卡同一枚狀態徽章（「可以開工」或「等 N 項」）；數字磚依序「波次」（N）、「宣告的前置」（`dependsOn` 筆數）、「同名需求重疊」（`requirementOverlap` 筆數）、「封存前要等」（`archiveAfter` 筆數）；一個提示框，標題由兩個子句組成——`blockedBy` 空為「可以開工」、非空為「等 N 項前置」；`archiveAfter` 空且無同名衝突為「可直接封存」、`archiveAfter` 非空為「封存前要等 M 個變更」、有同名衝突為「有同名衝突要先改」——兩個子句皆為綠態時提示框為 tip、否則為 warning；內文列出前置名、要先封存的變更名與同名衝突的對方變更名，並說明排程於看板刷新時重算。缺 `wave` 的兩種情況 SHALL NOT 渲染籤槽、數字磚與提示框。

#### Scenario: 排程分頁四段

- **WHEN** 開啟 wave=2、dependsOn=["add-a"]、blockedBy=["add-a"]、requirementOverlap=[{change:"add-b",capability:"desktop-app",requirement:"看板與任務",ownOperation:"MODIFIED",otherOperation:"MODIFIED",conflict:false}]、archiveAfter=["add-b"] 的變更詳情頁並點左欄「排程」節點
- **THEN** 閱讀卡依序顯示波次卡「第 2 波」、前置卡（徽章「等 1 項」、前置列 add-a 附移除鈕、新增下拉）、重疊卡一列（add-b、`desktop-app › 看板與任務`、兩枚「修改」標籤）、封存順序卡（add-b 與提示句），頁面無 data-plan-section="blocked"；四卡頭為 16px 粗體且進本頁目錄

#### Scenario: 新增前置寫回

- **WHEN** 於排程文件的下拉選單選擇 add-c
- **THEN** 該變更 .meta.yaml 的 depends_on 含 add-c，前置卡出現 add-c，看板順序刷新

#### Scenario: 引擎拒絕新增

- **WHEN** 於排程文件選擇會成環的變更
- **THEN** 顯示引擎的成環單行錯誤，前置卡不變，meta 檔不變

#### Scenario: 成環時可移除前置

- **WHEN** 清單 planError 非 null，開啟 dependsOn=["add-a"]（無 wave）的變更詳情頁並點「排程」節點
- **THEN** 顯示「依賴成環，先移除一條前置才有排程資訊」與前置卡的 add-a 列（附移除鈕），無狀態徽章、無新增下拉、無波次／重疊／封存順序卡；按移除鈕即寫回

#### Scenario: remote 分頁唯讀

- **WHEN** 於 capability setDepends 為 false 的資料源開啟詳情頁的排程文件
- **THEN** 不出現移除鈕與下拉選單，四卡照常顯示

#### Scenario: worktree 映射時候選來自副本名冊

- **WHEN** 主 checkout 且 worktree 政策開啟，add-dark-mode 已映射到 worktree，add-auth 在分支前建立、add-late 在分支後才於主 checkout 建立，開啟 add-dark-mode 的排程文件
- **THEN** 新增下拉選單含 add-auth、不含 add-late，看板清單仍列出 add-late

#### Scenario: 重新查詢名冊失敗沿用前次結果

- **WHEN** 排程文件已以名冊 add-a、add-b 呈現候選（作用中變更清單另有 add-d，add-a 已是前置），看板刷新後名冊查詢失敗
- **THEN** 新增下拉的候選仍只有 add-b，不出現 add-d

#### Scenario: 可開工徽章與同名衝突標籤

- **WHEN** 開啟 wave=1、blockedBy=[]、requirementOverlap=[{change:"add-x",capability:"auth",requirement:"登入",ownOperation:"ADDED",otherOperation:"ADDED",conflict:true}]、archiveAfter=[] 的變更詳情頁並點「排程」節點
- **THEN** 前置卡徽章為「可以開工」，重疊卡的 add-x 列帶「同名衝突」標籤與兩枚「新增」標籤，封存順序卡顯示「與 add-x 同名衝突：先改掉其中一邊，較晚封存的一方才不會被拒」且不顯示「可直接封存」

#### Scenario: 舊 server 缺新欄位的空態

- **WHEN** remote 清單項帶 wave 與 blockedBy 但無 requirementOverlap 與 archiveAfter
- **THEN** 重疊卡顯示「無重疊」、封存順序卡顯示「可直接封存」，其餘兩卡照常

#### Scenario: 舊 server 的重疊阻擋列在前置卡

- **WHEN** remote 清單項 wave=2、dependsOn=[]、blockedBy=["add-a"]（舊 server 把先配置、共用 capability 的 add-a 算進阻擋），無 requirementOverlap 與 archiveAfter
- **THEN** 前置卡徽章「等 1 項」、內容「尚無前置」與一列「另需等待（server 依重疊判定）：add-a」，該列無移除鈕；本機與新 server 的 blockedBy 只含宣告前置，不出現這一列

#### Scenario: 互相等待時封存卡警示

- **WHEN** add-a 的 archiveAfter=["add-b"]、add-b 的 archiveAfter=["add-a"]，開啟 add-a 的排程文件
- **THEN** 封存順序卡列 add-b 與提示句，另顯示「與 add-b 互相等待：兩邊都要先等對方封存，須調整拆分」；單向的 archiveAfter 不出現這句警示

#### Scenario: 成環時環外變更的前置卡空態

- **WHEN** 清單 planError 非 null，開啟 dependsOn=[]（無 wave）的環外變更詳情頁並點「排程」節點
- **THEN** 顯示「依賴成環，先移除一條前置才有排程資訊」與前置卡「尚無前置」，無狀態徽章、無新增下拉

#### Scenario: 排程的數字磚與提示框兩態

- **WHEN** 開啟排程分頁四段 scenario 的變更（wave=2、dependsOn 與 blockedBy 各一、重疊一列、archiveAfter 一個）並點「排程」節點；再開啟 wave=1、dependsOn=[]、blockedBy=[]、requirementOverlap=[]、archiveAfter=[] 的變更
- **THEN** 第一個變更的籤槽為「等 1 項」、數字磚「波次 2」「宣告的前置 1」「同名需求重疊 1」「封存前要等 1」、warning 提示框標題「等 1 項前置，封存前要等 1 個變更」且內文列出 add-a 與 add-b；第二個變更的籤槽為「可以開工」、數字磚「波次 1」「宣告的前置 0」「同名需求重疊 0」「封存前要等 0」、tip 提示框標題「可以開工，也可直接封存」

##### Example: 提示框標題的兩個子句

| blockedBy | archiveAfter | 同名衝突 | 標題 | 型 |
| --------- | ------------ | -------- | ---- | -- |
| [] | [] | 無 | 可以開工，也可直接封存 | tip |
| [add-a] | [] | 無 | 等 1 項前置，可直接封存 | warning |
| [] | [add-b] | 無 | 可以開工，封存前要等 1 個變更 | warning |
| [add-a] | [add-b] | 無 | 等 1 項前置，封存前要等 1 個變更 | warning |
| [] | [] | add-x | 可以開工，有同名衝突要先改 | warning |

#### Scenario: 缺 wave 時無數字磚與提示框

- **WHEN** 開啟 remote 清單項缺 wave 且缺 dependsOn 的變更詳情頁並點「排程」節點
- **THEN** 閱讀卡只顯示「此模式尚未提供排程資訊」，籤槽、數字磚與提示框皆缺席
