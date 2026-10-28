先按每組 RED → GREEN／REFACTOR 推進；採入上游測試時先確認它能捕捉尚未整合的行為，既有保留測試則先鎖定通過基線。每個實作步驟在受影響測試通過後才做必要整理，不改造額外抽象。此單一整合含多個小任務，因它們共享 metadata 與命令型別而不拆成彼此獨立的 apply。

## 1. 固定整合範圍

- [x] 1.1 D1 固定版本與逐檔整合：在 docs/development/upstream-0.11.0-inventory.md 記錄共同祖先、Fork／上游完整提交、雙方增量與逐檔處置；上游自用資料夾搬移不套入本 repo，現有討論與變更仍在 openspec/。驗證以 git merge-base、兩端 diff 與保留清單互相比對，所有交集檔都列明取捨；開始前保存原有文件，不能覆蓋使用者修改。 <!-- speclink-task:tsk_01M613JDCTFENR6RKT8YMFX0Y1 -->

## 2. 先測預設路徑再補入設定

- [x] 2.1 RED：在 crates/adapters/speclink-cli/tests/it/spec_dir_layout.rs、crates/adapters/speclink-cli/tests/it/spec_dir_migration.rs 及 crates/engine/speclink-core/src/workspace/init/tests.rs 建立新初始化明寫 openspec、update 先補值、自訂與已使用 speclink 佈局保留、壞設定／寫失敗不遷移／不更新技能的案例。以 cargo test -p speclink-cli --test it spec_dir_ 及 cargo test -p speclink-core workspace::init 驗證正確原因的失敗；既有路徑案例先確認基線。 <!-- speclink-task:tsk_01M613JDCT8S5PE3C966P5KC9K -->

- [x] 2.2 GREEN／REFACTOR：依 D2 明確預設與路徑入口，調整 crates/engine/speclink-core/src/workspace/init.rs、crates/engine/speclink-core/src/workspace/config.rs、crates/engine/speclink-core/src/workspace/workspace.rs、crates/engine/speclink-core/src/workspace/migrate.rs 與 crates/adapters/speclink-cli/src/verbs/init.rs，先決定實際資料夾再建骨架，adopt 就地補齊，update 守門後補值再規劃遷移，本 repo .speclink.yaml 明寫 openspec；重用 YAML 編輯及原子寫入。驗證重跑 2.1 的兩組測試，斷言重試不重複鍵、tools 與未知鍵保留，init／update 人眼與錯誤輸出只在刻意差異處改變。 對應需求：工作區補齊入口；本機專案明寫預設規格資料夾；init 匯入 OpenSpec 專案；update 自動遷移舊佈局；技能檔依實際規格資料夾渲染；技能足跡同步不遷移規格資料夾；規格資料夾的預設位置與舊佈局相容。 <!-- speclink-task:tsk_01M613JDCT1PY1783G2W8YC4MK -->

## 3. 新舊資料檔與品質歷史交易

- [x] 3.1 RED：在 crates/engine/speclink-fs/tests/store_fs.rs、crates/engine/speclink-fs/src/checks.rs 與 crates/host/speclink-host/tests/ 補上 .meta.yaml 優先、舊檔 fallback、兩檔並存、寫新刪舊、品質交易／journal 故障及 CAS 競爭的案例。驗證 cargo test -p speclink-fs 及 cargo test -p speclink-host，確認新檔名或交易假設導致的失敗，並保留既有 history_recovery_required 案例。 <!-- speclink-task:tsk_01M613JDCT0DG1RACNXP7CWD3N -->

- [x] 3.2 GREEN／REFACTOR：依 D3 新舊資料檔與歷史交易，採入 crates/engine/speclink-core/src/lifecycle/model.rs、crates/engine/speclink-core/src/store.rs、crates/engine/speclink-fs/src/layout.rs 與 crates/engine/speclink-fs/src/lib.rs 的新名稱與 fallback；新寫入只使用 .meta.yaml，讀不到新檔的錯誤不等同缺席，remote archive 不整批改鍵。驗證 cargo test -p speclink-fs 與 cargo test -p speclink-core，斷言讀取優先順序、舊資料相容及 .meta.yaml 成為唯一寫入來源。 對應需求：遠端封存變更資料檔的鍵；預設檔案系統佈局；預設檔案系統佈局不變。 <!-- speclink-task:tsk_01M613JDCT7XNMK3KX1ZPXZ3B1 -->

- [x] 3.3 GREEN／REFACTOR：在 crates/engine/speclink-core/src/quality/history.rs、crates/engine/speclink-core/src/quality/station.rs、crates/engine/speclink-fs/src/checks.rs 與 crates/host/speclink-host/src/bridge.rs 將 .meta.yaml 映射至 typed metadata，保留 ChecksCommit 的來源／目的 expected、索引及刪除工單同一交易。同步 crates/host/speclink-host/src/projection.rs 與既有排序、plan、討論鏈、worktree、trace 的資料檔定位；不另開繞過歷史的寫入路徑。驗證 cargo test -p speclink-fs、cargo test -p speclink-host 與 cargo test -p speclink-core，故障恢復後原工單／metadata／索引一致，revision 競爭零提交，JSON 原有欄位集合不變。 對應需求：change metadata 損壞的跨入口處置；穩定錯誤碼註冊表；投影佈局與 manifest；board_rank 不進 CLI 輸出且既有輸出逐位元不變；meta 寫入路徑對 board_rank 互不破壞；壞 metadata 不參與看板排序寫入；欄內存在缺 rank 卡時整欄補章；欄內拖排以中點 rank 單檔寫回；看板卡片順序以 board_rank 欄位為真相；change depends 動詞寫入宣告依賴；change rank 動詞寫入看板順序鍵；plan 動詞輸出波次與阻擋清單；討論以 link 動詞併入既有變更；討論重新結論標記已反映變更待重新反映；討論鏈結動詞對壞 change metadata 拒絕；拖排寫回以全文 CAS 與一次重試收斂；單環髒資料的寬容組裝；溯源鏈組裝；list 的 worktree 觀察面輸出。 <!-- speclink-task:tsk_01M613JDCTY1J8GHHY5CWC92WC -->

## 4. 兩種內建產出流程與切換核心

- [x] 4.1 RED：在 crates/adapters/speclink-cli/tests/it/no_spec_change_lifecycle.rs、crates/adapters/speclink-cli/tests/it/change_schema.rs 與 crates/adapters/speclink-cli/tests/it/schema_commands.rs 採入上游正常／拒絕案例，以現有 CLI 驗證新 schema、切換命令與無 specs 拒寫 delta 失敗，涵蓋 stdin／force、不合法名稱、含 delta、未結驗證工單、同值不變。驗證 cargo test -p speclink-cli --test it no_spec_change_lifecycle、cargo test -p speclink-cli --test it change_schema 及 cargo test -p speclink-cli --test it schema_commands。 <!-- speclink-task:tsk_01M613JDCT6N22S2Y6B72N53PG -->

- [x] 4.2 GREEN／REFACTOR：依 D4 產出流程與跨入口命令，在 crates/engine/speclink-core/assets/schema/no-spec-change/、crates/engine/speclink-core/src/workspace/schema.rs、crates/engine/speclink-core/src/lifecycle/change_schema.rs、crates/engine/speclink-core/src/command/、crates/engine/speclink-core/src/quality/validate.rs 採入兩個內建與 schema 守門，正常 no-spec-change validate／archive 不要求 delta，切換僅清驗證章且保留輪數與完整歷史。驗證 cargo test -p speclink-core；逐一核對 status／instructions 的 schemaName、artifacts、applyRequires 型別及 custom→user→built-in 解析順序。 對應需求：change schema 動詞切換變更的 schema；no-spec-change 內建 schema 的組成；schema 沒有 specs 產出時拒絕寫入 delta spec；單一正典載入。 <!-- speclink-task:tsk_01M613JDCTW9T698G1JS53DBE9 -->

## 5. 品質統計與時間相容

- [x] 5.1 RED：在 crates/engine/speclink-core/src/quality/station/tests.rs、crates/engine/speclink-core/src/lifecycle/archive/tests.rs 及 crates/adapters/speclink-cli/tests/it/meta_time_stamps.rs 建立輪數跨蓋章累加、壞輪數拒寫、清章保留輪數、歷史索引故障不部分寫入、純日期相容與封存重試時戳案例。驗證 cargo test -p speclink-core quality::station、cargo test -p speclink-core lifecycle::archive 及 cargo test -p speclink-cli --test it meta_time_stamps，確認新增統計／時間差異的失敗。 <!-- speclink-task:tsk_01M613JDCTXD87BFB2FGTGPRDC -->

- [x] 5.2 GREEN／REFACTOR：依 D5 輪數時戳與歷史保留，在 crates/engine/speclink-core/src/lifecycle/model.rs、crates/engine/speclink-core/src/quality/station.rs、crates/engine/speclink-core/src/util.rs 採入非負輪數及秒級時間，將欄位放進現有 ChecksCommit；保留 review／verify 歷史與安全驗收，accept 不繞過安全條件。驗證 cargo test -p speclink-core quality::station、cargo test -p speclink-fs 與 cargo test -p speclink-host；JSON 清單仍僅顯示既有日期與欄位，守門失敗／discard 不增加輪數。 對應需求：CLI 清單輸出的相容性釘住；放棄審查；蓋章守門與蓋章效果；蓋章記錄審查輪數；CLI 清單輸出的驗證欄位釘住；放棄驗證；蓋章記錄驗證輪數；驗證蓋章守門與蓋章效果。 <!-- speclink-task:tsk_01M613JDCT9XMJZE20GHV3KMRX -->

- [x] 5.3 GREEN／REFACTOR：在 crates/engine/speclink-core/src/lifecycle/archive.rs、crates/engine/speclink-core/src/lifecycle/inprogress.rs、crates/engine/speclink-core/src/lifecycle/newcmd.rs 與 crates/engine/speclink-core/src/lifecycle/cost.rs 以同一時間產生 archive 日期、archived_at 與 trace，移動失敗留活動變更並可重試；不為缺 metadata 的舊變更新建資料檔，保留 Fork 封存檢查紀錄與安全摘要。驗證 cargo test -p speclink-core lifecycle、cargo test -p speclink-cli --test it meta_time_stamps；舊日期可讀、未知值不補 0、讀取成本不會反寫歷史。 對應需求：change metadata 時間欄位寫入時戳、對外輸出日期；in-progress 標記可自 change meta 移除(零工作痕跡守門)；in-progress 標記真相存於 change meta；restale_from 記錄變更待重新反映的討論並經 CLI 觀測；任務完成蘊含開工標記；壞 metadata 使生命週期寫入 fail closed；封存寫入 archived_at；歸檔保留完整生命週期歸屬。 <!-- speclink-task:tsk_01M613JDCT599ZM068W72VR5GT -->

## 6. Host 與各操作入口

- [x] 6.1 RED：在 crates/adapters/speclink-cli/tests/it/remote_verb_parity.rs、crates/host/speclink-server/tests/it/api/、crates/adapters/speclink-node/__test__/dispatch-contract.spec.ts 增補 change schema 的 Local／Remote／SDK 同形、reader／401／404／409、stale revision／離線與同值零事件案例；workflow-config show 同時斷言 schema 與 securityVerification。驗證 cargo test -p speclink-cli --test it remote_verb_parity 及 cargo test -p speclink-server --test it，確認尚缺命令或欄位的失敗；Node 案例依 6.4 建置後執行。 <!-- speclink-task:tsk_01M613JDCTKB1DEM8D083RMS1R -->

- [x] 6.2 GREEN／REFACTOR：在 crates/protocol/speclink-protocol/src/command.rs、crates/protocol/speclink-remote/src/client.rs、crates/host/speclink-host/src/bridge.rs、crates/host/speclink-server/src/api/routes.rs 共用核心切換邏輯，保留權限、revision、transaction、outbox；remote 只解析內建，成功／拒絕／同值依 delta 回應。驗證 cargo test -p speclink-protocol、cargo test -p speclink-remote、cargo test -p speclink-host 及 cargo test -p speclink-server；reader 或 stale revision 不寫入，changed=false 無事件。 對應需求：變更 schema 寫入端點。 <!-- speclink-task:tsk_01M613JDCTW4ACVRFWDNJR9M77 -->

- [x] 6.3 GREEN／REFACTOR：在 crates/adapters/speclink-cli/src/main.rs、crates/adapters/speclink-cli/src/verbs/plan.rs、crates/adapters/speclink-cli/src/verbs/config.rs 與 crates/engine/speclink-core/src/workspace/config.rs 接上 change schema 與 schema 查詢；保留安全物件、review 行、系統／Server 語言預設。驗證重跑第 4 組 CLI 測試、cargo test -p speclink-cli --test it workflow_config 及 remote_verb_parity；JSON 欄位 camelCase 且型別正確、無 stdin 的命令不取 stdin、--no-color 無 ANSI、失敗 stderr 不混入成功 payload。 對應需求：workflow-config show 動詞。 <!-- speclink-task:tsk_01M613JDCT78191CWJBBSG4SGW -->

- [x] 6.4 GREEN／REFACTOR：在 crates/adapters/speclink-node/src/lib.rs、crates/adapters/speclink-node/src/store_bridge.rs、crates/adapters/speclink-node/src/render.rs、crates/adapters/speclink-node/index.d.ts 與 crates/adapters/speclink-node/__test__/ 接上上游命令、Store 能力與結果型別，保留 Fork render targets 及品質歷史 DocumentId。驗證先執行 npm --prefix crates/adapters/speclink-node run build，再執行 npm --prefix crates/adapters/speclink-node test；確認 dispatch schema 切換、舊 API 及宣告型別一致。 <!-- speclink-task:tsk_01M613JDCTQS80R23F7BV6KZMT -->

## 7. 桌面路徑與流程成本

- [x] 7.1 RED：在 apps/desktop/core/src/project.rs、apps/desktop/core/src/settings.rs、apps/desktop/core/src/manage.rs 的既有測試與 apps/desktop/src/__tests__/ 建立初始化／啟用明寫 openspec、update 不搬目錄、顯示實際路徑、明寫預設可遷移到遠端、custom／雙目錄拒絕、remote 雙內建 schema 案例。驗證 cargo test -p speclink-desktop-core 與 npm test -w apps/desktop，確認新預設或守門差異的失敗，取消操作仍零寫入。 <!-- speclink-task:tsk_01M613JDCTTN2GWN4MXXSPCKNS -->

- [x] 7.2 GREEN／REFACTOR：依 D2、D6 桌面摘要與共用呈現，在 apps/desktop/core/src/project.rs、apps/desktop/core/src/settings.rs、apps/desktop/core/src/manage.rs、apps/desktop/core/src/cache.rs、apps/desktop/core/src/query.rs 與 apps/desktop/src-tauri/src/ 接上共用路徑、兩個 schema 與一次載入成本；Tauri 保持委派，成功匯入後才備份並寫 marker，watcher 跟隨實際路徑。驗證 cargo test -p speclink-desktop-core；docLines 等五個數字欄位缺值不序列化，舊 remote 回應可讀，既有安全／品質歷史／獨立模式測試保持通過。 對應需求：remote 模式的內建限縮與誤解析修正；未初始化目錄經確認後自動初始化；未啟用資料夾經確認後補齊啟用；畫面依實際規格資料夾與資料檔名顯示；新舊預設資料夾都可遷移到遠端；遷移成功後才轉換本地且一律備份；已封存清單的流程成本欄位。 <!-- speclink-task:tsk_01M613JDCTTGYH5YJWJDHJNNGD -->

- [x] 7.3 RED：在 packages/ui/src/__tests__/processCost.test.ts、packages/ui/src/__tests__/processCostSummary.test.tsx、packages/ui/src/__tests__/archivedList.test.tsx 與 apps/desktop/src/__tests__/assetNotice.test.tsx 採入上游具體例子：100／200／400 的文件行數中位數 200、P90 400、兩筆 6／30.5 小時顯示 18.3、搜尋不縮小樣本、缺資料隱藏；更新提示不聲稱預設搬移。驗證 npm test -w packages/ui 與 npm test -w apps/desktop，確認新增摘要與提示差異的失敗。 <!-- speclink-task:tsk_01M613JDCTR57B49CD31MPHT58 -->

- [x] 7.4 GREEN／REFACTOR：在 packages/ui/src/processCost.ts、packages/ui/src/components/ProcessCostSummary.tsx、packages/ui/src/components/ArchivedList.tsx、packages/ui/src/adapter.ts、packages/ui/src/i18n.tsx 及 apps/desktop/src/ 設定／提示元件採入摘要與實際路徑顯示，沿用共用元件與變體；保留桌面外觀、專案排序、Proxy 診斷、安全規則說明及品質歷史入口。驗證 npm test -w packages/ui、npm test -w apps/desktop 與 npm test -w apps/server-web，確認共用改動未影響 Server Web。 對應需求：GUI 勾任務與 CLI 完成語意一致；已封存頁的流程成本摘要；技能檔提示承載規格資料夾遷移；詳情頁的排程分頁。 <!-- speclink-task:tsk_01M613JDCTGEZHP7Y8MN4SXYMF -->

## 8. 技能與外掛衍生物

- [x] 8.1 RED：在 crates/engine/speclink-core/tests/it/render_golden.rs 與 scripts/claude-code/skill-groups.test.mjs 增加 schema 分流、無規格驗證、主分支重疊與 CI 附註、openspec 路徑及四工具 Fork 內容保留的斷言。驗證 cargo test -p speclink-core --test it render_golden:: 及 node --test scripts/*.test.mjs scripts/*/*.test.mjs，確認新內容造成的失敗，不先更新 golden 掩蓋缺失。 <!-- speclink-task:tsk_01M613JDCTA0RGCE05P1THNYW9 -->

- [x] 8.2 GREEN／REFACTOR：依 D7 技能衍生物與交付保留，整合 crates/engine/speclink-core/assets/skills/、crates/engine/speclink-core/assets/schema/、crates/engine/speclink-core/src/workspace/skills.rs 的上游 schema 與 quality 指引；保留安全指引、Antigravity、系統／Server 語言預設及提案條列。審閱差異後分別用 UPDATE_GOLDEN／UPDATE_ASSETS_LOCK 的既有開關重產快照與指紋，再由整合引擎更新 .agents/skills/、.claude/skills/、.github/skills/ 受管技能。驗證 cargo test -p speclink-core --test it render_golden::、cargo test -p speclink-cli --test it init_tools 與 instructions_policy；生成內容使用正確工具語法，不把本 repo 搬走。 對應需求：依是否改到正式規格選擇 schema；新需求改到承諾行為時切回 spec-driven；不改規格變更改到行為時的修正步驟；不改規格變更的行為檢查；兩站時序的編排行為；蓋章前的預防提醒。 <!-- speclink-task:tsk_01M613JDCTVDEQR2CE0EJ4PRY8 -->

- [x] 8.3 在 integrations/claude-code/speclink-skills/hooks/ 與 integrations/claude-code/speclink-skills/tests/register.test.tsx 對照上游資料檔名稱後，保留 session-focus、技能列與面板原有互動；若 hook 經 CLI 取得資料而無須修改，盤點明列此理由。驗證執行 README 已載明的 claude plugin test integrations/claude-code/speclink-skills及 node --test scripts/*.test.mjs scripts/*/*.test.mjs，斷言只填入指令、保持參數與封存完成標記，避免無關外掛改寫。 對應需求：依封存後清單標示完成；點技能只填入指令並保留參數；面板開關與名稱填入。 <!-- speclink-task:tsk_01M613JDCTS97T45GYTV4W60A9 -->

## 9. 文件、保留清單與最後驗證

- [x] 9.1 在 README.md、README.zh-TW.md、docs/、openspec/manual/、apps/desktop/src/release-notes/release-notes.json、CHANGELOG.md 與 CHANGELOG.zh-TW.md 採入 D7 的定位與預設說明：明寫 openspec、不宣稱完整 OpenSpec 互通、.meta.yaml 舊檔相容、品質提醒；不改寫歷史 artifacts 或提前加入未選版號的新發佈條目，兩份 CHANGELOG 由現有 JSON 重產。驗證 node scripts/release/release-notes-render.mjs --check 及 node --test scripts/*.test.mjs scripts/*/*.test.mjs，中英文事實與連結一致。 對應需求：workflow 文件載明蓋章前的預防規矩；使用者文件採漸進揭露與單一責任；使用者文件載明本地產物的 OpenSpec 結構相容性；使用者文件載明本地產物的純文字可攜性；入門教學與設定說明載明規格資料夾的遷移。 <!-- speclink-task:tsk_01M613JDCVHEV2BY5MQ6FFEYT9 -->

- [x] 9.2 依 audit discipline 檢查本次路徑預設、schema 名稱與新寫入邊界，在 docs/development/upstream-0.11.0-inventory.md 列出搜尋 spec_dir.is_some、DEFAULT_SPEC_DIR、.openspec.yaml、.meta.yaml 的結果，逐一標記新舊檔 fallback、typed metadata、CAS 與 user schema 邊界；沿既有失敗測試修正本次引入的危險預設。驗證新命令不讀任意路徑、reader／離線／revision 錯誤不部分寫入，安全政策與 audit 開關均未改動；重跑對應失敗案例的 package 測試。 <!-- speclink-task:tsk_01M613JDCVKPVPKP3XMWJ6BZQJ -->

- [x] 9.3 完成 docs/development/upstream-0.11.0-inventory.md 保留矩陣，對照 .github/workflows/、apps/desktop/src-tauri/tauri.conf.json、scripts/desktop/、scripts/release/、crates/engine/speclink-core/tests/golden/ 與 .speclink.yaml；確認更新 endpoint／現有 pubkey／GHCR／Windows portable／三平台建置守門、Fork 安全與歷史、四工具、語言及桌面功能均有保存或測試證據。驗證 node --test scripts/*.test.mjs scripts/*/*.test.mjs、cargo test -p speclink-core --test it render_golden:: 及 cargo test -p speclink-cli；受影響的 FS／Host／Protocol／Server／desktop-core、三個前端 workspace 與 Node SDK 測試沿前述 package 命令完成且無未解失敗，完整多平台組合交既有 CI，未執行部分如實記錄，不更換版本或發佈。 <!-- speclink-task:tsk_01M613JDCVVCX5D3TG2M7NBZVF -->

## 10. 規格資料夾模式與單向轉換（2026-10-28 ingest）

原第 1～9 組的已完成項目保留；下列任務擴充 D8，不重開原先預設不搬移的完成紀錄。第 8.3 項外掛測試仍維持原有未完成狀態。

- [x] 10.1 RED：在 crates/engine/speclink-core/src/workspace/migrate.rs 的測試及 crates/engine/speclink-core/src/workspace/init/tests.rs 為 D8 規格資料夾模式與單向轉換建立案例：明確 spec_dir: openspec 可主動轉成 speclink、同向重試冪等、remote／custom／兩目錄／目的地檔案或 symlink／來源 symlink／worktree／髒規格／壞設定／降級拒絕零寫入，以及設定並行改動不被舊文字覆蓋。驗證 cargo test -p speclink-core，記錄尚缺明確轉換行為的失敗；新專案與一般 update 維持 OpenSpec 預設的既有案例先鎖定通過。對應需求：規格資料夾模式的明確單向轉換。 <!-- speclink-task:tsk_01M619FZSMWAK5CASXSFYNKCN3 -->

- [x] 10.2 GREEN／REFACTOR：依 D8 在 crates/engine/speclink-core/src/workspace/migrate.rs、crates/engine/speclink-core/src/workspace/init.rs 重用既有路徑、守門、Windows rename 重試、KeyLines／yaml_scalar 與技能同步，提供固定 openspec→speclink 的唯一流程；不得刪 spec_dir 觸發 update、不得增加反向或任意路徑目標。以 .speclink/spec-dir-conversion.json 保留原設定與進度，搬資料夾後明寫 speclink，tools／未知鍵／註解及原文件與 metadata／checks 逐位元保留；不批次改 metadata 名稱。驗證重跑 cargo test -p speclink-core 的 10.1 案例，斷言成功後 update 不搬回、無重複鍵。對應需求：規格資料夾模式的明確單向轉換；技能檔依實際規格資料夾渲染。 <!-- speclink-task:tsk_01M619FZSM75EM4YTZVS89BBHZ -->

- [x] 10.3 RED→GREEN：在 crates/engine/speclink-core/src/workspace/migrate.rs 的既有測試邊界注入資料夾改名、設定寫入／回復與技能同步失敗，先重現 D8 的中斷狀態，再補上回復或同向續作；前兩步失敗不回假成功，回復也失敗時保留資料與恢復記錄，路徑生效後技能失敗留在 Speclink 並可重開續作，成功才清掉記錄。驗證 cargo test -p speclink-core 與 cargo test -p speclink-fs，逐次斷言原設定、實際資料夾、原工單與歷史內容、四工具技能路徑與重試結果；手改進度記錄不能導向任意路徑。對應需求：規格資料夾模式的明確單向轉換。 <!-- speclink-task:tsk_01M619FZSMQ2YHP5CXZ8DJ54DN -->

- [x] 10.4 RED→GREEN：在 apps/desktop/core/src/settings.rs 增加 convert_spec_dir_to_speclink_at(root) 與快照的選填 specDirConversion.pending/message，測試先涵蓋預設／既有 Speclink／custom／remote、失敗與重開的實際路徑，再接上 core 唯一流程；於 apps/desktop/src-tauri/src/lib.rs、apps/desktop/src-tauri/src/watch.rs 沿既有 updateAssets 邊界暫停並重掛監看，在 apps/desktop/src/session.ts 與 apps/desktop/src/adapter/workspace.ts 加本機可選 convertSpecDirToSpeclink()，remote 不提供也不發請求。驗證 cargo test -p speclink-desktop-core、cargo check -p speclink-desktop 與 npm test -w apps/desktop 的 adapter／workspace 測試；成功 payload 的 specDir 是字串 speclink、pending 是布林、message 是選填字串，舊快照缺新欄位仍可讀，外部檔案修改仍可更新看板與文件。對應需求：專案設定顯示規格資料夾模式並確認單向轉換。 <!-- speclink-task:tsk_01M619FZSMPJNNZF5FDGDNPMSR -->

- [x] 10.5 RED：在 apps/desktop/src/__tests__/projectSettingsView.test.tsx 及 apps/desktop/src/__tests__/workspace.test.ts 先測 D8 的畫面流程：預設與既有模式、選取只開不可逆確認、取消／Escape／關閉不呼叫、確認僅一次、執行中禁重送、成功禁選回、失敗維持實際現值與同向重試、remote／custom／parseError／未儲存工具不轉換，並覆蓋 tw/en 的警告。驗證 npm test -w apps/desktop，確認尚缺模式選項與確認步驟的失敗。對應需求：專案設定顯示規格資料夾模式並確認單向轉換。 <!-- speclink-task:tsk_01M619FZSMVNSB9H8MF9J3T0HG -->

- [x] 10.6 GREEN／REFACTOR：依 D8 在 apps/desktop/src/views/project-settings/SpeclinkPage.tsx、apps/desktop/src/views/project-settings/useProjectSettings.ts、apps/desktop/src/views/ProjectSettingsView.tsx 與 apps/desktop/src/i18n/messages.ts，於設定檔區塊接上 OpenSpec／Speclink 選項；重用 @speclink/ui 的 SettingsRow、Select、ConfirmDialog 與 Button，選取只開確認、確認才轉換，文案明說轉換後無法轉回，成功重載快照／文件與看板、未完成可同向重試。保留工具編輯與其他未儲存表單，不用轉換隱含儲存。驗證 npm test -w apps/desktop 全套與 tsc --noEmit -p apps/desktop，10.5 案例全部通過，無新增自建共用元件外觀。對應需求：專案設定顯示規格資料夾模式並確認單向轉換。 <!-- speclink-task:tsk_01M619FZSMBHQ64J91K406N10C -->

- [x] 10.7 在 README.md、README.zh-TW.md、docs/getting-started.md、docs/getting-started.zh-TW.md、docs/configuration.md、docs/configuration.zh-TW.md、openspec/manual/spec-folder.md、openspec/manual/desktop-projects.md 及 docs/development/upstream-0.11.0-inventory.md 補上 D8 的入口、兩模式與預設、先確認不可逆、取消零寫入、轉換成功與未完成重試、自訂／remote 限制；保留 .meta.yaml 相容範圍與一般 update 不搬 OpenSpec 的說明，不宣稱兩模式等於完整 OpenSpec 互通。驗證 node --test scripts/*.test.mjs scripts/*/*.test.mjs，確認中英同事實與連結、手冊原有安全／品質歷史說明保留。對應需求：文件說明規格資料夾模式的單向轉換。 <!-- speclink-task:tsk_01M619FZSMGJ9VHZB7TXT6JBKY -->

- [x] 10.8 完成 D8 規格資料夾模式與單向轉換的回歸／audit：在 docs/development/upstream-0.11.0-inventory.md 記錄搜尋 spec_dir.is_some、DEFAULT_SPEC_DIR、openspec/speclink 模式選項及反向入口的結果，逐一說明保留的自動 update 守門與專用明確轉換，確認轉換不覆蓋目的地、不洗掉原設定／工單／checks，也沒有反向、custom 或 remote 寫入。驗證 cargo test -p speclink-core、cargo test -p speclink-fs、cargo test -p speclink-desktop-core、cargo test -p speclink-cli、cargo test -p speclink-core --test it render_golden::、npm test -w apps/desktop，以及 node --test scripts/*.test.mjs scripts/*/*.test.mjs；CLI 人眼／JSON、四工具、語言、安全政策與 Fork 發佈身份均依既有契約保留。本 repo 仍為 openspec，不由測試／套用自行轉換。 <!-- speclink-task:tsk_01M619FZSM8Y03C5NZKBXM1800 -->
