import type { Flag } from "boya_wasm";

export function getFlagValue(value: number, flag: Flag) {
  if (flag.length === 1) {
    return (value >> flag.start) & 1;
  } else {
    return (value >> flag.start) & ((1 << (flag.length + 1)) - 1);
  }
}
