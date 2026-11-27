import { describe, it, expect } from "vitest";
import { BUILTIN_TOOLS, builtinToolLabel } from "../builtinTools";

describe("upstream 0.20.0 desktop regressions", () => {
  it("BUILTIN_TOOLS 包含 pi 與 cursor 及其技能根目錄", () => {
    expect(BUILTIN_TOOLS).toContain("pi");
    expect(BUILTIN_TOOLS).toContain("cursor");
    expect(builtinToolLabel("pi" as never)).toBe("pi [.pi]");
    expect(builtinToolLabel("cursor" as never)).toBe("cursor [.cursor]");
  });
});
