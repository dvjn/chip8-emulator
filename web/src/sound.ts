export class SoundPlayer {
  isInitialized: boolean;

  ctx: AudioContext;
  oscillator: OscillatorNode;
  isPlaying: boolean;

  constructor() {
    this.isInitialized = false;
  }

  initialize() {
    this.isInitialized = true;
    this.ctx = new AudioContext();
    this.oscillator = this.ctx.createOscillator();
    this.isPlaying = false;

    this.oscillator.frequency.value = 440;
    this.oscillator.start();
  }

  playTone() {
    if (!this.isInitialized) this.initialize();
    if (this.isPlaying) return;

    this.oscillator.connect(this.ctx.destination);
    this.isPlaying = true;
  }

  stopTone() {
    if (!this.isInitialized || !this.isPlaying) return;

    this.oscillator.disconnect(this.ctx.destination);
    this.isPlaying = false;
  }
}
