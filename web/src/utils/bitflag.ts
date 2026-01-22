import type { Flag } from "boya_wasm";

export function getFlagValue(value: number, flag: Flag) {
  return (value >> flag.start) & ((1 << flag.length) - 1);
}

export function getFlagBits(value: number, flag: Flag) {
  return getFlagValue(value, flag)
    .toString(2)
    .padStart(flag.length, "0")
    .split("");
}
