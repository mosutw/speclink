## MODIFIED Requirements

### Requirement: 依封存後清單標示完成
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

主代理的 `speclink archive` 呼叫完成後 SHALL 重讀變更清單，以封存前已知名稱辨識目標；目標不再出現在新清單時 SHALL 於焦點顯示 `✓`，下次送出訊息時標題 SHALL 同樣帶 `✓`。清單讀取失敗 SHALL 視為沒有可用的已知變更；此顯示不是成功封存證據，亦 SHALL 不執行額外封存、認證或 revision 寫入。

本能力 SHALL 不新增 CLI 子指令、旗標、stdin、exit code、stdout／stderr、JSON 輸出或檔案寫入；proposal.md、delta spec、.meta.yaml、任務 evidence 與 snapshots 不因焦點更新而建立或修改。焦點步驟與變更名稱 SHALL 保留原指令文字，不隨工作流 `tw`／`ja`／`en` 或未設定值翻譯；其他介面文字依 claude-code-skill-bar 的中英選擇及中文弱偵測。`--no-color` 不控制焦點 UI。本基準 SHALL 記錄既有顯示與判定，不改變既有 CLI 人眼與 JSON 位元級輸出。

#### Scenario: 目標從清單消失

- **WHEN** 先送出 `/speclink-archive add-auth`，封存呼叫完成後清單不再含 add-auth
- **THEN** 技能列顯示 add-auth 與 `✓`；下次送出訊息時標題是 `archive · add-auth ✓`

#### Scenario: 重讀失敗的判定限制

- **WHEN** 已知目標為 add-auth，archive 呼叫後重讀清單因離線或認證失效而失敗
- **THEN** 已知清單清空，目標被判為不在清單而顯示 `✓`，不能據此確認遠端封存成功

Evidence: `integrations/claude-code/speclink-skills/hooks/register.tsx` 的清單失敗與封存後判定；`integrations/claude-code/speclink-skills/hooks/focus.ts` 的標題呈現；`integrations/claude-code/speclink-skills/tests/register.test.tsx` 的封存完成測試。清單失敗情境由已讀取程式確認，沒有宣稱已實跑遠端失敗情境。
