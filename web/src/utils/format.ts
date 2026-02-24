export function formatHex(
  value: number,
  params?: { width?: number; prefix?: string },
) {
  return `${params?.prefix ?? "0x"}${value.toString(16).padStart(params?.width ?? 8, "0")}`;
}

export function parseHex(value: string, params?: { prefix?: string }) {
  return parseInt(value.replace(params?.prefix ?? "0x", ""), 16);
}

export function getHexWidth(dataType: "Byte" | "HWord" | "Word") {
  if (dataType === "Byte") return 2;
  if (dataType === "HWord") return 4;
  return 8;
}

export function formatFileSize(size: number) {
  if (size < 1024) {
    return `${size} B`;
  } else if (size < 1024 ** 2) {
    return `${(size / 1024).toFixed(2)} KB`;
  } else {
    return `${(size / 1024 ** 2).toFixed(2)} MB`;
  }
}
