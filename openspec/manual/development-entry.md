---
title: 從原始碼啟動開發環境
section: 開始使用
order: 26
keywords: [開發, dev, server, desktop, CLI, 重置]
sources: ["dev-harness#一鍵啟動 remote 開發環境", "dev-harness#env 驅動的 dev 設定與 .env.example 對照", "dev-harness#dev 資料持久化與顯式重置", "dev-harness#checkout 內 CLI 測試入口", "dev-harness#單獨啟動 server", "dev-harness#單獨啟動 desktop", "dev-harness#dev 模式前端由 dev server 供應且變更免重編", "dev-harness#dev 啟動自動佈署當前 checkout 的 sidecar", "user-documentation#開發者入口文件雙語對"]
generated: 2026-11-07T21:34:37+08:00
---

# 從原始碼啟動開發環境

在 Speclink 原始碼專案的根目錄，可依需要選擇同時啟動、單獨啟動或只執行 CLI。這些入口使用目前這份原始碼，而非依賴 PATH 裡已安裝的 Speclink。

| 目的 | 指令 | 結果 |
| --- | --- | --- |
| 同時開發 server 與桌面 app | `npm run dev` | 先建置目前版本的 CLI，再啟動 server 與桌面視窗；首次設定的 `/setup` 連結會出現在終端 |
| 只啟動 server | `npm run dev:server` | 啟動 server，不啟動桌面視窗 |
| 只啟動桌面 app | `npm run dev:desktop` | 啟動桌面開發視窗，不啟動 server |
| 透過本份原始碼執行 CLI | `npm run cli -- status` | 執行這份專案的 CLI；尚未建置時會先建置 |
| 清除開發資料 | `npm run dev:reset` | 清除 `.dev/` 後結束；下次啟動重新進入開箱流程 |

> [!WARNING]
> `npm run dev:reset` 會刪除 `.dev/` 裡的開發資料；`.env` 不會被刪除。若要保留既有資料，先備份再重置。

server 預設在本機 `127.0.0.1:8080` 啟動，資料存於 `.dev/`。可用專案根目錄的 `.env` 設定開發環境；同名的程序環境變數優先。設定有誤，例如選用 PostgreSQL 卻沒有連線網址時，啟動會停止並指出原因。

桌面開發畫面由開發伺服器提供；修改前端後畫面可更新，不需為此重新編譯 Rust。連接埠 1420 被占用時啟動會失敗，請先處理占用情況。按 Ctrl+C 可結束同時啟動的 server 與桌面程序。

**出處**：`dev-harness`、`user-documentation`
