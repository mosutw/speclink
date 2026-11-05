## Purpose

Herdr 終端工作區中的 Speclink 整合，涵蓋面板、技能交付、Agent 狀態、工作區側欄與系統語言。資料沿用既有 CLI 查詢與命令，不建立第二份工作流真相；能力只交付整合來源及安裝指引，不自動安裝或啟用使用者的 Herdr。

## ADDED Requirements

### Requirement: 面板跟隨工作區與資料狀態

Herdr 0.1.13 以上的 Speclink mod SHALL 提供停靠右欄與浮動面板，跟隨焦點終端所在 space 的專案，依 CLI 查詢結果呈現提案中、進行中、已就緒的變更、討論及已轉出討論。停靠開關 SHALL 由每個 space 分別記憶，未記錄時預設開啟；非停靠版面 SHALL 可用同一快捷鍵開關浮動面板。面板 SHALL 顯示波次、阻擋、任務進度、worktree 與 review／verify 的章；未轉完工作的已轉出討論 SHALL 留在可繼續轉出的討論群組。查詢失敗 SHALL 顯示錯誤，不把失敗當成空專案。

#### Scenario: 切換 space 恢復各自面板

- **WHEN** space A 的右欄開啟、space B 的右欄關閉，使用者來回切換焦點
- **THEN** 面板切換為對應專案資料並恢復各自開關，不把 A 的變更列顯示為 B 的工作

#### Scenario: 阻擋與章狀態

- **WHEN** add-b 等待 add-a，review 已蓋有效章、verify 工單仍開著
- **THEN** add-b 顯示等待對象，兩站各呈現真實狀態，不把開著工單當有效章

### Requirement: 技能推薦與送達

每列 SHALL 依階段、任務、章、worktree 與待重新反映狀態建議下一技能，展開鈕列出適用的其他技能。程式任務完成且有 worktree 時 SHALL 優先建議 worktree-merge；合併後兩站尚未有效蓋章時建議 quality，已就緒且兩站有效蓋章時建議 archive+commit；待重新反映時 SHALL 建議 ingest。archive+commit SHALL 依序送出封存與提交意圖，一次只排一份。面板 SHALL 不代替 CLI 的安全驗收、認證、revision 與封存守門，拒絕時不得自動加入旁路旗標或跳過失敗步驟。

技能 SHALL 按已確認的 Agent 標籤、worktree 與 Claude Code 回報決定送達對象。Agent 正等回答時 SHALL 不把技能當回答送入；沒有合適目標時依技能條件開新分頁或顯示不可用原因，不把背景提及當工作歸屬。

#### Scenario: worktree 優先合併

- **WHEN** 程式任務已完成、仍有 worktree、兩站已有章
- **THEN** 建議 worktree-merge，archive 與 archive+commit 不作為直接執行選項

#### Scenario: 不誤送等待回答的 Agent

- **WHEN** 對應 Agent 正等待回答，使用者點技能鈕
- **THEN** 不把技能送進回答位置，依送達條件開新分頁或顯示不可用原因

#### Scenario: 安全驗收仍阻擋封存

- **WHEN** review／verify 的章可用，但專案納入驗收條件的安全報告未滿足，使用者執行 archive+commit
- **THEN** 封存被既有守門拒絕，不自行接受風險、不加入繞過參數，也不跳過封存後直接提交

### Requirement: 開工與技能選單

開工 SHALL 在目前 space 開新 Agent；worktree 政策開啟時先讓使用者選 apply-with-worktree 或直接 apply。頂端技能鈕與快捷鍵 SHALL 開選單，只列符合條件的變更／討論，可開新分頁或一次放入輸入框；適用時可多選，archive+commit SHALL 不允許多選執行。列上的附加鈕 SHALL 僅將名稱放入目前輸入框，不送出；無 Agent 或正等回答時不可用。

#### Scenario: 開工選擇工作模式

- **WHEN** worktree 政策開啟，使用者點提案中變更的開工
- **THEN** 先顯示兩種模式，確認後在目前 space 開新 Agent，不自動切到另一個 space

#### Scenario: 輸入框僅附加名稱

- **WHEN** 點 add-auth 的附加鈕
- **THEN** 名稱放入目前輸入框而不執行命令，無可用 Agent 時鈕停用

### Requirement: Agent 追蹤與側欄呈現

變更的 Agent 比對 SHALL 依 speclink.change、worktree 路徑、speclink.focus 的順序，討論只以 speclink.discussion 標籤辨識。面板 SHALL 顯示並允許跳轉至工作者，區分做事中、閒置、完成及等待回答；閒置 Agent 不無故阻擋新工作。停靠版面的 Spaces 列 SHALL 依階段顯示數量，Agents 清單可依 space 及狀態分組、呈現階段與進度，並可切回原排序；有獨立分頁的 Agent 不重複列在父 Agent 下。

#### Scenario: 不把提及當成正在工作

- **WHEN** A 只在文字提及 add-auth，B 有 speclink.change=add-auth
- **THEN** 面板把 B 顯示為工作者，點擊跳到 B，不把 A 誤判為工作者

#### Scenario: 討論獨立追蹤

- **WHEN** A 有 speclink.discussion=talk-a，B 僅有變更焦點
- **THEN** talk-a 列只顯示 A，不從變更焦點猜測討論歸屬

### Requirement: 桌面導航與初始化提示

點變更／討論列 SHALL 用 speclink:// 開對應詳情，波次或路線圖鈕開路線圖，Logo 用 project 網址切到目前專案看板；非本機 space 或未初始化時 Logo 只喚起 app。只有明確未初始化才 SHALL 顯示 init 提示卡，按初始化在該專案新分頁執行原有 speclink init 並經原互動工具選擇；找不到 CLI、壞設定、離線或認證失效 SHALL 顯示錯誤，不以未初始化掩蓋，也不自動寫設定。

#### Scenario: Logo 開啟對應專案

- **WHEN** 本機 space 已初始化且使用者點 Logo
- **THEN** 桌面 app 開啟該專案看板，不改寫規格文件

#### Scenario: 壞設定不顯示初始化

- **WHEN** CLI 回報 .speclink.yaml 解析錯誤
- **THEN** 面板顯示錯誤原因，不自動初始化覆寫設定

### Requirement: 系統語言與安裝邊界

面板、技能選單及整合的 Herdr 內建畫面 SHALL 跟隨系統語言，弱中文偵測符合 zh／Chinese／中文時用繁中，其餘含日文與未設定用英文；不由工作流 tw／ja／en 決定 Herdr 語言。翻譯 SHALL 不改檔名、差異內容或未知字串。repo 提供安裝來源、版本需求與選用 hook 的中英指引及測試；建置、測試與整合 SHALL NOT 自動修改使用者 Herdr／Claude Code 設定或啟動其他 Agent。

#### Scenario: 中文系統與非中文系統

- **WHEN** 系統語言依序為 zh-TW、Chinese、ja-JP、en-US、未設定，工作流 locale 都是 tw
- **THEN** Herdr 介面依序為繁中、繁中、英文、英文、英文，名為 Settings 的檔案不被翻譯

#### Scenario: 測試不變更使用者環境

- **WHEN** 在未安裝 Herdr 的環境執行整合測試
- **THEN** 模型與 fixture 測試可執行，不建立使用者 Herdr 設定、不寫 Claude Code settings，也不啟動外部 Agent
