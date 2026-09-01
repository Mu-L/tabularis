import { useCallback, useEffect, useRef, useState } from "react";

/**
 * Encapsulates the common "copy text → show checkmark → reset" pattern
 * used across dozens of components. Handles cleanup on unmount so
 * `setState` is never called on an unmounted component.
 *
 * @param resetMs  Time in ms before `copied` resets to false (default 2000).
 * @returns `{ copied, copy }` — `copy(text)` writes `text` to the
 *          clipboard and flips `copied` to true for `resetMs` ms.
 */
export function useCopyFeedback(resetMs = 2000) {
  const [copied, setCopied] = useState(false);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const clearTimer = useCallback(() => {
    if (timerRef.current !== null) {
      clearTimeout(timerRef.current);
      timerRef.current = null;
    }
  }, []);

  // Cleanup on unmount — prevents setState after unmount.
  useEffect(() => clearTimer, [clearTimer]);

  const copy = useCallback(
    async (text: string) => {
      await navigator.clipboard.writeText(text);
      clearTimer();
      setCopied(true);
      timerRef.current = setTimeout(() => setCopied(false), resetMs);
    },
    [clearTimer, resetMs],
  );

  const reset = useCallback(() => {
    clearTimer();
    setCopied(false);
  }, [clearTimer]);

  return { copied, copy, reset } as const;
}
