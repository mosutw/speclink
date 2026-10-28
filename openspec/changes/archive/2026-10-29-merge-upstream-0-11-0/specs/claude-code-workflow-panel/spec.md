## MODIFIED Requirements

### Requirement: 面板開關與名稱填入
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

使用者點面板按鈕或送出 `/speclink-panel` 時，面板 SHALL 在開啟與關閉間切換；斜線指令沒有額外旗標或 stdin，也不是新增的 speclink CLI 子指令。開啟時 SHALL 取得看板資料。點擊變更、下一步、討論或工單名稱 SHALL 將名稱設為輸入框最前方指令的參數；沒有指令時 SHALL 接在現有文字後方，不自動執行。

此 UI SHALL 沿用 claude-code-skill-bar 的語言選擇：明選 en／zh-TW 優先，auto 對中文、chinese、zh 作弱偵測，其餘（含日文與未設定）選英文，不讀工作流 `tw`／`ja`／`en`。面板操作 SHALL 不建立或修改 proposal.md、delta spec、.meta.yaml、任務 evidence 或 snapshots。本基準 SHALL 記錄既有呈現，不變更 CLI 人眼或 JSON 位元級輸出；面板沒有自己的 stdout／stderr、exit code、`--json` 或 `--no-color` 模式。

#### Scenario: 開關回應

- **WHEN** 繁中介面送出 `/speclink-panel`
- **THEN** 未開啟時開啟並回覆 `speclink 面板已開啟。`，已開啟時關閉並回覆 `speclink 面板已關閉。`

#### Scenario: 點擊名稱

- **WHEN** 輸入框是 `/speclink-apply old-one`，點擊 add-auth
- **THEN** 輸入框成為 `/speclink-apply add-auth`；輸入框原為 `look at` 時則成為 `look at add-auth`

Evidence: `integrations/claude-code/speclink-skills/hooks/register.tsx`、`integrations/claude-code/speclink-skills/hooks/skills.ts`、`integrations/claude-code/speclink-skills/hooks/text.ts`；`integrations/claude-code/speclink-skills/tests/register.test.tsx` 的面板開啟與名稱填入測試。關閉回應由已讀取程式確認。
