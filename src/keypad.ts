import * as cpu from "chip8-emulator-wasm";

export class Keypad {
  static keyMap = {
    1: 0x1,
    2: 0x2,
    3: 0x3,
    4: 0xc,
    q: 0x4,
    w: 0x5,
    e: 0x6,
    r: 0xd,
    a: 0x7,
    s: 0x8,
    d: 0x9,
    f: 0xe,
    z: 0xa,
    x: 0x0,
    c: 0xb,
    v: 0xf,
  };

  constructor() {}

  addListeners = () => {
    document.addEventListener("keydown", Keypad.keyDownEventHandler);
    document.addEventListener("keyup", Keypad.keyUpEventHandler);
  };

  removeListeners = () => {
    document.removeEventListener("keydown", Keypad.keyDownEventHandler);
    document.removeEventListener("keyup", Keypad.keyUpEventHandler);
  };

  static keyDownEventHandler = (event: KeyboardEvent) => {
    const key = Keypad.keyMap[event.key.toLowerCase()];
    if (key !== undefined) {
      cpu.set_key_down(key);
    }
  };

  static keyUpEventHandler = (event: KeyboardEvent) => {
    const key = Keypad.keyMap[event.key.toLowerCase()];
    if (key !== undefined) {
      cpu.set_key_up(key);
    }
  };
}
