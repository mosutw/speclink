## ADDED Requirements

### Requirement: macOS 本機安裝的簽章身分

macOS 的本機安裝腳本 SHALL 優先用 APPLE_SIGNING_IDENTITY，未設定時找可用的 Developer ID Application。用 Developer ID 時 SHALL 驗證 app 內的 CLI 具有 Team ID 且使用正式簽章才安裝。找不到憑證時沿用 ad-hoc 並告知金鑰圈可能會重新詢問。正式發行沿用既有 fork 流程。

#### Scenario: 明示身分且簽章失敗
- **WHEN** 指定 APPLE_SIGNING_IDENTITY，但產出的 bundle 內 CLI 的簽章沒有 Team ID
- **THEN** 本機安裝腳本顯示簽章錯誤、以非零結束、已安裝的 app 不被替換

#### Scenario: 未找到 Developer ID
- **WHEN** 未指定身分，且本機沒有 Developer ID Application
- **THEN** 腳本印出 ad-hoc 的說明並可完成安裝
