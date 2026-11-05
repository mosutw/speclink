## MODIFIED Requirements

### Requirement: built-in tools 權威收斂

<!-- BEFORE: 兩種工具原先各有實體技能目錄，取消任一工具會清理其全部受管技能。 -->

Workspace 工具同步 SHALL 將請求中的 Claude／Codex／Copilot／Antigravity 集合視為 built-in tools 的完整期望狀態。被選取的 built-in SHALL 生成或更新其 `speclink-*` Skills；未選取的 built-in SHALL 在該實體目錄沒有其他仍選取工具時移除 Speclink 產生的 Skills；Codex 與 Antigravity 共用 .agents/skills，只要任一仍選取就 SHALL 保留並同步共用技能，兩者皆未選取才 SHALL 清理該目錄下受管技能。未選取的 copilot 其移除範圍 SHALL 僅限 `.github/skills/` 下名稱以 `speclink-` 為前綴的目錄（因而變空的 `.github/skills/` 一併移除；同步前就是空的目錄 SHALL 保留）；`.github/` 下其他內容 SHALL 位元級不變。任一未選取 built-in 的 skills 目錄本身是 symlink 時，同步 SHALL NOT 透過它移除任何目錄，symlink 本身保留。無論選取與否，同步 SHALL 剝除 claude 與 codex 指令檔（`CLAUDE.md`、`AGENTS.md`）中遺留的 `SPECLINK:START..END` 區塊——SHALL 保留區塊外的使用者內容，且指令檔在剝除後全空時才刪除；不含區塊的指令檔 SHALL 位元級不變。copilot 沒有指令檔：同步 SHALL NOT 建立、改寫或刪除 `.github/copilot-instructions.md`。同步 SHALL 保留 `.speclink.yaml` 內的 custom descriptor、unknown tool entry、remote、spec_dir 與其他頂層鍵。相同期望狀態重試 SHALL 收斂到相同檔案結果，不破壞使用者內容。

#### Scenario: Claude 切換為 Codex並保留自訂工具

- **WHEN** `.speclink.yaml` 含 `claude`、一個 custom descriptor、remote section 與未知頂層鍵，`CLAUDE.md` 同時含遺留 Speclink 區塊和使用者文字，然後同步 built-in 選集 `[codex]`
- **THEN** `tools` 保留 custom descriptor 並將 built-in 集合改為僅 `codex`，remote 與未知鍵值不變，Codex Skills 被補齊，Claude Skills 被移除，`CLAUDE.md` 的遺留區塊被剝除且使用者文字仍存在

##### Example: built-in 選集轉換

| 原 built-in | 新選集 | 保留 custom descriptor | 受管結果 |
| --- | --- | --- | --- |
| claude | codex | 是 | 移除 Claude Skills、補齊 Codex Skills |
| claude,codex | claude | 是 | 更新 Claude Skills、移除 Codex Skills |
| codex | claude,codex | 是 | 補齊兩組 Skills |
| claude | claude,copilot | 是 | 更新 Claude Skills、補齊 Copilot Skills |
| claude,copilot | claude | 是 | 更新 Claude Skills、移除 Copilot Skills |
| codex | copilot | 是 | 移除 Codex Skills、補齊 Copilot Skills |

#### Scenario: 既有選集缺少產物時自動補齊

- **WHEN** `.speclink.yaml` 的 built-in tools 為 `[codex]`，但任一 Codex Skill 缺少，然後再次同步 `[codex]`
- **THEN** 缺少或過期的技能檔被補齊至正典內容，其他使用者檔案維持不變，且同步成功

#### Scenario: 壞設定在寫入前被拒

- **WHEN** `.speclink.yaml` 無法解析，然後請求同步任一 built-in 選集
- **THEN** 同步以單行解析錯誤失敗，原設定、Skills 與指令檔逐字元不變

#### Scenario: 更新時剝除內建工具的遺留 marker

- **WHEN** 舊版引擎注入過 marker 的專案（`CLAUDE.md` 含 SPECLINK 區塊與使用者段落）以 tools=[claude] 執行新版 speclink update
- **THEN** 技能檔再生為現版內容，`CLAUDE.md` 的區塊被剝除、使用者段落原樣保留，stdout 摘要列出被剝除的檔案

#### Scenario: 取消 Copilot 只移除 speclink 技能目錄

- **WHEN** `.speclink.yaml` 的 built-in tools 為 `[claude, copilot]`，`.github/skills/` 下另有使用者自己的 `my-skill/SKILL.md`，`.github/workflows/ci.yml` 與 `.github/copilot-instructions.md` 存在，然後同步 built-in 選集 `[claude]`
- **THEN** `.github/skills/` 下所有 `speclink-` 前綴目錄被移除，`my-skill/SKILL.md`、`.github/workflows/ci.yml` 與 `.github/copilot-instructions.md` 位元級不變，Claude Skills 被更新

#### Scenario: symlink 的 Copilot 技能目錄不被穿透清除

- **WHEN** `.github/skills` 是指向 `../.claude/skills` 的 symlink，`.speclink.yaml` 的 built-in tools 為 `[claude]`，執行 speclink update
- **THEN** `.claude/skills/` 下的 Claude Skills 全數保留，`.github/skills` symlink 本身保留，清除摘要不列 copilot

#### Scenario: 同步前就空的 Copilot 技能目錄保留

- **WHEN** `.speclink.yaml` 的 built-in tools 為 `[claude]`，`.github/skills/` 是同步前就存在的空目錄，執行 speclink update
- **THEN** `.github/skills/` 仍存在

#### Scenario: 共用目錄取消一種工具

- **WHEN** tools 原含 codex 與 antigravity，使用者同步為只含 antigravity，再同步為只含 codex
- **THEN** 每次 .agents/skills 的受管技能均完整可用且內容相同，tools 保存實際所選名稱，不因另一種被取消而刪除共用技能

#### Scenario: 共用工具全數取消

- **WHEN** tools 原含 codex 與 antigravity，使用者同步為只含 claude，.agents/skills 另有 my-skill
- **THEN** .agents/skills 的受管 speclink-* 目錄移除，my-skill、AGENTS.md 的非受管內容及 Claude 技能保留


### Requirement: 技能檔過期探測

<!-- BEFORE: 逐工具獨立比較各自目錄，未定義兩種內建工具共用實體目錄。 -->

引擎 SHALL 提供唯讀的技能檔過期探測：依 .speclink.yaml 的 tools 清單——內建工具與通過驗證的自訂描述子——讀取各工具 skills 目錄下技能檔 frontmatter 的版本欄位並與當前產物層版號比對，回報五態之一——缺失（任一工具的 skills 目錄下無任何 speclink- 技能檔，即從未安裝或整組移除）、過期（任一工具的技能版號舊於現版）、較新（任一工具的技能版號新於現版，即工作區檔案領先引擎）、現版、無法判定（設定解析失敗或技能檔存在但讀取錯誤）。逐工具方向資訊的工具名 SHALL 為內建名（claude、codex、copilot、antigravity）或描述子的 name。無法通過驗證的描述子 SHALL NOT 參與探測，也 SHALL NOT 使結果變為無法判定（壞描述子由 update 的錯誤告知）。方向 SHALL 以版號數值比較判定：去除 v 前綴、以點號拆段、逐段數值比較，段數不足補零；任一邊無法完整解析為數字段時，該工具 SHALL 退回字串相等判定（不等即過期），SHALL NOT 對無法解析的版號排序方向、SHALL NOT 據以判較新。聚合優先序 SHALL 為 較新 > 缺失 > 過期 > 現版：任一工具較新即整體回報較新。過期、缺失或較新時 SHALL 一併回報「更新將新建或改寫且內容與現版 render 不同」的受管檔清單（專案根相對路徑，描述子的路徑以其 skills_dir 起頭）與各工具的方向資訊；比對前 SHALL 正規化換行，僅換行形式差異的檔案 SHALL NOT 列入清單。探測 SHALL NOT 寫入任何檔案。

#### Scenario: 舊版工作區判過期並列差異檔

- **WHEN** 工作區技能檔的版號數值舊於當前產物層版號，執行過期探測
- **THEN** 回報過期，並列出內容與現版 render 不同的技能檔相對路徑

#### Scenario: 工作區檔案領先引擎判較新

- **WHEN** 工作區技能檔的版號數值新於當前產物層版號（如新引擎再生後以舊版 app 探測），執行過期探測
- **THEN** 回報較新，並回報差異檔清單；SHALL NOT 與過期混同

##### Example: 引擎 v1.11.0 探測 v1.14.0 工作區

- **GIVEN** 引擎產物層版號 v1.11.0，工作區技能檔 frontmatter 版本 v1.14.0
- **WHEN** 執行過期探測
- **THEN** status 為 "newer"（2026-08-05 事故情境：舊判準回報「過期」並導致按「更新」降級 30 檔）

#### Scenario: 較新優先於缺失與過期

- **WHEN** tools 清單含 claude 與 codex，.claude/skills/ 技能版號新於現版而 .agents/skills/ 下無任何 speclink- 技能檔，執行過期探測
- **THEN** 回報較新（非缺失）——任何會改寫領先檔案的動作都不應被提供

#### Scenario: 無法解析的版號退回相等判定

- **WHEN** 工作區技能檔的版本欄位為無法解析為數字段的字串（如手改壞的 frontmatter），與現版不等，執行過期探測
- **THEN** 該工具判過期（字串不等），SHALL NOT 判較新

#### Scenario: 現版工作區不過期

- **WHEN** 工作區全部技能檔由當前版本的 init 或 update 生成，執行過期探測
- **THEN** 回報現版，差異清單為空

#### Scenario: 技能目錄缺少判缺失

- **WHEN** tools 清單含 claude 與 codex，.claude/skills/ 為現版而 .agents/skills/ 下無任何 speclink- 技能檔（如 clone 後技能未進版控），執行過期探測
- **THEN** 回報缺失，並列出更新將新建或改寫且內容與現版 render 不同的受管檔相對路徑；不與無法判定混同

#### Scenario: 設定損壞回報無法判定

- **WHEN** .speclink.yaml 無法解析，執行過期探測
- **THEN** 回報無法判定；SHALL NOT 與現版或過期混同

#### Scenario: 換行差異不誤報

- **WHEN** 工作區技能檔內容與現版 render 僅換行形式不同（CRLF 對 LF），執行過期探測
- **THEN** 該檔不出現在差異清單

#### Scenario: 描述子技能檔缺失判缺失

- **WHEN** tools 清單含 claude 與一個合法描述子（name 為 cursor、skills_dir 為 .cursor/skills），.claude/skills/ 為現版而 .cursor/skills/ 下無任何 speclink- 技能檔，執行過期探測
- **THEN** 回報缺失；逐工具資訊含 tool 為 cursor 且 missing 為真的一項；差異清單含以 `.cursor/skills/speclink-` 起頭的描述子技能檔路徑（非 Claude 子集，worktree 政策關閉時不含兩顆 worktree 技能），不含任何 .claude/skills/ 路徑

##### Example: 描述子缺失時的回報

- **GIVEN** tools 為 `[claude, {name: cursor, skills_dir: .cursor/skills, …}]`，.claude/skills/ 全為現版，.cursor/skills/ 不存在
- **WHEN** 執行過期探測
- **THEN** status 為 "missing"；tools 含 `{tool: "claude", missing: false, stale: false, newer: false}` 與 `{tool: "cursor", missing: true, workspaceVersion: null}`；differingFiles 每一項皆以 `.cursor/skills/speclink-` 起頭並以 `/SKILL.md` 結尾

#### Scenario: 描述子技能檔過期判過期

- **WHEN** tools 清單含一個合法描述子，其 skills_dir 下技能檔的版號數值舊於現版，執行過期探測
- **THEN** 回報過期；逐工具資訊該描述子 stale 為真；差異清單列出其內容與現版 render 不同的技能檔路徑

#### Scenario: 無效描述子不影響探測

- **WHEN** tools 清單含 claude 與一個無法通過驗證的描述子（缺 skills_dir），.claude/skills/ 為現版，執行過期探測
- **THEN** 回報現版，逐工具資訊只含 claude，差異清單為空

Codex 與 Antigravity 選取同一 .agents/skills 時，過期探測 SHALL 使用與生成相同的共用預期內容；differingFiles 的每個路徑 SHALL 只列一次，逐工具方向仍 SHALL 包含實際選取的兩個邏輯工具。探測 SHALL NOT 為未選取工具要求額外副本。

#### Scenario: 同選兩工具後不反覆過期

- **WHEN** tools 為 codex,antigravity，以現版同步後連續探測兩次
- **THEN** 兩次均為現版且 differingFiles 為空，工具方向分別含 codex 與 antigravity；再次同步不改變技能內容

#### Scenario: 共用技能缺失路徑去重

- **WHEN** tools 為 codex,antigravity，.agents/skills 不存在，執行過期探測
- **THEN** 回報缺失，兩工具均有缺失資訊，但每個 .agents/skills/speclink-*/SKILL.md 相對路徑僅列一次


### Requirement: Antigravity 技能生成與獨立清理

<!-- BEFORE: Antigravity 生成於 .agent/skills，與 Codex 的 .agents/skills 分開，使用固定工具身分與專用引用。 -->

Antigravity SHALL 為內建工具名 antigravity，與 Codex 同樣使用專案根 .agents/skills 下的 speclink-* 技能目錄；AI 工具選擇畫面 SHALL 顯示 antigravity [.agents]。單選 Codex、單選 Antigravity 或同選兩者 SHALL 生成相同的共用技能內容，按「Codex 與 Antigravity 共用技能內容」執行；SHALL NOT 以最後寫入者決定內容。tools SHALL 保存實際選取的工具名稱，不合併或改名邏輯工具。

生成 SHALL 沿既有工具同步計畫，SHALL NOT 生成 AGENTS.md、GEMINI.md 或修改使用者全域設定。更新、過期探測與清理 SHALL 共用實體目錄期望值；取消任一工具時，只要另一種仍選取就 SHALL 保留共用技能。兩者均取消時 SHALL 僅清理受管技能並保留自有技能與 .agents 其他檔案；SHALL NOT 跟隨目錄連結清理。舊 .agent/skills 依「Antigravity 舊目錄升級」處理。舊自訂描述子撞名、撞 .agents/skills 或撞仍保留供升級的 .agent/skills SHALL 在寫入前拒絕並給明確遷移提示，不靜默覆寫。

#### Scenario: 同時使用 Codex 與 Antigravity

- **WHEN** 使用者以 tools codex,antigravity 初始化或更新專案
- **THEN** tools 保存兩種名稱，受管技能只生成一份於 .agents/skills，內容含兩工具呼叫及身分說明、不互相覆寫，不再生成 .agent/skills 的新技能

#### Scenario: 取消 Antigravity 保留使用者檔案

- **WHEN** 已生成兩種工具技能的專案自 tools 移除 antigravity 並更新
- **THEN** Codex 仍選取時 .agents/skills 的共用受管技能保持可用，自有技能、.agent 其他檔案與 .agents 下自有內容保持不變；連結目錄不被跟隨清理

#### Scenario: 舊自訂描述子與新內建工具衝突

- **WHEN** tools 含 name antigravity 的描述子，或自訂 skills_dir 正規化等同 .agent/skills 或 .agents/skills
- **THEN** 更新以非零退出、提示撞名或撞目錄及改名 antigravity-custom／改用內建工具的可行方式，工作區技能不變

#### Scenario: 介面與本機及遠端工具生成位置一致

- **WHEN** 使用者在 AI 工具設定只選 Antigravity 並同步，或在遠端 checkout 初始化 Antigravity 技能
- **THEN** 介面顯示 antigravity [.agents]，技能落於本機 checkout 的 .agents/skills，不為此建立本機規格樹或修改遠端認證



### Requirement: Copilot 渲染目標

<!-- BEFORE: Copilot 與 Codex 內容除工具前綴及身分外逐字相同；Codex 此次改成共用渲染。 -->

內建工具 copilot 的技能檔 SHALL 生成於專案根的 `.github/skills/speclink-<name>/SKILL.md`。生成集合 SHALL 與 codex 相同：registry 中標記為非 Claude 工具也生成的技能（即除 analyze 以外的技能），worktree 政策關閉時排除兩顆 worktree 技能。frontmatter SHALL 依序僅含 name、description、license、compatibility、metadata（author、version、generatedBy），SHALL NOT 含 context、agent、disallowedTools、allowed-tools、argument-hint、user-invocable、disable-model-invocation。內文 SHALL NOT 含 Claude fork 前言（`## Claude fork context`）。技能之間的引用 SHALL 寫成 `/speclink-<name>`，SHALL NOT 出現 `$speclink-`；`{{TOOL}}` 的代換值 SHALL 為 `copilot`，因此技能指示建立變更與蓋章時帶 `--agent copilot`；`{{PLAN_DIR}}` 的代換值 SHALL 為空字串。Copilot 的既有工具專用渲染 SHALL 保留；Codex 與 Antigravity 則依共用技能內容契約生成，不要求與 Copilot 逐字相同。copilot SHALL NOT 產生、改寫或剝除任何指令檔（`.github/copilot-instructions.md`、`AGENTS.md`、`CLAUDE.md`）。工作區 footprint 偵測（`.speclink.yaml` 無 tools 清單時的遺留回退，以及 desktop checkout 的預選）SHALL NOT 因 `.github/` 或 `.github/skills/` 存在而推斷 copilot。

#### Scenario: 選取 Copilot 生成正確的技能檔

- **WHEN** 在空目錄以 `--tools copilot` 執行 speclink init，檢視 `.github/skills/speclink-propose/SKILL.md`
- **THEN** frontmatter 只有 name、description、license、compatibility、metadata 五個頂層鍵，name 為 `speclink-propose`；內文引用其他技能寫成 `/speclink-apply` 這類形式、不含 `$speclink-`、不含 `## Claude fork context`，建立變更的指令帶 `--agent copilot`

##### Example: 三個內建工具的渲染差異

| 工具 | skills 目錄 | 技能引用寫法 | `--agent` 值 | Claude 專用 frontmatter | 含 analyze |
| --- | --- | --- | --- | --- | --- |
| claude | `.claude/skills` | `/speclink-apply` | `claude` | fork 技能有 context、agent；唯讀技能有 disallowedTools | 是 |
| codex | `.agents/skills` | 共用前言說明 `$speclink-apply` | 執行前填 codex | 無 | 否 |
| copilot | `.github/skills` | `/speclink-apply` | `copilot` | 無 | 否 |

#### Scenario: Copilot 與 Codex 生成集合相同

- **WHEN** 在 openspec/config.yaml 無 worktree 鍵的專案以 tools 為 `[codex, copilot]` 執行 speclink update
- **THEN** `.github/skills/` 與 `.agents/skills/` 下的 `speclink-` 目錄名稱集合相同，兩者都不含 `speclink-analyze`、`speclink-apply-with-worktree`、`speclink-worktree-merge`

#### Scenario: 選取 Copilot 不產生指令檔

- **WHEN** 在空目錄以 `--tools copilot` 執行 speclink init
- **THEN** 專案根不存在 `AGENTS.md`、`CLAUDE.md`，`.github/` 下只有 `skills/` 目錄，不存在 `.github/copilot-instructions.md`

#### Scenario: footprint 偵測不推斷 Copilot

- **WHEN** 專案含 `.claude/` 目錄與 `.github/workflows/ci.yml`、`.speclink.yaml` 不含 tools 清單，執行 speclink update
- **THEN** 只生成 `.claude/skills/` 下的技能檔，`.github/skills/` 不被建立，`.github/workflows/ci.yml` 位元級不變

## ADDED Requirements

### Requirement: Codex 與 Antigravity 共用技能內容

共用技能 SHALL 使用非 Claude 的標準 frontmatter，所有技能間引用以 speclink-<技能名> 的工具無關名稱表述。前言 SHALL 說明 Codex 的 $speclink- 與 Antigravity 的 /speclink- 入口、使用當前宿主可用的互動介面，且這些技能入口不是 speclink CLI 子指令。CLI 範例的 --agent SHALL 以 <current-agent> 表示執行前要依當前宿主填入的 codex 或 antigravity；前言 SHALL 要求在執行前替換，不以另一種工具身分冒名、不執行字面佔位值。所有生成模板 SHALL 已替換，不得殘留 {{TOOL}}、{{SPEC_DIR}} 或 {{PLAN_DIR}}。僅工作流允許時生成 worktree 技能，安全技能與其他受管集合維持原有政策；Claude、Copilot 與自訂 neutral 的既有渲染 SHALL 維持原契約。

#### Scenario: 兩工具可執行同一份技能

- **WHEN** 分別以 Codex 與 Antigravity 讀取同一份共用 propose 或 review 技能並執行命令
- **THEN** 前言可分別得到對應宿主技能入口，建立變更或蓋章時 --agent 分別填 codex、antigravity，不把 <current-agent> 或另一工具名寫入紀錄；兩者讀取的檔案內容一致

#### Scenario: 政策關閉時共用集合收斂

- **WHEN** tools 同選 codex,antigravity，把 worktree 政策由開啟改為關閉並同步
- **THEN** .agents/skills 不再包含受管 apply-with-worktree 與 worktree-merge 技能，自有技能和其他應生成的技能保留，再次同步結果相同

### Requirement: Antigravity 舊目錄升級

既有專案執行 update 或桌面工具同步時 SHALL 先驗證設定、描述子、降級守門與目錄邊界，再成功生成選取工具所需的 .agents/skills 全部內容，才清理 .agent/skills 的舊受管 speclink-*；不再選任何共用工具時 SHALL 清理舊／新目錄的受管技能而保留自有內容。不新增 CLI 旗標、stdin payload 或設定欄位，維持既有成功摘要與 exit code 0，失敗非零、stderr 指出失敗路徑；SHALL NOT 自動 commit。SHALL 保留自有技能、規則和 .agent 其他檔案，不直接刪除整個 .agent。清理 SHALL NOT 跟隨目錄或技能子目錄的連結／重解析點，也不得透過連結父目錄存取專案外檔案；不安全的新寫入位置 SHALL 在任何技能寫入前拒絕。

新位置寫入失敗 SHALL 保留舊位置來源；新位置全部完成後清理失敗 SHALL 保留已可用的新技能並回報剩餘失敗位置。重跑 SHALL 收斂到相同有效結果，SHALL NOT 為恢復而覆蓋使用者檔案。舊 .agent/skills 保留為遷移保留路徑，自訂描述子不可佔用；較新版資產繼續受既有降級守門保護。

#### Scenario: 舊版更新保留自有內容

- **WHEN** 專案 tools 含 antigravity，舊 .agent/skills 含受管技能和 my-skill，.agent/rules/team.md 存在，執行 update
- **THEN** exit code 0，新 .agents/skills 含完整共用技能，舊受管技能被清理，my-skill 與 team.md 位元不變，spec_dir 與規格樹不變

#### Scenario: 新位置失敗不刪舊技能

- **WHEN** 新 .agents/skills 寫入被拒或中途失敗
- **THEN** exit code 非零、stderr 含失敗路徑，舊 .agent/skills 內容仍在；修正權限後重跑成功且技能探測為現版

#### Scenario: 清理失敗可續作

- **WHEN** 新技能已全部生成，刪除一個舊受管技能時遭拒
- **THEN** 回報非零及失敗路徑，新技能仍完整可用，自有內容保留；移除障礙後重跑完成清理而不新增第二份共用技能

#### Scenario: 目錄連結不被穿透

- **WHEN** 舊 .agent、其 skills 目錄或受管技能子目錄是導向其他位置的連結或 Windows 重解析點
- **THEN** 清理保留該連結及其目標內容並回報跳過，不跟隨刪除；新 .agents 位置不安全時先拒絕寫入，不改外部檔案
