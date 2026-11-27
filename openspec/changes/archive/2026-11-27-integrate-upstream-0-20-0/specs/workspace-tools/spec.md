## ADDED Requirements

### Requirement: 新增工具及遠端技能來源

init 與 update SHALL 支援 Pi 的 .pi/skills 與 Cursor 的 .cursor/skills，桌面工具選擇同源。遠端 update 按 Server 政策及 checkout 取得技能，較舊模板提供版本提示。Codex 與 Antigravity 共用 .agents/skills，也保留現有 fork 受管技能。

#### Scenario: 工具共存
- **WHEN** 同時選取 Codex、Antigravity、Pi、Cursor 並同步
- **THEN** 各目標生成可呼叫的技能，取消其中一個共用目標後另一個仍可用

#### Scenario: 遠端取得失敗
- **WHEN** 遠端 update 時 Server 離線或憑證失效
- **THEN** CLI 非零退出且 stderr 顯示原因，既有技能不被空結果覆寫
