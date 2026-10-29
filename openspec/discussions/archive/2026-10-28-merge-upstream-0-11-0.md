---
topic: 原作者已釋出0.11.0, 以我目前的版本為主，將原作者的功能合併到我的版本
slug: merge-upstream-0-11-0
status: promoted
created: 2026-10-28
created_by: mosutw <luke1974@gmail.com>
promoted_to: merge-upstream-0-11-0
---

# Discussion: 原作者已釋出0.11.0, 以我目前的版本為主，將原作者的功能合併到我的版本

<!--
Document rules:
- Rounds are appended by `speclink discuss add-round`; never rewrite an earlier round.
  A changed position gets a new round that names what changed and why.
- Each round distills one focus question: **Focus** / **Position** / **Ruled out** / **Open**.
- The conclusion must resolve or explicitly defer every open question left by the rounds.
-->

## Context

使用者要以 Fork 目前版本為主整合原作者 0.11.0。目標與版本範圍明確，不需要先追問目標。比較基準為 Fork e272d4be、原作者 v0.11.0 對應 64623a6bfbe866f4ae33e37b918cb39b941f38cb，共同祖先為 05e5e65ac043d04e6e0b2a396c0bb0997e52ae70；本地參考位置 upstream/release-v0.11.0。原作者此版本包含規格資料夾與變更資料檔遷移、no-spec-change、change schema、流程成本摘要及品質關卡提醒。使用者關心保留 OpenSpec 與其生態工具的操作與可攜性，最後明確收斂為只調整 .speclink.yaml 的預設 spec_dir，以縮小與原作者的差異。
相關正式規格為 desktop-release、release-notes、security-review、workspace-tools、workflow-schemas、store-abstraction；目前無進行中的變更。既有 store-abstraction 承諾 openspec/ 預設佈局；workspace-tools 與 workflow-schemas 涵蓋設定生成及產出流程。先前討論曾裁定維持檔案可直接操作、啟用既有專案不改既有 openspec/ 內容，以及產出流程驗證對齊 OpenSpec；這些是歷史設計依據，不能當成目前跨工具完整相容已驗證。
Prior discussions: schema-engine-openspec-parity, desktop-workspace-auto-init, manual-spec-edit-integrity

## Rounds

<!-- `### Round N — <mode> (<date>)` entries are appended here by the CLI. -->

### Round 1 — assumptions (2026-10-28)

**Focus**: 整合原作者 0.11.0 時是否應直接採用預設資料夾遷移。
**Position**: 使用者要求保留 OpenSpec 的操作與轉用其他工具的可能性，修正了直接搬到 speclink/ 的初始假設。
- 初始建議為保留 Fork 客製功能並整合上游新增功能；使用者針對 openspec/ 改名提出疑慮。
- 原作者 v0.11.0 README.zh-TW.md 明述匯入後 OpenSpec CLI 無法直接讀取；這是單向匯入，不能將 Speclink 讀取舊佈局解釋為兩套工具能繼續共用。
- workspace/migrate.rs 在明確設定 spec_dir 時不執行自動遷移；speclink-fs 的 write_meta 仍會使用 .meta.yaml，路徑保留與完整格式相容是兩個不同問題。
- 過往 manual-spec-edit-integrity 的否決理由為鎖檔或防手改會違反檔案即真相與 OpenSpec/Spectra 相容的定位；desktop-workspace-auto-init 保留既有內容，但把實質格式轉換留待後續；schema-engine-openspec-parity 對齊產出流程驗證，沒有保證所有命令互通。
**Ruled out**: 未說明相容影響就預設搬走 openspec/——違反使用者保留工具選擇的要求。
**Open**: 設定預設值如何落地；是否擴大為完整格式相容改造；上游其他功能與 Fork 客製功能如何整合與驗收。

### Round 2 — interview (2026-10-28)

**Focus**: 保留規格資料夾應採取什麼具體設定。
**Position**: 使用者指定 .speclink.yaml 預設明寫 spec_dir: openspec。
- 新專案預設產生有效的 spec_dir: openspec 設定，不能只留註解或靠省略欄位的隱含值。
- 本 repo 的 .speclink.yaml 目前只有 tools 清單，整合時補入此明確值；既有專案明確指定的自訂路徑沿用原值。
- 初始化生成落點已存在於 crates/engine/speclink-core/src/workspace/init.rs 的 app_config_text，沿用現有設定與路徑解析，不新增儲存抽象或同步機制。
**Ruled out**: 只在文件建議手動設定——不符合使用者要求的預設行為。
**Open**: 是否擴大為完整格式相容改造；上游其他功能與 Fork 客製功能如何整合與驗收。

### Round 3 — interview (2026-10-28)

**Focus**: 為了 OpenSpec 操作性要把此次整合擴大到什麼程度。
**Position**: 使用者將差異收斂為預設 spec_dir: openspec，以免與原作者 0.11.0 差異過大。
- 此裁定取代上一輪延伸出的完整相容改造建議；不因相容性疑慮額外重寫 .meta.yaml、品質紀錄或封存格式。
- 上游其他新增功能盡量沿用原作，與 Fork 現有行為交會時才做必要整合；安全審查、品質歷史、Antigravity、系統／Server 語言預設、提案條列與 Fork 發佈來源等維持原有功能。
- 預設值解決資料夾被搬走的問題；OpenSpec CLI 全流程與其他生態工具的互通尚未實測，不據此宣稱完整相容。
- 原作者的 no-spec-change、change schema、流程成本摘要、品質關卡主分支與 CI 提醒仍在整合範圍；資料夾遷移相關能力依明確 spec_dir 尊重使用者路徑。
**Ruled out**: 本次新增一整套 OpenSpec 相容層、雙份目錄同步或另寫資料檔格式——超過使用者指定的預設值差異，增加後續追上游的負擔。
**Open**: none

## Conclusion

**Decision**: 以目前 Fork 版本整合原作者 v0.11.0，專案路徑的差異收斂為 .speclink.yaml 預設明寫 spec_dir: openspec。
- 原作者版本鎖定 64623a6bfbe866f4ae33e37b918cb39b941f38cb，不混入 v0.11.0 之後的 main；Fork 起點為 e272d4be，以共同祖先 05e5e65ac043d04e6e0b2a396c0bb0997e52ae70 逐檔確認雙方改動。
- 新的本機專案初始化時預設生成 spec_dir: openspec；本 repo 亦補入此設定。既有 openspec/ 專案更新時須在會觸發上游遷移前補妥此預設，已有明確自訂 spec_dir 者保留原值，不將自訂路徑覆寫為 openspec。
- 具體結果：預設初始化後 .speclink.yaml 含 spec_dir: openspec，規格位於 openspec/specs/、變更位於 openspec/changes/；後續 speclink update 不因上游預設遷移而將此目錄改為 speclink/。明確指定 docs/specs 的專案則持續使用 docs/specs。
- .meta.yaml 與其他上游格式改動原則沿用原作；不擴大為另造 OpenSpec 相容層、雙份目錄同步或資料檔格式改寫。保留資料夾路徑不能當作 OpenSpec CLI 與其他工具完整互通已驗證。
- 整合上游 no-spec-change、change schema、品質輪數與秒級時戳、已封存頁流程成本摘要，以及 quality 的主分支重疊檢查與 CI 提醒；路徑顯示、技能及相關遷移能力尊重明確 spec_dir。
- 保留 Fork 現有安全審查與安全標準資料流程、品質紀錄保存與查閱、Antigravity、系統／Server 語言預設、提案條列、桌面客製功能、Windows 免安裝版及 Fork 更新來源與簽章／發佈設定；遇交會處以保留使用者版本行為為主，納入上游新功能。
- 使用既有設定生成、路徑解析及儲存邊界完成整合，不新增抽象層；技能、回歸快照與更新日誌由整合後的來源重產。驗證包含預設初始化、既有專案更新前補值、自訂路徑保留與 update 不搬目錄，以及受影響的 CLI、桌面、Server、SDK、安全與品質歷史流程。
**Rationale**: 使用者希望保留 openspec/ 的操作與可攜性，同時只維持最少量的 Fork 差異。上游已提供明確 spec_dir 跳過自動遷移的接點，設定預設值即可沿用此機制；完整跨工具格式互通不能僅靠這項設定保證，亦不擴大此次整合範圍。
**Rejected alternatives**:
- 完全採用上游預設並把既有 openspec/ 搬成 speclink/ — 不符合使用者保留 openspec/ 的明確要求。
- 只靠未設定時的隱含路徑或文件提醒使用者手動設定 — 不能確保生成預設值及更新前避開自動搬移。
- 全面新增 OpenSpec 相容層、雙份目錄同步或強制保留舊資料檔格式 — 使用者明確要求只做預設值差異，以減少和原作者的分歧。
**Deferred**:
- OpenSpec CLI 全流程與其他 OpenSpec 生態工具的完整互通保證 — 本次只確定預設路徑，須另以指定工具版本與實測界定相容範圍。
- 整合版本號、實際發佈時間及公開發佈 — 留待整合完成後的發版決策。
**Capture to**: proposal | design | spec | tasks
**Next**: $speclink-propose --from-discussion merge-upstream-0-11-0
