## ADDED Requirements

### Requirement: 品質模型與產出語言

workflow-config SHALL 支援以站與工具指定 subagent_model，省略時沿用原選擇，工單記下當輪模型。instructions specs SHALL 回報有效 spec_locale，tw、ja、en 分別回傳對應語言，auto 跟 locale，未設定維持 fork 既有預設。指向正式規格的名稱沿用原文，無變更寫入不重寫設定。

#### Scenario: 變更語言不改需求名稱
- **WHEN** 專案語言為繁中、spec_locale 為 ja，修改一條正式規格中的繁中需求
- **THEN** specs 的 JSON locale 回報日文，規格本文用日文，既有需求及情境名稱保留原文

#### Scenario: 相同模型再寫一次
- **WHEN** 現有 review 的 Codex 模型已等於指定值，再執行 workflow-config subagent-model
- **THEN** 成功退出，設定的內容和修改時間不變，不重生技能
