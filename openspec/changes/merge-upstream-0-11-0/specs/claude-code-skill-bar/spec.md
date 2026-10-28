## MODIFIED Requirements

### Requirement: 點技能只填入指令並保留參數
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

點擊技能 SHALL 將對應斜線指令放在輸入框最前方，取代原先最前方的斜線指令並保留後方文字；SHALL 不自動送出或執行指令。`archive+commit` SHALL 填入 `/speclink-archive + /speclink-commit`，更換技能時 SHALL 將兩個指令一起取代。此操作 SHALL 不建立或修改 proposal.md、delta spec、.meta.yaml、任務 evidence 或 snapshots；這些效果屬之後送出指令的工作流程。

#### Scenario: 保留原有參數

- **WHEN** 輸入框是 `/speclink-review add-x`，使用者選擇 verify
- **THEN** 輸入框成為 `/speclink-verify add-x`

#### Scenario: 換掉合併指令

- **WHEN** 輸入框是 `/speclink-archive + /speclink-commit add-x`，使用者選擇 apply
- **THEN** 輸入框成為 `/speclink-apply add-x`

Evidence: `integrations/claude-code/speclink-skills/hooks/skills.ts`、`integrations/claude-code/speclink-skills/hooks/register.tsx`；`integrations/claude-code/speclink-skills/tests/register.test.tsx` 的指令填入、合併指令與技能列互動測試。
