export function formatHex(value: number, width = 8) {
  return `0x${value.toString(16).padStart(width, "0")}`;
}
