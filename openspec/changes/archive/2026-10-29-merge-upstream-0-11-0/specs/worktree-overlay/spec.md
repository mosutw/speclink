## MODIFIED Requirements

### Requirement: list 的 worktree 觀察面輸出
<!-- BEFORE: 既有需求沿用舊 metadata 名稱；本次改為新資料檔契約，保留未明列修改的行為。 -->

映射成立的 change，speclink list SHALL 呈現 worktree 觀察面：--json 條目 SHALL 增加 worktree 物件欄位（camelCase），含 path（字串，worktree 絕對路徑）與 branch（字串，分支全名）兩欄；人眼輸出 SHALL 於該 change 既有行尾追加固定字面「 [worktree]」，--no-color 下字面相同。無映射的 change 條目 SHALL NOT 出現 worktree 欄位（缺席不序列化）。此為刻意變更：既有無 worktree 情境的輸出維持位元級不變，新增情境的 golden 與 CLI 測試同批更新。

映射成立的 change，其任務計數、狀態與開工戳記等既有欄位的「值」SHALL 來自該 worktree 副本內的 change 目錄（openspec/changes/<change名>/），欄位名與型別 SHALL 維持既有契約不變；該 change 於主副本的檔案 SHALL NOT 被 list 讀取或寫入變更。worktree 移除後重跑 list，SHALL 回讀主副本且 worktree 觀察面消失，SHALL NOT 留下任何殘餘標示或欄位。worktree 副本內 change 中介資料損壞時，SHALL 沿用既有的損壞診斷欄位行為（metaError）如實呈現。

#### Scenario: 任務計數即時反映 worktree 副本

- **WHEN** 主副本的 add-dark-mode 完成 0 個任務，其 worktree 副本內勾完 3 個任務（共 5 個），於主 checkout 執行 speclink list --json
- **THEN** add-dark-mode 條目的 completedTasks 為 3、totalTasks 為 5，且含 worktree 欄位

##### Example: 計數與欄位形狀

- **GIVEN** worktree 位於 /repos/speclink.worktrees/add-dark-mode、分支 openspec/add-dark-mode，副本內 tasks.md 勾 3／5
- **WHEN** 於主 checkout 執行 speclink list --json
- **THEN** 該條目含 "completedTasks": 3、"totalTasks": 5、"worktree": { "path": "/repos/speclink.worktrees/add-dark-mode", "branch": "openspec/add-dark-mode" }

#### Scenario: 人眼輸出的標示

- **WHEN** 映射成立，執行 speclink list 與 speclink list --no-color
- **THEN** 該 change 行尾均出現「 [worktree]」字面，兩者字面一致

#### Scenario: worktree 移除後還原

- **WHEN** 先前映射成立的 worktree 以 git worktree remove 移除後，於主 checkout 執行 speclink list --json
- **THEN** 該條目無 worktree 欄位，任務計數回讀主副本的值

#### Scenario: worktree 副本中介資料損壞如實診斷

- **WHEN** 映射成立，但 worktree 副本內該 change 的 .meta.yaml 為無法解析的 YAML，執行 speclink list --json
- **THEN** 該條目呈現 metaError 欄位（與主副本損壞時同款行為）
