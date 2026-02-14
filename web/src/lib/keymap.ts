import {
  IconArrowDown,
  IconArrowLeft,
  IconArrowRight,
  IconArrowUp,
  type Icon,
} from "@tabler/icons-react";

export const keys = {
  A: 1 << 0,
  B: 1 << 1,
  Select: 1 << 2,
  Start: 1 << 3,
  Right: 1 << 4,
  Left: 1 << 5,
  Up: 1 << 6,
  Down: 1 << 7,
  R: 1 << 8,
  L: 1 << 9,
};

export const defaultKeymaps: Record<string, number | undefined> = {
  KeyX: keys.A,
  KeyZ: keys.B,
  Space: keys.Select,
  Enter: keys.Start,
  ArrowRight: keys.Right,
  ArrowLeft: keys.Left,
  ArrowUp: keys.Up,
  ArrowDown: keys.Down,
  KeyS: keys.R,
  KeyA: keys.L,
};

export const keyIconMap: Record<string, Icon | undefined> = {
  Right: IconArrowRight,
  Left: IconArrowLeft,
  Up: IconArrowUp,
  Down: IconArrowDown,
};

export type Key = keyof typeof keys;
export type Keymap = typeof defaultKeymaps;

export function getActiveKeys(keypad: number): string[] {
  const activeKeys: string[] = [];

  for (const [name, value] of Object.entries(keys)) {
    if ((~keypad & value) !== 0) {
      activeKeys.push(name);
    }
  }

  return activeKeys;
}

export function getLabel(key: number): string {
  return Object.entries(keys)
    .find((pair) => key == pair[1])
    ?.at(0) as string;
}
