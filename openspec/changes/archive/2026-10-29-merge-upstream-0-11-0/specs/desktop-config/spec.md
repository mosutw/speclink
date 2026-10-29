## ADDED Requirements

### Requirement: 畫面依實際規格資料夾與資料檔名顯示

桌面 app 畫面上提到規格資料夾或變更資料檔的地方 SHALL 依專案實際使用的資料夾與檔名顯示，SHALL NOT 寫死 `openspec/` 或 `.openspec.yaml`：專案設定各頁標題下的檔案註記（例如 `openspec/config.yaml · context`；已使用上游目錄的專案為 `speclink/config.yaml · context`；明寫 `spec_dir` 時為該路徑）與分類欄的組名、討論詳情顯示的記錄路徑。遷移到遠端與來源選擇對話框 SHALL 以「本機規格資料夾」稱呼本機的規格資料夾。變更資料檔解析失敗的卡片提示 SHALL 以「變更資料檔」稱呼，不帶寫死的檔名。新專案的說明文字（初始化對話框、空狀態與來源選擇）SHALL 寫 `openspec/`。文案 SHALL 跟隨 UI 語言。

#### Scenario: 新佈局專案的檔案註記

- **WHEN** 使用者在規格資料夾為 openspec/ 的專案打開專案設定的專案說明頁
- **THEN** 頁標題下的註記為 `openspec/config.yaml · context`，分類欄組名為 `openspec/config.yaml`

#### Scenario: 暫緩遷移的舊佈局專案

- **WHEN** 使用者在仍是 openspec/ 的專案打開專案設定的產出規則頁
- **THEN** 頁標題下的註記為 `openspec/config.yaml · rules`

#### Scenario: 討論詳情的記錄路徑

- **WHEN** 使用者在 openspec/ 專案打開討論 board-search-bar 的詳情
- **THEN** 顯示的記錄路徑為 `openspec/discussions/board-search-bar.md`

#### Scenario: 資料檔解析失敗的提示

- **WHEN** 某變更的資料檔無法解析，使用者在看板看到該卡片
- **THEN** 卡片提示以「變更資料檔」稱呼並附解析原因，不出現 `.openspec.yaml` 字樣

### Requirement: 專案設定顯示規格資料夾模式並確認單向轉換

本機專案設定的 .speclink.yaml 頁 SHALL 在既有「設定檔」區塊顯示「規格資料夾」選項，以實際路徑選取 OpenSpec（openspec/，預設）或 Speclink（speclink/）。缺鍵但實際使用 openspec 的專案 SHALL 顯示 OpenSpec，已使用 speclink 的專案 SHALL 顯示 Speclink，不為顯示而改檔。自訂路徑 SHALL 唯讀顯示實際值、不強迫二選一；remote SHALL 不提供本機模式選擇或發出轉換要求。設定解析失敗、轉換執行中或工具選集尚未儲存時 SHALL 禁止發起轉換並顯示對應原因。

從 OpenSpec 選取 Speclink SHALL 先開確認對話框，內容明說「這會將規格資料夾從 openspec/ 轉換為 speclink/。轉換後無法轉回 OpenSpec 模式。」；英文介面 SHALL 呈現等義警告。確認框 SHALL 提供「取消」與「轉換成 Speclink 模式」，選取本身不得改變生效現值、搬資料或儲存工具。取消、Escape 或關閉確認框 SHALL 不呼叫轉換入口且零寫入。

使用者確認後 SHALL 只送出一次轉換要求、執行期間禁止重複提交；成功後顯示 Speclink、停用 OpenSpec 選項並說明無法轉回，重載實際路徑、設定、看板與開啟文件，持續監看 speclink/。拒絕或失敗 SHALL 顯示後端原因並依實際路徑重載現值，不可樂觀顯示成功。未完成轉換 SHALL 顯示未完成及「重試完成轉換」，依「規格資料夾模式的明確單向轉換」同向續作。既有工具、專案底色、安全政策及其他表單不被此操作隱含儲存或重設。

#### Scenario: 預設及既有模式如實顯示
- **WHEN** 使用者開啟實際路徑為 openspec/ 的專案設定
- **THEN** 設定檔區顯示規格資料夾選項與 OpenSpec 預設現值；實際為 speclink/ 時顯示 Speclink，自訂路徑唯讀顯示，讀取不產生檔案修改

#### Scenario: 選取 Speclink 先提示且取消零寫入
- **WHEN** OpenSpec 專案選取 Speclink，使用者隨後取消、按 Escape 或關閉確認框
- **THEN** 曾看到無法轉回 OpenSpec 的警告，轉換入口從未被呼叫，現值仍為 OpenSpec，資料夾、設定及未儲存工具選集保持原狀

#### Scenario: 確認後轉換與重新開啟
- **WHEN** 使用者明確確認轉換且操作成功，再次開啟專案設定
- **THEN** 現值仍是 Speclink，OpenSpec 選項停用且有無法轉回說明，看板與文件依 speclink/ 正常呈現，外部檔案修改可自動刷新

#### Scenario: 失敗不顯示成功且可重試收尾
- **WHEN** 轉換被守門拒絕，或新路徑生效後技能同步失敗
- **THEN** 顯示具體原因，前者維持 OpenSpec，後者顯示 Speclink 與未完成提示及同向重試，不把錯誤當成功或顯示空專案

#### Scenario: 語言與轉換限制
- **WHEN** tw 或 en 介面開啟模式選項與確認框，或專案是 remote／自訂路徑／設定解析失敗／存在未儲存工具修改
- **THEN** 文案依介面語言顯示，兩語言都明說不可逆；不支援轉換的情況不送出要求，工作流文件語系為 ja 或未設定也不改變原有 UI 語言決策

## MODIFIED Requirements

### Requirement: remote 模式的內建限縮與誤解析修正
<!-- BEFORE: 既有啟用與初始化不明寫預設路徑，亦未涵蓋上游的新佈局與雙內建流程。 -->
remote 模式下產出流程頁 SHALL 只列內建 schema（spec-driven 與 no-spec-change），切換下拉的可選目標 SHALL 只含這兩個內建（config 的名稱非內建時 SHALL 以停用項顯示現值——沿政策下拉未知值顯性呈現的既有模式，不可被選取）。remote 設定快照解析 schema 名稱時 SHALL NOT 讀取 client 本機的 user 層目錄——名稱為內建即以內嵌定義解析；非內建時 SHALL 顯性呈現「遠端自訂尚不支援」的狀態而非猜測，且產出規則分節不呈現猜測的固定鍵。

#### Scenario: remote 快照不讀本機 user 層
- **WHEN** remote 專案的 config 指定 schema 名稱 X，且 client 本機 user 層目錄恰有同名 schema
- **THEN** 設定快照不以本機定義解析 X；X 非內建時產出規則分節為空並顯示遠端自訂尚不支援的狀態

##### Example: remote 解析結果表
| config 的 schema 名稱 | 本機 user 層有同名 | 快照結果 |
|-----------------------|--------------------|----------|
| spec-driven | 否 | 內建定義解析，artifact 圖正常 |
| spec-driven | 是 | 內建定義解析（本機定義不參與） |
| no-spec-change | 是 | 內建定義解析（本機定義不參與），artifact 圖為 proposal、design、tasks |
| my-flow | 是 | 不解析，顯示遠端自訂尚不支援 |

#### Scenario: remote 下拉僅內建
- **WHEN** 工作區連線 remote store 且使用者開啟切換下拉
- **THEN** 可選的切換目標只有內建的 spec-driven 與 no-spec-change；config 的名稱非內建時以停用項顯示現值（沿政策下拉未知值顯性呈現的既有模式），不可被選取

### Requirement: 未初始化目錄經確認後自動初始化
<!-- BEFORE: 既有啟用與初始化不明寫預設路徑，亦未涵蓋上游的新佈局與雙內建流程。 -->

所選目錄向上探索未命中任何 speclink 專案時，app SHALL NOT 逕行寫入，而 SHALL 顯示初始化確認對話框（含 AI 工具多選 claude／codex／copilot，預設只勾選 claude）。使用者確認後 app SHALL 執行與 speclink init 等效的初始化（openspec/ 骨架含 specs/、changes/archive/ 與 config.yaml、專案根的 .speclink.yaml 記錄所選 tools 與 spec_dir: openspec、為每個所選工具生成 skills 檔），隨即切換至該專案；使用者取消時 app SHALL 維持原專案，且目標目錄 SHALL NOT 產生任何寫入。初始化失敗時 app SHALL 顯示單行錯誤訊息且 SHALL NOT 切換 root。

#### Scenario: 確認後初始化並切入新專案

- **WHEN** 使用者選定不含任何 speclink 標記的空目錄，於確認對話框保持預設（claude）並確認
- **THEN** 該目錄產生 openspec/（含 specs/、changes/archive/、config.yaml）、.speclink.yaml（tools 含 claude）與 .claude/skills/ 技能檔，不產生 CLAUDE.md，且 app 切換至該專案並於看板顯示空清單

#### Scenario: 勾選 codex 時生成對應工具檔

- **WHEN** 使用者於確認對話框加勾 codex 後確認
- **THEN** 目標目錄除 claude 對應檔案外，另產生 .agents/skills/ 技能檔而無 AGENTS.md，.speclink.yaml 的 tools 同時記錄 claude 與 codex

#### Scenario: 取消初始化則零寫入

- **WHEN** 使用者於確認對話框取消
- **THEN** app 維持原專案，所選目錄內容與選擇前完全相同（無任何新檔案或目錄）

#### Scenario: 勾選 copilot 時生成對應工具檔

- **WHEN** 使用者於確認對話框加勾 copilot 後確認
- **THEN** 目標目錄除 claude 對應檔案外，另產生 .github/skills/ 技能檔，不產生 .github/copilot-instructions.md、AGENTS.md，.speclink.yaml 的 tools 同時記錄 claude 與 copilot

### Requirement: 未啟用資料夾經確認後補齊啟用
<!-- BEFORE: 既有啟用與初始化不明寫預設路徑，亦未涵蓋上游的新佈局與雙內建流程。 -->

所選目錄向上探索命中 workspace、store mode 為本地檔案、且該 workspace root 不存在 `.speclink.yaml` 時，app SHALL 判定為未啟用 speclink，SHALL NOT 逕行寫入亦 SHALL NOT 直接以既有專案開啟，而 SHALL 顯示啟用確認對話框（含 AI 工具多選 claude／codex／copilot，預設只勾選 claude；文案為啟用語意，遵循 openspec/LANGUAGE.md、不出現工程詞）。判定與寫入 SHALL 錨定向上命中的 workspace root，而非使用者所選的子目錄。

使用者確認後 app SHALL 經引擎的工作區補齊入口執行啟用（補 openspec/ 骨架缺件、專案根 `.speclink.yaml` 記錄所選 tools 與 spec_dir: openspec、為每個所選工具生成 skills 檔，不生成指令檔），既有 openspec/ 內容 SHALL 零觸碰，隨即切換至該專案；使用者取消時 app SHALL 維持原專案，目標目錄 SHALL NOT 產生任何寫入。啟用失敗時 app SHALL 顯示單行錯誤訊息且 SHALL NOT 切換 root。`.speclink.yaml` 存在的專案 SHALL 照舊直接開啟，SHALL NOT 出現啟用對話框；向上探索完全未命中的目錄 SHALL 照舊走初始化確認流程。

#### Scenario: 遷移資料夾確認啟用後補齊並切入

- **WHEN** 使用者選定含 openspec/（內有既有規格文件）但無 .speclink.yaml 的資料夾，於啟用確認對話框保持預設（claude）並確認
- **THEN** 專案根產生 .speclink.yaml（tools 含 claude）與 .claude/skills/ 技能檔、不產生 CLAUDE.md，openspec/ 內既有文件位元級不變，app 切換至該專案並於看板呈現既有內容

#### Scenario: 取消啟用則零寫入

- **WHEN** 使用者於啟用確認對話框取消
- **THEN** app 維持原專案，所選資料夾內容與選擇前完全相同

#### Scenario: 已啟用專案不出現啟用對話框

- **WHEN** 使用者選定專案根含 .speclink.yaml 的資料夾開啟
- **THEN** app 直接開啟該專案進看板，無啟用對話框

#### Scenario: 子目錄開啟錨定專案根

- **WHEN** 使用者選定未啟用專案的子目錄開啟並確認啟用
- **THEN** .speclink.yaml 與工具檔產生於向上命中的專案根，app 切入該根

#### Scenario: 既有工作流設定不被覆蓋

- **WHEN** 未啟用資料夾的 openspec/config.yaml 已存在且含使用者自訂政策，使用者確認啟用
- **THEN** 該檔位元級不變，僅補齊其餘缺件

#### Scenario: 啟用時勾選 copilot

- **WHEN** 使用者選定含 openspec/ 但無 .speclink.yaml 的資料夾，於啟用確認對話框改為只勾 copilot 並確認
- **THEN** 專案根產生 .speclink.yaml（tools 僅含 copilot）與 .github/skills/ 技能檔，不產生 .claude/、.agents/、CLAUDE.md、AGENTS.md，openspec/ 內既有文件位元級不變
