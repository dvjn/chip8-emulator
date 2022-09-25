import init, * as cpu from "chip8-emulator-wasm";

import { SoundPlayer } from "./sound";
import { Keypad } from "./keypad";
import { Display } from "./display";
import { RomSelector } from "./rom";

class Emulator {
  keypad: Keypad;
  display: Display;
  soundPlayer: SoundPlayer;
  romSelector: RomSelector;

  constructor() {
    this.keypad = new Keypad();
    this.display = new Display();
    this.soundPlayer = new SoundPlayer();
    this.romSelector = new RomSelector();
  }

  async initialize() {
    await this.romSelector.initialize(this.startEmulator.bind(this), this.stopEmulator.bind(this));
  }

  startEmulator(rom: Uint8Array) {
    cpu.reset();
    cpu.load_rom(rom);

    this.keypad.addListeners();

    window.requestAnimationFrame(this.gameLoop.bind(this));
  }

  stopEmulator() {
    this.keypad.removeListeners();
    cpu.reset();
  }

  gameLoop() {
    for (let i = 0; i < 10; i++) cpu.execute_instruction_cycle();
    this.display.render(cpu.get_display_buffer());
    cpu.decrement_timers();

    if (cpu.is_sound_playing()) this.soundPlayer.playTone();
    else this.soundPlayer.stopTone();

    if (this.romSelector.romLoaded) window.requestAnimationFrame(this.gameLoop.bind(this));
    else this.soundPlayer.stopTone();
  }
}

init().then(async () => {
  const emulator = new Emulator();
  await emulator.initialize();
});
