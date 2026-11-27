// relativeDays（fix-english-status-labels-and-plurals）：翻譯物件由呼叫端注入；英文 2 天以上用複數，
// 差 1 天固定寫「昨天」——所以 time.daysAgo 的單數形不會出現（desktop-config「數量為 0 或 2 以上時用複數」）。
import { describe, it, expect, vi, afterEach } from "vitest";

import { renderHook, act } from "@testing-library/react";
import { translatorFor } from "../i18n";
import { relativeDays, useLocalToday } from "../time";

const daysAgo = (days: number) => new Date(Date.now() - days * 86_400_000 - 3_600_000).toISOString();

describe("relativeDays", () => {
  const en = translatorFor("en");

  it("英文 3 天前寫「3 days ago」", () => {
    expect(relativeDays(daysAgo(3), en)).toBe("3 days ago");
  });

  it("差 1 天寫「Yesterday」，不寫「1 day ago」", () => {
    expect(relativeDays(daysAgo(1), en)).toBe(en.t("time.yesterday"));
  });

  it("繁中 3 天前用繁中字串", () => {
    const zh = translatorFor("zh-TW");
    expect(relativeDays(daysAgo(3), zh)).toBe(zh.t("time.daysAgo").replace("{n}", "3"));
  });

  it("沒有日期回 null，不可解析的字串原樣回傳", () => {
    expect(relativeDays(null, en)).toBeNull();
    expect(relativeDays("not a date", en)).toBe("not a date");
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("本地剛過午夜 00:05 時，前一日 YYYY-MM-DD 應顯示昨天（不受 UTC 時差影響）", () => {
    vi.useFakeTimers();
    // 2026-11-27 00:05:00 local time
    vi.setSystemTime(new Date(2026, 10, 27, 0, 5, 0));
    expect(relativeDays("2026-11-26", en)).toBe(en.t("time.yesterday"));
    expect(relativeDays("2026-11-27", en)).toBe(en.t("time.today"));
  });

  it("useLocalToday 在本地午夜跨日後自動刷新為新日期", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date(2026, 10, 26, 23, 59, 58));
    const { result } = renderHook(() => useLocalToday());
    expect(result.current).toBe("2026-11-26");

    act(() => {
      vi.advanceTimersByTime(3000);
    });
    expect(result.current).toBe("2026-11-27");
  });
});
