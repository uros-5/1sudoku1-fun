// https://stackoverflow.com/questions/20618355/the-simplest-possible-javascript-countdown-timer

import { Ref, ref } from "vue";

const HURRY = 10000;

export class Clock {
  duration: number;
  increment: number;
  granularity: number;
  running: boolean;
  connecting: boolean;
  timeout: ReturnType<typeof setTimeout> | null;
  startTime: number;
  tickCallbacks: ((diff: number) => void)[];
  flagCallback: (() => void) | null;
  id: string;
  overtime: boolean;
  hurry: boolean;
  hurryCallback: (() => void);
  ticks: boolean[];
  public currentMin: Ref<string> | string;
  public currentSec: Ref<string> | string;
  

  // game baseTime (min) and increment (sec)
  constructor(
    baseTime: number,
    increment: number,
    byoyomiPeriod: number,
    id: string
  ) {
    this.duration = baseTime * 1000 * 60;
    this.increment = increment * 1000;
    this.granularity = 500;
    this.running = false;
    this.connecting = false;
    this.timeout = null;
    this.startTime = 0;
    this.tickCallbacks = [];
    this.flagCallback = null;
    this.id = id;
    this.overtime = false;
    this.hurry = false;
    this.hurryCallback = function () {},
    this.ticks = [
      false,
      false,
      false,
      false,
      false,
      false,
      false,
      false,
      false,
      false,
    ];
    this.currentMin = ref("");
    this.currentSec = ref("");

    this.renderTime(this.duration);
  }

  restart() {
    
  }

  start(duration = 0) {
    if (this.running) return;
    if (duration !== 0) this.duration = duration;

    this.running = true;
    this.startTime = Date.now();

    const timer = () => {
      const diff = this.duration - (Date.now() - this.startTime);
      if (diff <= HURRY && !this.hurry) {
        this.hurry = true;
        this.hurryCallback();
      }


      if (diff <= 0) {
          if (this.flagCallback !== null) this.flagCallback();
          this.pause(false);
          return;
      }
      this.timeout = setTimeout(timer, this.granularity);
      this.tickCallbacks.forEach(function (callback) {
        // @ts-ignore
        callback.call(this, diff);
      }, this);
    };

    timer();
  }

  onTick(callback: (diff: number) => void) {
    if (typeof callback === "function") {
      this.tickCallbacks.push(callback);
    }
    return this;
  }

  onHurry(callback: () => void) {
    if (typeof callback === "function") {
      this.hurryCallback = callback;
    }
  }

  onFlag(callback: () => void) {
    if (typeof callback === "function") {
      this.pause(false);
      this.flagCallback = callback;
    }
    return this;
  }

  pause(withIncrement: boolean) {
    if (!this.running) return;

    this.running = false;
    if (this.timeout) clearTimeout(this.timeout);
    this.timeout = null;

    this.duration -= Date.now() - this.startTime;
    if (withIncrement && this.increment) {
        this.duration += this.increment;
        this.hurry = this.duration < HURRY;
    }
    this.renderTime(this.duration);
  }

  setTime(millis: number) {
    this.duration = millis;
    this.renderTime(this.duration);
  }

  printTime(millis: number): PrintedTime {
    let minutes = Math.floor(millis / 60000);
    let seconds = (millis % 60000) / 1000;
    let secs, mins;
    if (Math.floor(seconds) === 60) {
      minutes++;
      seconds = 0;
    }
    minutes = Math.max(0, minutes);
    seconds = Math.max(0, seconds);
    if (millis < HURRY) secs = seconds.toFixed(1);
    else secs = Math.floor(seconds).toString();
    mins = (minutes < 10 ? "0" : "") + minutes;
    secs = (seconds < 10 && secs.length < 4 ? "0" : "") + secs;
    return {
      minutes: mins,
      seconds: secs,
    };
  }

  renderTime(time: number) {
    const printed = this.printTime(time);
    this.currentMin = printed.minutes;
    this.currentSec = printed.seconds;
    if (this.granularity > 100 && time < HURRY) this.granularity = 100;
  }
}

export interface PrintedTime {
  minutes: string;
  seconds: string;
}