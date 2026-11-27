import { useEffect, useState } from "react";
import type { Translator } from "./i18n";

/** 本地日曆日 YYYY-MM-DD。 */
export function localDateString(now: Date = new Date()): string {
  const y = now.getFullYear();
  const m = String(now.getMonth() + 1).padStart(2, "0");
  const d = String(now.getDate()).padStart(2, "0");
  return `${y}-${m}-${d}`;
}

/** 距離下一個本地午夜的毫秒數（含 100ms 緩衝，避免提早觸發仍停在舊日）。 */
export function msUntilNextLocalMidnight(now: Date = new Date()): number {
  const next = new Date(now.getFullYear(), now.getMonth(), now.getDate() + 1, 0, 0, 0, 100);
  return Math.max(100, next.getTime() - now.getTime());
}

/** 訂閱本地日曆日：午夜及視窗回到前景時自動更新，讓相對時間與日期篩選無需重開即刷新。 */
export function useLocalToday(): string {
  const [today, setToday] = useState(() => localDateString());
  useEffect(() => {
    let timer: ReturnType<typeof setTimeout> | null = null;
    const sync = () => {
      const next = localDateString();
      setToday((prev) => (prev === next ? prev : next));
    };
    const schedule = () => {
      if (timer !== null) clearTimeout(timer);
      timer = setTimeout(() => {
        sync();
        schedule();
      }, msUntilNextLocalMidnight());
    };
    schedule();
    const onResume = () => {
      if (typeof document === "undefined" || document.visibilityState !== "hidden") sync();
    };
    if (typeof window !== "undefined") window.addEventListener("focus", onResume);
    if (typeof document !== "undefined") document.addEventListener("visibilitychange", onResume);
    return () => {
      if (timer !== null) clearTimeout(timer);
      if (typeof window !== "undefined") window.removeEventListener("focus", onResume);
      if (typeof document !== "undefined") document.removeEventListener("visibilitychange", onResume);
    };
  }, []);
  return today;
}

/** 相對時間（天級即可；日期通常是 YYYY-MM-DD）。翻譯函式由呼叫端注入；無日期回 null、
 * 不可解析原樣回傳（可見的失敗，非靜默錯值）。差 1 天回「昨天」，所以 `time.daysAgo` 實際只在
 * 2 天以上出現——它的 `.one` 是字典守門要求的完整性，不會顯示。 */
export function relativeDays(date: string | null | undefined, { t, tn }: Translator): string | null {
  if (!date) return null;
  let targetDay: Date;
  const dateOnly = /^(\d{4})-(\d{2})-(\d{2})$/.exec(date.trim());
  if (dateOnly) {
    targetDay = new Date(Number(dateOnly[1]), Number(dateOnly[2]) - 1, Number(dateOnly[3]));
  } else {
    const parsed = Date.parse(date);
    if (Number.isNaN(parsed)) return date;
    const d = new Date(parsed);
    targetDay = new Date(d.getFullYear(), d.getMonth(), d.getDate());
  }
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const days = Math.round((today.getTime() - targetDay.getTime()) / 86_400_000);
  if (days <= 0) return t("time.today");
  if (days === 1) return t("time.yesterday");
  return tn("time.daysAgo", days);
}
