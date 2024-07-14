export class Keypad {
  onKeyDown: (key: number) => void;
  onKeyUp: (key: number) => void;

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

  constructor(onKeyDown, onKeyUp) {
    this.onKeyDown = onKeyDown;
    this.onKeyUp = onKeyUp;
  }

  addListeners = () => {
    document.addEventListener("keydown", this.keyDownEventHandler);
    document.addEventListener("keyup", this.keyUpEventHandler);
  };

  removeListeners = () => {
    document.removeEventListener("keydown", this.keyDownEventHandler);
    document.removeEventListener("keyup", this.keyUpEventHandler);
  };

  keyDownEventHandler = (event: KeyboardEvent) => {
    const key = Keypad.keyMap[event.key.toLowerCase()];
    if (key !== undefined) {
      this.onKeyDown(key);
    }
  };

  keyUpEventHandler = (event: KeyboardEvent) => {
    const key = Keypad.keyMap[event.key.toLowerCase()];
    if (key !== undefined) {
      this.onKeyUp(key);
    }
  };
}
