export class RomSelector {
  romSelect: HTMLSelectElement;
  loadRomButton: HTMLButtonElement;
  romName: string | null;
  romLoaded: boolean;

  static cache: { [id: string]: Uint8Array } = {};

  constructor() {
    this.romSelect = document.querySelector("#rom-select") as HTMLSelectElement;
    this.loadRomButton = document.querySelector("#load-rom-btn") as HTMLButtonElement;
    this.romName = null;
    this.romLoaded = false;
  }

  async initialize(startEmulatorCallback: (rom: Uint8Array) => void, stopEmulatorCallback: () => void) {
    this.romSelect.addEventListener("change", async (event) => {
      this.romName = (event.target as HTMLSelectElement).value || null;
      this.loadRomButton.disabled = this.romName === null;
    });

    this.loadRomButton.addEventListener("click", async () => {
      this.loadRomButton.disabled = true;

      if (this.romLoaded) await this.ejectRom(stopEmulatorCallback);
      else await this.loadRom(startEmulatorCallback);

      this.loadRomButton.disabled = false;
    });

    this.romSelect.disabled = false;
  }

  async loadRom(startEmulatorCallback) {
    if (this.romName === null) return;
    if (this.romLoaded) return;

    this.romSelect.disabled = true;
    this.romLoaded = true;

    const rom = await this.fetchRom();

    this.loadRomButton.innerText = "Eject ROM";
    this.loadRomButton.classList.add("eject");

    startEmulatorCallback(rom);
  }

  async ejectRom(stopEmulatorCallback) {
    if (!this.romLoaded) return;

    this.romLoaded = false;

    this.loadRomButton.innerText = "Load ROM";
    this.loadRomButton.classList.remove("eject");
    this.romSelect.disabled = false;

    stopEmulatorCallback();
  }

  async fetchRom() {
    if (!this.romName) return;

    if (this.romName in RomSelector.cache) return RomSelector.cache[this.romName];

    const response = await fetch(`roms/${this.romName}`);
    const arrayBuffer = await response.arrayBuffer();
    const rom = new Uint8Array(arrayBuffer);

    RomSelector.cache[this.romName] = rom;

    return rom;
  }
}
