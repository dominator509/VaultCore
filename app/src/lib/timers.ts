export function nextCountdown(value: number): number {
  return Math.max(0, value - 1);
}

export function ttlToSeconds(ttlMs: number): number {
  return Math.max(1, Math.ceil(ttlMs / 1000));
}
