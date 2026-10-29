## Context

需求來自已結論的 merge-upstream-0-11-0 討論。Fork 基準 e272d4be、上游正式版 64623a6bfbe866f4ae33e37b918cb39b941f38cb、共同祖先 05e5e65ac043d04e6e0b2a396c0bb0997e52ae70。這是一個 Feature 型的版本整合：新行為包含 no-spec-change、change schema、流程成本與品質提醒；Fork 特有差異是本機設定預設明寫 spec_dir: openspec。

整合開始前 .speclink.yaml 只有 tools；當時初始化的 app_config_text 省略預設路徑。上游改用 speclink/ 作為內部 fallback，update 在 spec_dir 存在時跳過搬移。只在範本加一行仍不夠：CLI 在建立骨架前決定目錄，adopt 可先觸發匯入，桌面到遠端的遷移又會把明確 spec_dir 當自訂路徑。因此須在既有入口補上同一個明確預設，避免先搬資料才寫設定。

另一個整合交會處是 Fork 的品質歷史：station 的 write_stamp 使用 ChecksCommit 同時保存完整工單、索引、metadata 與刪除活動工單；上游在此新增輪數與精確時間。不能用上游較簡單的直接寫入替換這條既有交易。安全政策目前停用，只保留既有功能與驗收守門，不啟動安全標準審查。

2026-10-28 ingest 補充來自使用者檢視桌面專案設定後的回饋：保留預設 OpenSpec 模式，增加選項讓使用者自行單向轉成 Speclink 模式；選取時先提示無法轉回，確認前不寫入。這項明確操作擴充原先「只調預設」的範圍；原始討論對預設、格式與 Fork 保留的決策繼續有效。先前已完成的 24 個任務保留原文與勾選，新增工作由第 10 組承接。

## Goals / Non-Goals

**Goals:**

- 預設初始化、啟用、更新後設定明寫 spec_dir: openspec，資料仍在 openspec/，所有入口顯示相同實際路徑。
- 桌面專案設定提供 OpenSpec／Speclink 兩種資料夾模式，OpenSpec 為預設；Speclink 選取先提示不可逆，只有明確確認才執行單向轉換，成功後依新路徑繼續操作。
- 取得鎖定的上游 0.11.0 新功能，metadata 新寫入使用 .meta.yaml，舊資料可讀。
- 保留 Fork 安全驗收、品質歷史、工具與語言、桌面功能及發佈來源；對每個上游與 Fork 交會檔案留下可審查的整合結果。

**Non-Goals:**

- 不增加完整 OpenSpec 相容層、雙份目錄同步或 OpenSpec 專用資料格式；兩種模式只代表規格資料夾路徑，不承諾全部 OpenSpec 生態工具互通。
- 不在初始化、啟用、一般 update 或本次 ingest 自動搬動本 repo 的 openspec/；日後只有使用者在設定中明確確認 D8 的單向轉換才搬移。不批次改寫已封存討論／變更內文與歷史追蹤資料，也不提供反向轉換。
- 不合入上游 0.11.0 之後的提交，不選定新發佈版本、不 push 或發佈、不更換更新簽章金鑰。
- 不新增遠端自訂 schema、流程成本 server 清單欄位或品質關卡的自動 CI 守門。

## Decisions

### D1 固定版本與逐檔整合

以共同祖先分別比較兩端，建立 docs/development/upstream-0.11.0-inventory.md：列出上游實作改動、Fork 專有檔案、雙方交集、規格改動及保留／調整／不採用的理由。上游 self-spec-dir-migration 是原作者自己搬資料的提交，不照搬至本 repo；其產品功能增量仍從對應 runtime 與測試採入。產出 docs、技能與快照的來源先整合，再重產衍生物。

本整合跨 core、Host、adapters、desktop/UI 與技能，但它們共同依賴同一個 metadata 名稱、schema 型別及固定的版本差異；切成可獨立 apply 的變更會產生兩套過渡契約。採一個變更、分階段的小任務，盤點先行，各區測試分別驗證，最後才檢查整體保留清單。

### D2 明確預設與路徑入口

沿用 core workspace 的 resolve_spec_dir、init/adopt/update 與 app_config_text，不另造設定服務或第二套路徑解析。CLI 未指定 --dir 的初始化與桌面初始化在建立骨架前傳入 openspec，設定序列化必寫有效 spec_dir。使用者明確 --dir 或既有 spec_dir 優先。adopt 對既有 openspec/ 就地補齊，不能先 import_openspec 再補預設；已採 speclink/ 的專案沿解析結果繼續使用。

update 在完成設定解析、工具描述子與技能降版守門之後，針對「缺少 spec_dir 且實際使用 openspec/」的本機專案補值；補值成功後重新載入 AppConfig，再呼叫上游 migrate::plan。補值使用既有保留內容的 YAML 編輯方式與原子單檔寫入；純量走 util::yaml_scalar，寫前解析重驗，不覆蓋 tools、remote 或未知鍵。若補值成功而後續技能寫入失敗，設定保留有效 openspec 值、規格不搬移，回報失敗並可重試；不把成功補值回滾成會觸發搬移的狀態。

新設定行不代表自訂目錄：本機到遠端遷移的守門接受 openspec 與 speclink 兩種已知佈局，只拒絕其他自訂路徑及兩棵規格目錄並存。上傳成功前本機不變；成功後備份實際目錄再寫 remote marker。設定中的本機路徑不得反過來改變遠端唯讀投影或 store 的文件鍵。

### D3 新舊資料檔與歷史交易

沿用上游 META_FILE、LEGACY_META_FILE、Store 的 metadata 存取及實際檔名顯示；FsStore 先讀 .meta.yaml，缺席才讀 .openspec.yaml，其他讀取錯誤不能被解釋成缺席。新寫入只用 .meta.yaml、完成後淘汰同變更舊檔，兩者並存時新檔是唯一權威。預設明確 spec_dir 跳過 update 的整批改名，個別 metadata 寫入才轉換。

Fork 的 ChecksCommit、FsStore journal 與 Host BridgeStore 仍是品質寫入唯一邊界。修改 history 路徑白名單及檔名到 DocumentId::ChangeMeta 的映射，不能把 .meta.yaml 誤存成普通 artifact。舊檔轉換的 CAS／expected 同時守住讀取來源與新檔缺席條件，刪舊檔與刪活動工單列在同一筆交易；不要另開一條 write_meta 而繞過歷史恢復。原檔、索引、活動工單、輪數及章全部成功才回成功；本地失敗由 journal 恢復，尚未恢復則明示 history_recovery_required，遠端由 UoW 不提交。

遠端 active metadata 沿既有 typed DocumentId 存取；只有已封存 metadata 的相對鍵新增 .meta.yaml 寫入與 .openspec.yaml 讀取 fallback。既有 store 不整批搬鍵。context projection、worktree、排序、討論鏈、trace 與 Claude 外掛對 metadata 的引用一起適配，既有操作語意不變。

### D4 產出流程與跨入口命令

內建定義透過既有 schema 登記表增加 no-spec-change，其 artifact 只有 proposal／design／tasks，applyRequires 為 tasks。正常流程沒有 delta 不警告、不需 skip-specs；spec-driven 的既有約束保留。new artifact spec 在寫入之前依 Schema::has_specs 拒絕不合法輸入，即使帶 stdin 或 force 也不能繞過。

change_schema::set_schema 是唯一 schema 切換邏輯：驗證作用中變更、有效 metadata、kebab-case schema 名稱、可解析的目標、delta 與目標 specs 能力、未結驗證工單，依上游順序處理。相同值不寫入／不發事件；真正改變只改 schema 並清掉五個驗證章欄位，保留 verified_rounds、審查章與已完成歷史。

本機由 Host 注入 project→user→built-in 解析，remote 只允許內建，不讀 client 或 server 的 user 目錄。Protocol 的 Command、Outcome、RemoteClient、CLI、Server 與 Node SDK 共用 Host 裁決，Server reader 403、找不到 404、拒絕 409、revision 衝突沿既有 registry；不新增相同用途的 adapter。workflow-config show 增加 schema 行／欄位，但保留 Fork 的 securityVerification 物件、審查規則與未設定時的系統／Server 語言語意。

### D5 輪數時戳與歷史保留

ChangeMeta 讀取新 RFC 3339 值與既有純日期，寫入使用同一次呼叫取得的帶時區時戳；對外清單序列化仍投影為日期。原始 metadata 文字保留完整時戳，不能先經日期用模型反寫而截斷小時。reviewed_rounds 與 verified_rounds 逐次累加工單輪數、缺值為 0，放棄與守門失敗不累加；非法非負整數拒絕寫入。

write_stamp 在既有 ChecksCommit 裡加入新輪數與精確時間，不改動歷史檔名、rules 快照、索引、接受理由及安全驗收語意。清驗證章不清 verified_rounds。archive 全守門通過後、在移動之前寫 archived_at，與封存目錄日期及 trace.updated 使用同一時間；不為缺 metadata 的舊變更新建資料檔。沿上游接受「移動失敗留在活動目錄且帶 archived_at、重試取代為新時間」的狀態，但 Fork 已保存的歷史及安全驗收摘要仍須依既有交易保存，不得因增加時間而覆蓋。

### D6 桌面摘要與共用呈現

core lifecycle::cost 只透過 Store 讀封存文件計算單筆成本，desktop-core 的快取／查詢一次帶出 docLines、leadDays、leadHours、reviewRounds、verifyRounds；無資料就缺席，不用 0 或 null 補值。server archived list 不新增這些欄位，remote 舊回應仍可解析。

中位數、P90 與樣本數沿用上游 packages/ui 的 processCost；ProcessCostSummary 只在共用 ArchivedList 接一次。沿用 @speclink/ui 的既有頁標題、列表卡與數字磚呈現原語，顏色／圓角／陰影用共用變體，不由 apps 的 className 覆蓋。摘要使用完整封存清單、排除討論，不跟搜尋縮小樣本；缺欄位時隱藏。Desktop 設定與提示使用已解析的實際路徑，watcher 更新後重新跟隨該路徑。Tauri command 保持單行委派，邏輯與測試放 desktop-core。

### D7 技能衍生物與交付保留

先合併 assets/skills 與兩個 schema 定義，再由 renderer 產生受管技能和 golden；禁止逐工具複製上游生成檔而遺失 Antigravity、安全審查指引或 Fork 語言預設。propose／ingest／verify 採入上游 schema 分流；quality 採入首輪前主分支交集檢查與 CI 附註，保留既有「先報告再等使用者裁示」流程。

主分支比較使用 git merge-base 與各自 diff 的檔案集合，路徑用 NUL 分隔避免空白、Unicode 與 Windows 分隔符誤判；找不到主分支按上游直接繼續。此提醒不自動 rebase、merge、commit 或 push。README／docs 的實際預設維持 openspec，說明純文字可攜性及 .meta.yaml，不能沿用「一般更新預設會搬走 openspec」的上游文案；已封存記錄與 trace 區段不批次改寫。

保留 Fork 的更新來源 mosutw/speclink、現行公鑰、GHCR 所有者路徑、Windows portable 與三平台交付守門，保留桌面主題／專案順序／Proxy 診斷和安全資料流程。Cargo 與桌面發佈版本先維持目前 Fork 的一致值；本次完成整合但不導入原作者 bump 提交或提前建立對應 0.11.0 日誌條目。上游新功能及 Fork 日誌歷史先保留在內容盤點，正式新版本條目於後續發版時策展；現有 JSON 與兩份 CHANGELOG 必須維持渲染一致。

### D8 規格資料夾模式與單向轉換

**設定與畫面。** 重用 SettingsSnapshot.specDir 與 resolve_spec_dir；不新增 spec_mode 或第二套路徑來源。apps/desktop/src/views/project-settings/SpeclinkPage.tsx 在既有「設定檔」卡的檔案列下加入「規格資料夾」列，選項為 OpenSpec（openspec/，預設）與 Speclink（speclink/）。新專案仍明寫 spec_dir: openspec，既有 speclink 專案直接顯示 Speclink，不做隱含回退。其他自訂目錄顯示實際路徑與不可選取的自訂現值，不強迫歸類。remote 沿既有規則不出現此頁或本機轉換能力。

沿用 @speclink/ui 的 SettingsRow、Select、ConfirmDialog、Button，不自建選單／對話框外觀。從 OpenSpec 選 Speclink 時只打開確認框，已生效的現值維持 OpenSpec；繁中訊息為「這會將規格資料夾從 openspec/ 轉換為 speclink/。轉換後無法轉回 OpenSpec 模式。」；英文為 “This moves the spec folder from openspec/ to speclink/. After conversion, you cannot switch back to OpenSpec mode.”。按鈕為「取消」「轉換成 Speclink 模式」；按取消、Escape 或關閉確認框均不送出轉換要求、不改設定與工具勾選。確認後進行期間停用重複操作；成功後 OpenSpec 選項停用，顯示已轉換與無法轉回的說明。這是產品不支援反向操作的契約，不宣稱作業系統無法手動搬動檔案。

**唯一操作入口。** 明確轉換與自動 update 分開；不得用刪除 spec_dir 再呼叫 update 的方式觸發搬移。core workspace/migrate.rs 增加專用的單向轉換流程，重用既有 worktree／dirty／symlink／目的地檢查與 Windows rename 重試；流程僅接受已啟用的本機 openspec 來源與固定 speclink 目的地，沒有任意路徑或反向 target 參數。apps/desktop/core/src/settings.rs 提供 convert_spec_dir_to_speclink_at(root)，Tauri command convert_spec_dir_to_speclink 接受 root，委派至桌面核心，成功回既有 camelCase SettingsSnapshot，specDir 為字串 speclink。WorkspaceSettingsProvider 加可選的 convertSpecDirToSpeclink()，只有本機 provider 提供；遠端不發請求。語意錯誤沿既有單行訊息回傳，不新增 HTTP、CLI 或 Node SDK 命令。

**轉換順序與失敗狀態。**

1. 先解析最新設定及實際路徑，驗工具描述子與技能降級守門；拒絕 remote、自訂來源、兩目錄並存、目的地已存在（含檔案／symlink）、來源 symlink、linked worktree、其他 worktree 及 openspec 內未提交／未追蹤修改。拒絕在任何轉換寫入之前完成。若工具選集在畫面上尚未儲存，先要求使用者儲存或放棄該筆工具編輯，不能偷偷連同轉換一起儲存。
2. 在被 .gitignore 排除的 .speclink/spec-dir-conversion.json 保留原設定文字及最小轉換進度，使用既有原子單檔寫入；不另建通用交易框架。原設定與目錄狀態在搬移前再比對，並行改動則拒絕，不以舊快照覆蓋新設定。
3. 搬整個 openspec/ 為 speclink/；規格、變更、討論、manual、完整 checks 歷史及既有資料檔內容逐位元不變。此明確轉換不批次改 metadata 名稱；.meta.yaml 優先／.openspec.yaml fallback 與個別寫入的新格式規則照 D3，不為模式另造格式。
4. 以 KeyLines 只改 spec_dir 為 util::yaml_scalar 處理過的 speclink，保留 tools、註解、未知鍵與其他設定，再原子寫入。資料夾搬移失敗時來源／設定維持原樣；設定寫失敗時回復原資料夾及設定。若回復也失敗，保留轉換記錄與資料，不對使用者回成功或讀成正常的空專案，錯誤指出實際路徑並允許同向續作。
5. 路徑與設定一致後，依已儲存的原工具選集同步技能。若此步失敗，保留 spec_dir: speclink 與 speclink/，不以回滾假裝支援反向轉換，保留未完成記錄，顯示尚未完成並提供「重試完成轉換」。重試不得再次搬移或增加重複鍵；完成後才清除轉換記錄。
6. 沿用桌面 updateAssets 的監看暫停／重掛邊界，清快取並重載設定、看板與開啟文件；watcher 改追 speclink/。中斷後重開專案依轉換記錄及實際資料夾顯示未完成狀態，只提供往 Speclink 的續作，不把缺少 openspec/ 當成新空專案。記錄的來源／目的固定值與實際路徑均須驗證，不信任手改的任意路徑。

續作先辨識並驗證轉換記錄與實際資料夾，再決定是否需要搬移；已搬到 speclink 的同向續作不因這次搬移本身造成的 Git 新增／刪除而被當成新的髒來源拒絕。記錄保留的原設定與依其計算的新設定，須與當下檔案相符；出現第三種並行修改則停止，不覆蓋。已完成、無轉換記錄的 Speclink 專案再呼叫同一入口只回現況，不建立反向操作或再次搬移。

SettingsSnapshot 只新增選填 specDirConversion: { pending: boolean, message?: string }，供未完成提示與續作按鈕；舊快照缺欄位視為沒有未完成轉換。原 specDir 仍是實際生效路徑。UI 的選項與文案跟隨 tw／en；本機轉換不讀工作流 locale／spec_locale，ja 或未設定不改變介面原有語言決策。完整互通保證、custom 轉換、反向轉換及自動提交不在本次範圍。

## Implementation Contract

| 使用者或呼叫者可見結果 | 介面與資料 | 失敗與驗收 |
| --- | --- | --- |
| 預設新專案與未主動轉換的升級仍使用 openspec/ | .speclink.yaml 的 spec_dir 為有效字串 openspec；明確 --dir／spec_dir 優先 | init、adopt、update 與 desktop 測試涵蓋新專案、舊專案、custom、已遷移、壞設定、寫失敗及重試；不先搬後補 |
| 專案設定可選規格資料夾模式 | 沿用 specDir，新增本機 convertSpecDirToSpeclink()；OpenSpec 為 openspec、Speclink 為 speclink | 選取 Speclink 先提示無法轉回；取消零寫入，確認一次才轉換；成功後不能選回，remote／custom 不轉換 |
| 單向轉換保留資料且可續作 | spec_dir 寫 speclink；選填 specDirConversion.pending 布林與 message 字串；暫存轉換記錄只在 .speclink/ | guard 拒絕零寫入；搬移與設定切換失敗保留原狀或明示可恢復狀態；技能失敗留在 Speclink 可重試；重開不呈現假空專案；歷史／工具／未知鍵完整保留 |
| 新資料檔可寫、舊資料可讀 | .meta.yaml 優先，.openspec.yaml fallback；remote archive 同鍵名規則 | FS／Host 交易故障注入與並行更新測試，不能遺失工單或品質歷史 |
| 不改規格的變更可完成流程 | status.schemaName、artifacts、applyRequires 及 schema 無 specs 拒寫 | 無 delta 可 validate／archive；含 delta 切到 no-spec-change 拒絕且零寫入 |
| 切換作用中變更流程 | change schema <name> <schema>，--json 回 change:string、schema:string、previous:string、changed:boolean；Server 另回 verifyStampCleared:boolean；無 stdin | 成功 0、拒絕非 0／stderr、--no-color 無 ANSI；reader／401／404／409／revision 及離線情境保留原資料 |
| 統計與品質歷史共同存在 | reviewed_rounds、verified_rounds 非負整數；created／started_at／reviewed_at／verified_at／archived_at 為新寫入秒級時間 | 舊日期可讀、壞輪數拒寫；蓋章成功只累加一次，歷史與安全條件不被覆蓋 |
| 已封存摘要讀取完整資料 | docLines、leadDays、leadHours、reviewRounds、verifyRounds 選填且缺值省略 | docLines 100／200／400 → median 200、P90 400；leadHours 6／30.5 → median 18.3；舊 remote 清單無摘要 |
| Fork 功能與發佈身份保留 | tools、語言、安全政策、checks 索引、updater endpoint／pubkey 與 portable 包裝契約 | 逐檔保留清單、既有回歸與 scripts 守門通過，不以編譯成功取代功能驗證 |

範圍內：proposal 列出的上游增量、metadata 引用調整與 Fork 交會處，以及 D8 本機設定的兩種規格資料夾模式與單向轉換。範圍外：改造 OpenSpec 相容層、遠端 schema 儲存抽象、自動發版與不相關死碼清理。所有 JSON 欄位使用 serde camelCase；新增選填欄位不要求舊 server 提供；人眼輸出只在 delta 明列處改變。tw／ja／en 的既有語言決策保留，新增成本文案依 UI 現有支援語言顯示；弱中文語言偵測不改。

## Risks / Trade-offs

- 回歸對照與 Fork 遺失 → 每個交集檔案記錄取捨，specs 的 MODIFIED 完整保留既有場景；golden 與 CLI 測試只在刻意改變處更新。
- Windows 改名鎖定、CRLF 與大小寫 → 採入上游跨平台處理，FS 舊檔／新檔共存、鎖定與回復測試保留；保留現有子程序 handle 修正；完整多平台測試由既有 CI 守門，單機結果不宣稱跨平台實測。
- 預設補值被當成自訂路徑 → desktop 到遠端遷移明確測試 openspec 值可接受，custom 與並存目錄在任何上傳前拒絕。
- .meta.yaml 寫入錯開品質歷史 CAS → expected 同時涵蓋來源與目的，測試新舊檔及索引並行變更、恢復故障，保持 Host 型別化 metadata 身分。
- 兩種資料夾與格式的語意混淆 → 文件明列本 Fork 的差異，路徑保留只解決資料夾遷移，完整 OpenSpec CLI 互通仍未驗證。
- 整合較大 → 任務按測試、核心、介面、UI、衍生物排序，每項有具體測試；不把上游所有測試一次新增後長時間留在全面失敗狀態。

- 設定選項被誤認為立即搬移 → 選取只打開不可逆確認，取消不呼叫後端；明確確認與一般工具儲存分開。
- 模式轉換中斷或技能寫入失敗 → 最小進度記錄保留原設定，搬移／設定失敗可回復或同向恢復；新路徑已生效後只往前續作，避免不一致設定被當成空專案。

## Migration Plan

1. 保存目前討論與提案，固定兩端提交並完成逐檔盤點；在 apply 的工作分支整合，保持本 repo 的 openspec/ 與來源討論連結。
2. 先以測試確認明確預設與設定守門，再補入 .speclink.yaml 的 spec_dir: openspec；整合上游時禁止尚未適配的引擎直接執行 update 而搬走此 repo。
3. 分批採入 metadata、schema、成本、Host／Protocol／adapters、桌面與技能功能，每批先重現新行為失敗，再完成實作並跑受影響測試。
4. 刷新生成資產與既有更新日誌衍生檔，完成全部保留清單；不改新版本號、不發佈。
5. 回退程式前先保留包含 .meta.yaml 與 checks 的專案副本；不直接以舊版覆蓋新版資料。正式回退若需要舊檔名，另依資料備份與明確轉換方案處理，本提案不新增反向遷移命令。

6. 套用第 10 組任務後，新設定頁仍以 OpenSpec 顯示本 repo；只有日後使用者在頁面選取 Speclink 並確認才轉換，所有預設與 update 回歸繼續成立。

## Open Questions

無阻擋實作的待決需求。新發佈版號、發佈時間及 OpenSpec 生態工具的完整互通保證均按討論結論留待後續，不作本次新增實作。
