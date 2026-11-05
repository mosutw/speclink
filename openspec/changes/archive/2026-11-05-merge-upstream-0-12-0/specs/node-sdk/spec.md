## MODIFIED Requirements

### Requirement: 渲染 API

<!-- BEFORE: Codex 與 Antigravity 渲染成兩份不同的工具專屬內容並各自對齊 CLI 生成。 -->
SDK SHALL 提供 skills.list()（回傳技能名與描述清單）與 skills.render(name, options)（回傳技能 Markdown 字串，不寫入檔案）。options SHALL 包含 target（claude｜codex｜copilot｜antigravity｜neutral），並接受 invocation（cli｜tool-call，預設 cli）、specDir（預設 openspec）與 toolName（neutral 專用，預設 speclink）。回傳字串內容 SHALL 與 CLI 以對等參數生成的內容一致。SDK 的公開 TypeScript 宣告 SHALL 接受上述五種 target 與選項，並拒絕未知 target、未知 invocation 及 store／worktree 渲染選項；SDK SHALL NOT 提供 instructions.render 指令檔介面。執行時未知 target SHALL 拋出錯誤並列出五個合法值，未知 invocation 與未知技能 SHALL 明確拋出錯誤。此為規格對齊既有技能 API，加上 Antigravity 型別修正；CLI 人眼輸出、JSON 與退出碼 SHALL 維持既有行為；Codex／Antigravity 生成內容依 workspace-tools 的共用內容規定刻意更新，其他 target 維持原行為。

#### Scenario: 中性 tool-call 渲染
- **WHEN** 執行 skills.render('propose', { target: 'neutral', invocation: 'tool-call' })
- **THEN** 回傳字串以「呼叫 speclink 工具」措辭表述動詞並使用 argv、不含 /speclink- 與 $speclink- 前綴，且沒有未替換模板；不承諾移除技能內的所有本機路徑

#### Scenario: 與 CLI 生成一致
- **WHEN** 以 target claude 呼叫 skills.render('apply', …)，並與相同 specDir 的 speclink init 生成的 .claude/skills/speclink-apply/SKILL.md 比對
- **THEN** 兩者內容一致

#### Scenario: copilot target 與 CLI 生成一致
- **WHEN** 以 target copilot 呼叫 skills.render('apply', …)，並與相同 specDir 的 speclink init --tools copilot 生成的 .github/skills/speclink-apply/SKILL.md 比對
- **THEN** 兩者內容一致，且回傳字串的技能引用為 `/speclink-` 形式、不含 `$speclink-`

#### Scenario: 未知 target 被拒
- **WHEN** 執行 skills.render('apply', { target: 'github-copilot' })
- **THEN** 呼叫拋出錯誤，訊息含 `github-copilot` 與 claude、codex、copilot、antigravity、neutral 五個合法值

#### Scenario: Copilot 原生渲染與 CLI 同源

- **WHEN** 以 target copilot 呼叫 skills.render，並以相同 specDir 生成 CLI Copilot 技能
- **THEN** SDK 回傳字串與 .github/skills 對應技能一致；Copilot 採 /speclink- 語法，Codex 的共用內容以宿主前言說明 $speclink-，不要求 Copilot 與共用內容逐位元相同；不寫入檔案。未知 target 或技能明確拋錯，neutral 的 invocation 語意不變。

#### Scenario: Antigravity 原生渲染與 CLI 同源

- **WHEN** SDK 以 target antigravity 渲染各對外技能，並以 CLI init --tools codex,antigravity 產生技能
- **THEN** SDK 與 .agents/skills 對應共用檔內容一致；前言說明 Antigravity 的 /speclink- 和 Codex 的 $speclink-，兩工具同選時內容不互相覆寫

#### Scenario: 公開 TypeScript 宣告可供使用者編譯

- **WHEN** TypeScript 使用者從套件入口匯入 skills 與 RenderOptions，以五種合法 target 分別呼叫，並傳入 invocation、specDir、toolName
- **THEN** strict、noEmit 編譯成功；未知 target／invocation、store／worktree 選項及 instructions 匯入均為編譯錯誤

#### Scenario: 選填參數與預設值

- **WHEN** 比較只給 target neutral 與明給 invocation cli、specDir openspec、toolName speclink 的渲染，並另以 specDir specs-local、toolName project-tool、invocation tool-call 渲染 verify
- **THEN** 前兩者內容一致；自訂渲染使用 specs-local/changes/ 與 --agent project-tool，不殘留未替換模板；tool-call 仍以呼叫 speclink 工具表述動詞，toolName 只替換技能內的工具身分

#### Scenario: 未知 invocation 被拒

- **WHEN** JavaScript 呼叫 skills.render('apply', { target: 'neutral', invocation: 'shell' })
- **THEN** 呼叫拋出錯誤，訊息含 shell、cli 與 tool-call

Codex 與 Antigravity target SHALL 回傳相同共用技能內容，不新增公開 options；兩者 SHALL 與相同 specDir 的 CLI 單選／同選生成逐位元一致。neutral 的 invocation 與 toolName 語意 SHALL 不受此共用改動影響。

#### Scenario: Codex 與 Antigravity 渲染對等

- **WHEN** 對每個兩工具可用的技能以 target codex、antigravity 呼叫 skills.render，並比對 CLI 三種選集的 .agents/skills 產物
- **THEN** 五份對應技能內容一致，包含當前宿主身分與呼叫方式說明、不殘留生成模板；API 不讀寫工作區或 .agent
