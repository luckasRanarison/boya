export function formatHex(
  value: number,
  params?: { width?: number; prefix?: string },
) {
  return `${params?.prefix ?? "0x"}${value.toString(16).padStart(params?.width ?? 8, "0")}`;
}

export function parseHex(value: string, params?: { prefix?: string }) {
  return parseInt(value.replace(params?.prefix ?? "0x", ""), 16);
}
