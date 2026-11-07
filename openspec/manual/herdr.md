---
title: Herdr 工作區中的 Speclink
section: Claude Code
order: 385
keywords: [Herdr, 面板, space, Agent, 技能, 路線圖]
sources: [herdr-integration, "claude-code-session-focus#Herdr 內即時回報已確認的變更"]
generated: 2026-11-07T21:34:37+08:00
---

# Herdr 工作區中的 Speclink

Herdr 0.1.13 以上可使用 Speclink 整合面板。專案提供整合來源與安裝指引；整合不會自動安裝 Herdr 或改寫你的 Herdr、Claude Code 設定。

## 查看目前工作

面板可停靠在右欄，也可在非停靠版面以快捷鍵開啟浮動面板。它跟隨目前終端所在的 space，顯示該專案的變更、討論、波次、阻擋、任務進度、worktree 與品質章。每個 space 各自記住停靠開關；資料讀取失敗時會顯示錯誤。

點變更或討論列可在桌面 app 開啟詳情，點波次或路線圖鈕可開啟路線圖。在已初始化的本機 space 點 Logo，會切到該專案看板。只有確定尚未初始化時才會出現初始化提示；設定錯誤、離線或認證失效會顯示原因，不會自動覆寫設定。

## 選擇並送出技能

每列依目前進度建議下一個技能，展開按鈕可看其他適用選項。程式任務完成但仍有 worktree 時，優先建議合併 worktree；需要重新反映討論時，建議 ingest。開工會在目前 space 開新 Agent；worktree 政策開啟時，先選擇使用 worktree 或直接實作。

列上的附加按鈕只把名稱放進輸入框，不會送出。Agent 正等待你回答時，面板不會把技能指令誤送成回答；沒有合適目標時會依條件開新分頁或說明不可用。封存仍受既有品質與安全驗收守門，面板不會替你跳過。

## Agent 與介面語言

面板可顯示工作者狀態並跳回對應 Agent，區分工作中、閒置、完成與等待回答。文字中只是提及變更名稱，並不代表 Agent 正在處理它。Claude Code 確認變更焦點後會回報給 Herdr；沒有 Herdr 時原有技能列與對話標題照常運作。

介面跟隨系統語言：中文系統使用繁體中文，其餘語言與未設定時使用英文。切換語言不會翻譯檔名或差異內容。

**出處**：`herdr-integration`、`claude-code-session-focus`
