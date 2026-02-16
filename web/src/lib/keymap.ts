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

export const controls = {
  reset: 0,
  toggleRun: 1,
  stepInto: 2,
  stepOut: 3,
  stepScanline: 4,
  stepFrame: 5,
  stepIRQ: 4,
  stop: 7,
};

export type Key = keyof typeof keys;

export type Keymap = Record<
  string,
  { type: "gamepad"; value: number } | { type: "debugger"; action: number }
>;

export const defaultKeymaps: Keymap = {
  KeyX: { type: "gamepad", value: keys.A },
  KeyZ: { type: "gamepad", value: keys.B },
  Space: { type: "gamepad", value: keys.Select },
  Enter: { type: "gamepad", value: keys.Start },
  ArrowRight: { type: "gamepad", value: keys.Right },
  ArrowLeft: { type: "gamepad", value: keys.Left },
  ArrowUp: { type: "gamepad", value: keys.Up },
  ArrowDown: { type: "gamepad", value: keys.Down },
  KeyS: { type: "gamepad", value: keys.R },
  KeyA: { type: "gamepad", value: keys.L },

  F5: { type: "debugger", action: controls.toggleRun },
  F11: { type: "debugger", action: controls.stepInto },
  ["Shift+F11"]: { type: "debugger", action: controls.stepOut },
  F9: { type: "debugger", action: controls.reset },
  KeyR: { type: "debugger", action: controls.stepFrame },
  KeyI: { type: "debugger", action: controls.stepIRQ },
  KeyL: { type: "debugger", action: controls.stepScanline },
  Escape: { type: "debugger", action: controls.stop },
};

export const keyIconMap: Record<string, Icon | undefined> = {
  Right: IconArrowRight,
  Left: IconArrowLeft,
  Up: IconArrowUp,
  Down: IconArrowDown,
};

export function getActiveKeys(keypad: number): string[] {
  const activeKeys: string[] = [];

  for (const [name, value] of Object.entries(keys)) {
    if ((~keypad & value) !== 0) {
      activeKeys.push(name);
    }
  }

  return activeKeys;
}

export function encodeKeyEvent(event: KeyboardEvent) {
  const mappings: string[] = [];

  if (event.ctrlKey) mappings.push("Ctrl");
  if (event.altKey) mappings.push("Alt");
  if (event.shiftKey) mappings.push("Shift");
  mappings.push(event.code);

  return mappings.join("+");
}

export function formatGamepadKey(key: number) {
  return Object.entries(keys)
    .find((pair) => key == pair[1])
    ?.at(0) as string;
}

export function formatKeyAction(action: number) {
  const actionMap = [
    "Reset",
    "Pause/Continue",
    "Step into",
    "Step out",
    "Step scanline",
    "Step frame",
    "Step IRQ",
    "Stop",
  ];

  return actionMap[action];
}
