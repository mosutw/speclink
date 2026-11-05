# claude-code-session-focus Specification

## Purpose

在 Claude Code 技能列與對話標題顯示目前執行的 Speclink 步驟與變更，支援 worktree、清空與接回對話，以及封存完成標示。焦點是對話內的顯示資訊，不改動變更生命週期。

## Requirements

### Requirement: 從使用者指令與工具呼叫辨識焦點

送出的訊息最前方是 Speclink 技能指令時，焦點 SHALL 改為該步驟；合併指令 SHALL 顯示 archive+commit。已知變更名稱 SHALL 以完整詞比對並取最後一個；變更操作技能沒帶名稱 SHALL 保留先前變更，一般訊息、面板指令及非 Speclink 指令 SHALL 不換步驟。discuss、propose、improve、audit、trace、manual、baseline、config 技能 SHALL 清除前一步變更焦點，不把指令或背景查詢提及的既有變更認作正在處理的變更；其中 propose 後續建立變更時 SHALL 允許由主代理 Speclink CLI 操作識別新變更。其他變更技能的主代理工具呼叫 SHALL 更新辨識出的變更。子代理呼叫 SHALL 不改主對話焦點。

#### Scenario: 沒帶名稱的下一步

- **WHEN** 先送出 /speclink-apply add-auth，再送出 /speclink-quality
- **THEN** 步驟改為 quality，變更仍為 add-auth；之後送出「繼續工作」不換焦點

#### Scenario: 不將名稱片段當成變更

- **WHEN** 目前在 apply，已知變更為 add-auth，文字為 cat add-auth-notes.md
- **THEN** 不辨識為 add-auth；speclink status --change add-auth --json 則辨識為 add-auth

#### Scenario: 合併指令與一般提問

- **WHEN** 送出 /speclink-archive + /speclink-commit add-auth
- **THEN** 步驟是 archive+commit；送出「幫我看 /speclink-apply 怎麼用」不改步驟

#### Scenario: 子代理工具呼叫

- **WHEN** 子代理呼叫 Speclink 技能或 Bash 指令
- **THEN** 主對話的步驟與變更焦點維持原值

#### Scenario: 討論不繼承正在做的變更

- **WHEN** 前一步為 apply add-auth，接著使用 discuss 並查詢 add-auth 作為背景
- **THEN** 焦點步驟為 discuss，變更為空，不把 add-auth 回報成正在處理的變更

#### Scenario: 提案建成後取得新焦點

- **WHEN** 從 propose 開始且變更焦點為空，主代理建立 add-login 並查詢該變更
- **THEN** 焦點綁定新建 add-login，後續技能列與回報能使用該名稱


<!-- @trace
source: merge-upstream-0-12-0
updated: 2026-11-05T23:11:44+08:00
-->

---
### Requirement: 技能列與對話標題更新時機

有焦點時，技能列 SHALL 顯示目前步驟及變更；送出訊息時 SHALL 將已知資訊組成對話標題 `步驟 · 變更`，僅一項已知時顯示該項。工具執行中辨識出新變更時 SHALL 立即更新技能列，對話標題 SHALL 等下次送出訊息再更新。沒有焦點時 SHALL 不新增焦點文字，也不改對話標題。

#### Scenario: 工具中途換變更

- **WHEN** 送出 `/speclink-apply add-auth` 後，執行 `speclink status --change "refactor-store" --json`
- **THEN** 技能列立即顯示 refactor-store，下次送出 `繼續工作` 時標題成為 `apply · refactor-store`

#### Scenario: 尚未使用 Speclink

- **WHEN** 無工作焦點的對話送出 `幫我看一下 README`
- **THEN** 技能列不呈現焦點標記，對話標題維持原樣

Evidence: `integrations/claude-code/speclink-skills/hooks/focus.ts`、`integrations/claude-code/speclink-skills/hooks/register.tsx`；`integrations/claude-code/speclink-skills/tests/register.test.tsx` 的標題更新與無焦點測試。

---
### Requirement: 從 worktree 與接回資訊恢復焦點

對話尚無焦點時，若所在目錄為已知變更的 worktree 或其子目錄，SHALL 以該變更作為焦點；否則 SHALL 先從對話紀錄恢復最後步驟與變更，再嘗試辨識原標題。標題中的變更 SHALL 是現有變更，或是帶步驟及完成記號 `✓` 的已封存標題；其他自訂標題 SHALL 不當成變更。

#### Scenario: 在 worktree 開始

- **WHEN** add-auth 的 worktree 是 `/repo/.worktrees/add-auth`，在該目錄開始對話
- **THEN** 技能列一開始顯示 add-auth

#### Scenario: 接回標題

- **WHEN** resume 接回的標題是 `review · refactor-store`，且 refactor-store 是現有變更
- **THEN** 恢復 review 與 refactor-store，不呈現封存完成記號

#### Scenario: 拒絕未知未封存名稱

- **WHEN** 現有變更不含 gone-change，原標題是 `apply · gone-change`
- **THEN** 不從標題恢復焦點；`archive · gone-change ✓` 則可恢復已封存焦點

Evidence: `integrations/claude-code/speclink-skills/hooks/focus.ts`、`integrations/claude-code/speclink-skills/hooks/register.tsx`；`integrations/claude-code/speclink-skills/tests/register.test.tsx` 的 worktree、歷史紀錄與 resume 測試。

---
### Requirement: clear 後只清除 mod 可辨識的舊標題

clear 後 SHALL 不顯示舊焦點；若帶入的標題可辨識為 mod 工作焦點，SHALL 等下一次送出訊息、且沒有新焦點時將標題改回 `Claude Code`。不可辨識的自訂標題 SHALL 保留。clear 當下 SHALL 不回傳標題修改。

#### Scenario: 清除 mod 的標題

- **WHEN** clear 帶入 `apply · add-auth`，下一次送出 `幫我看 README`
- **THEN** clear 當下不改標題，下次送出時標題為 `Claude Code`，技能列不顯示舊焦點

#### Scenario: 保留自訂標題

- **WHEN** clear 帶入 `我的工作`，下一次送出 `幫我看 README`
- **THEN** 不回傳標題修改

Evidence: `integrations/claude-code/speclink-skills/hooks/register.tsx`、`integrations/claude-code/speclink-skills/hooks/focus.ts`；`integrations/claude-code/speclink-skills/tests/register.test.tsx` 的 clear 測試。

---
### Requirement: 依封存後清單標示完成

主代理的 `speclink archive` 呼叫完成後 SHALL 重讀變更清單，以封存前已知名稱辨識目標；目標不再出現在新清單時 SHALL 於焦點顯示 `✓`，下次送出訊息時標題 SHALL 同樣帶 `✓`。清單讀取失敗 SHALL 視為沒有可用的已知變更；此顯示不是成功封存證據，亦 SHALL 不執行額外封存、認證或 revision 寫入。

本能力 SHALL 不新增 CLI 子指令、旗標、stdin、exit code、stdout／stderr、JSON 輸出或檔案寫入；proposal.md、delta spec、.meta.yaml、任務 evidence 與 snapshots 不因焦點更新而建立或修改。焦點步驟與變更名稱 SHALL 保留原指令文字，不隨工作流 `tw`／`ja`／`en` 或未設定值翻譯；其他介面文字依 claude-code-skill-bar 的中英選擇及中文弱偵測。`--no-color` 不控制焦點 UI。本基準 SHALL 記錄既有顯示與判定，不改變既有 CLI 人眼與 JSON 位元級輸出。

#### Scenario: 目標從清單消失

- **WHEN** 先送出 `/speclink-archive add-auth`，封存呼叫完成後清單不再含 add-auth
- **THEN** 技能列顯示 add-auth 與 `✓`；下次送出訊息時標題是 `archive · add-auth ✓`

#### Scenario: 重讀失敗的判定限制

- **WHEN** 已知目標為 add-auth，archive 呼叫後重讀清單因離線或認證失效而失敗
- **THEN** 已知清單清空，目標被判為不在清單而顯示 `✓`，不能據此確認遠端封存成功

Evidence: `integrations/claude-code/speclink-skills/hooks/register.tsx` 的清單失敗與封存後判定；`integrations/claude-code/speclink-skills/hooks/focus.ts` 的標題呈現；`integrations/claude-code/speclink-skills/tests/register.test.tsx` 的封存完成測試。清單失敗情境由已讀取程式確認，沒有宣稱已實跑遠端失敗情境。


<!-- @trace
source: merge-upstream-0-11-0
updated: 2026-10-29T05:48:54+08:00
-->

---
### Requirement: Herdr 內即時回報已確認的變更

Claude Code 外掛 SHALL 在確認當前變更時即時更新 Herdr 的 speclink.focus 標籤，供面板與 Agents 清單使用；沒有變更或已切到非變更技能時 SHALL 不保留錯誤的上一個工作歸屬。此整合 SHALL 僅在有 Herdr 環境時執行，沒有 Herdr 時 SHALL 保持既有技能列、標題與操作流程，不為回報自動安裝 Herdr 或寫入使用者設定。

#### Scenario: 名稱確認即回報

- **WHEN** 在 Herdr 內的 Claude Code session 從未指定名稱的 apply 經主代理 CLI 操作確認 add-auth
- **THEN** 面板可在下一次資料更新時由 speclink.focus 辨識該 Agent 正處理 add-auth，無須等待整個技能完成

#### Scenario: 沒有 Herdr 保持原功能

- **WHEN** 在一般 Claude Code 環境送出 apply add-auth
- **THEN** 技能列與對話標題仍依既有規則更新，不因缺少 Herdr 回報介面而失敗，不建立外部設定檔

<!-- @trace
source: merge-upstream-0-12-0
updated: 2026-11-05T23:11:44+08:00
-->