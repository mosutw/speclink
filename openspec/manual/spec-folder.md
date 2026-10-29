---
title: 規格資料夾與舊佈局遷移
section: 開始使用
order: 25
keywords: [規格資料夾, openspec, speclink, spec_dir, OpenSpec, .meta.yaml, speclink update]
sources: [spec-dir-migration, workspace-tools, user-documentation]
generated: 2026-10-29T05:54:00+08:00
---

# 規格資料夾與舊佈局遷移

本 Fork 預設在 `.speclink.yaml` 明寫 `spec_dir: openspec`，讓規格留在 `openspec/`。這是與原作者 0.11.0 的刻意差異；其他新功能及 `.meta.yaml` 格式沿用上游。

## 新專案與既有專案

`speclink init --tools claude,codex` 建立 `openspec/` 與明確設定。已有 OpenSpec 資料夾、沒有工作區設定時，init 原地補齊缺件，保留既有 config.yaml、自訂產出流程、規格與變更。不要求先提交未完成的修改，也不因其他 worktree 而搬移資料。

舊 Speclink 專案執行 `speclink update` 時，先驗證設定、工具與降級條件，再判定實際資料夾。如果只有 `openspec/` 且沒有 `spec_dir`，先補寫此鍵，再更新技能。工具、註解與未知鍵保留，重試不重複加鍵；設定壞掉或補寫失敗，則在遷移與技能生成前停止。

例如 `.speclink.yaml` 原本只有 `tools: [claude, antigravity]`，規格放在 `openspec/`。更新後工具清單不變，多一行 `spec_dir: openspec`，原有規格與討論不搬動。

## 已有其他目錄時

明確 `spec_dir` 最優先。未明寫時，解析器保留上游的既有專案辨識順序：先找具有規格結構的 `speclink/`，再找 `openspec/`，最後回退引擎內建位置。全新初始化則由本 Fork 的入口明確選 `openspec`。只有名字叫 speclink、裡面沒有規格結構的目錄，不會被當成規格資料夾。

已使用 `speclink/` 或自訂目錄的專案維持原路徑。兩種目錄並存又未明寫設定時，CLI 會提醒它實際選的是哪一個；先確認內容，再自行明寫選擇，不會合併兩個目錄的內容。

已使用 `speclink/`、未明寫 `spec_dir`，裡面仍有 `.openspec.yaml` 時，update 保留上游的資料檔改名流程；有其他 worktree 或未提交修改時暫緩。明寫 `spec_dir` 的專案不批次改名，仍可逐筆讀寫。

## 新舊變更資料檔

讀取先找 `.meta.yaml`，缺席才回退 `.openspec.yaml`。新檔存在但無法讀取或解析會回報錯誤，不會把舊檔當成成功結果。新寫入使用 `.meta.yaml` 並清掉舊檔；品質蓋章的 metadata、歷史索引與刪除工單共用交易，競爭或中途失敗不會留下部分結果。

只讀取不會改寫歷史。想回到舊版本前，先備份完整資料夾及 checks 歷史，不要直接用舊版覆蓋新資料。

## 技能與桌面 app

技能依實際資料夾產生：預設 `openspec/`、既有 `speclink/` 或自訂位置都可。worktree 政策切換只同步技能，不搬資料夾。桌面 app 的補齊及更新使用同一套核心；設定頁與監看目標跟隨實際路徑。

## 從 OpenSpec 轉成 Speclink

本機專案可在桌面 app 的專案設定 `.speclink.yaml` 頁選擇「規格資料夾」。預設為 OpenSpec（`openspec/`）；選 Speclink（`speclink/`）會先顯示不可逆提醒，只有按「轉換成 Speclink 模式」才會搬動資料。按取消、Escape 或關閉確認框都不會寫入。完成後，設定頁停用 OpenSpec 選項，產品沒有切回入口。

轉換會保留原規格、變更、討論、手冊、品質紀錄與 metadata 內容，不會把兩種資料夾格式化成不同內容，也不會建立雙份同步。若資料夾已搬好但技能同步未完成，專案會標示未完成；重新開啟後可按「重試完成轉換」。一般 `speclink update` 不會因這項選擇而搬資料夾。自訂路徑與 remote 專案不使用此入口。

## 與其他工具互通

Markdown 與 YAML 仍可直接讀寫；保留 OpenSpec 資料夾方便遷移內容。這不代表已驗證 OpenSpec CLI 或其他衍生工具的完整互通，尤其生命週期資料現在使用 `.meta.yaml`。

相關操作：[建立工作區](init-workspace.md)、[桌面專案與設定](desktop-projects.md)、[更新技能](desktop-update.md)、[遷移到 remote](migrate.md)。

**出處**：`spec-dir-migration`、`workspace-tools`、`user-documentation`
