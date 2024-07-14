import init, * as emulator from "chip8-emulator-wasm";

import { SoundPlayer } from "./sound";
import { Keypad } from "./keypad";
import { Display } from "./display";
import { RomSelector } from "./rom";

class Emulator {
  inner: emulator.Emulator;
  keypad: Keypad;
  display: Display;
  soundPlayer: SoundPlayer;
  romSelector: RomSelector;

  constructor() {
    this.inner = new emulator.Emulator();
    this.keypad = new Keypad(
      (key: number) => this.inner.set_key_down(key),
      (key: number) => this.inner.set_key_up(key)
    );
    this.display = new Display();
    this.soundPlayer = new SoundPlayer();
    this.romSelector = new RomSelector();
  }

  async initialize() {
    await this.romSelector.initialize(this.startEmulator.bind(this), this.stopEmulator.bind(this));
  }

  startEmulator(rom: Uint8Array) {
    this.inner.reset();
    this.inner.load_rom(rom);

    this.keypad.addListeners();

    window.requestAnimationFrame(this.gameLoop.bind(this));
  }

  stopEmulator() {
    this.keypad.removeListeners();
    this.inner.reset();
  }

  gameLoop() {
    for (let i = 0; i < 10; i++) this.inner.execute_instruction_cycle();
    this.display.render(this.inner.get_display_buffer());
    this.inner.decrement_timers();

    if (this.inner.is_sound_playing()) this.soundPlayer.playTone();
    else this.soundPlayer.stopTone();

    if (this.romSelector.romLoaded) window.requestAnimationFrame(this.gameLoop.bind(this));
    else this.soundPlayer.stopTone();
  }
}

init().then(async () => {
  const emulator = new Emulator();
  await emulator.initialize();
});
