import test from "node:test";
import assert from "node:assert/strict";
import {
  PALETTE_COMMANDS,
  createVisibilityTracker,
  detailUrlForPath,
  waitingReason,
} from "../../integrations/herdr/mods/speclink/model.js";

test("Herdr 0.1.18 視窗不可見時暫停輪詢，恢復可見時補抓一次", () => {
  let visible = false;
  let polls = 0;
  let resumes = 0;
  const tracker = createVisibilityTracker(
    () => visible,
    () => {
      resumes += 1;
    },
  );

  // 隱藏期間：計時器不觸發拉取
  assert.equal(tracker.tick(() => { polls += 1; }), false);
  assert.equal(tracker.tick(() => { polls += 1; }), false);
  assert.equal(tracker.isPaused(), true);
  assert.equal(polls, 0);
  assert.equal(resumes, 0);

  // 恢復可見：補抓一次且不重複計數
  visible = true;
  assert.equal(tracker.sync(), true);
  assert.equal(resumes, 1);
  assert.equal(tracker.isPaused(), false);
  assert.equal(tracker.sync(), false);
  assert.equal(resumes, 1);

  // 恢復後正常輪詢
  assert.equal(tracker.tick(() => { polls += 1; }), true);
  assert.equal(polls, 1);
});

test("worktree 路徑透過 detailUrlForPath 回到主專案詳情網址", () => {
  const board = {
    stages: {
      "in-progress": [
        { name: "wt-feat", worktree: { path: "/repo/.worktrees/wt-feat", branch: "feat" } },
      ],
    },
  };
  assert.equal(
    detailUrlForPath("/repo/.worktrees/wt-feat/openspec/changes/wt-feat/tasks.md", {
      root: "/repo",
      board,
    }),
    "speclink://change/wt-feat?project=%2Frepo",
  );
  assert.equal(
    detailUrlForPath("/repo/openspec/discussions/api-design.md", { root: "/repo", board }),
    "speclink://discussion/api-design?project=%2Frepo",
  );
});

test("waitingReason 抽取等候允許/回答/登入原因，PALETTE_COMMANDS 提供面板與技能入口", () => {
  assert.equal(waitingReason({ view: "blocked", waitingReason: "等待允許執行指令" }), "等待允許執行指令");
  assert.equal(waitingReason({ view: "blocked", reason: "請先登入" }), "請先登入");
  assert.equal(waitingReason({ view: "blocked" }), null);
  assert.deepEqual(
    PALETTE_COMMANDS.map((c) => c.id),
    ["mod.open:speclink", "mod.open:speclink-new"],
  );
});
