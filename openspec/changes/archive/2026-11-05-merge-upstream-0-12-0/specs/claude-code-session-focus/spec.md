## MODIFIED Requirements

### Requirement: 從使用者指令與工具呼叫辨識焦點

<!-- BEFORE: 沒帶名稱的所有技能均保留先前變更，Speclink Bash 查詢中提及名稱就改變焦點。 -->

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

## ADDED Requirements

### Requirement: Herdr 內即時回報已確認的變更

Claude Code 外掛 SHALL 在確認當前變更時即時更新 Herdr 的 speclink.focus 標籤，供面板與 Agents 清單使用；沒有變更或已切到非變更技能時 SHALL 不保留錯誤的上一個工作歸屬。此整合 SHALL 僅在有 Herdr 環境時執行，沒有 Herdr 時 SHALL 保持既有技能列、標題與操作流程，不為回報自動安裝 Herdr 或寫入使用者設定。

#### Scenario: 名稱確認即回報

- **WHEN** 在 Herdr 內的 Claude Code session 從未指定名稱的 apply 經主代理 CLI 操作確認 add-auth
- **THEN** 面板可在下一次資料更新時由 speclink.focus 辨識該 Agent 正處理 add-auth，無須等待整個技能完成

#### Scenario: 沒有 Herdr 保持原功能

- **WHEN** 在一般 Claude Code 環境送出 apply add-auth
- **THEN** 技能列與對話標題仍依既有規則更新，不因缺少 Herdr 回報介面而失敗，不建立外部設定檔
