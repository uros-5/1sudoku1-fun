import { Ref } from "vue";
import { Clock } from "./plugins/clock";
import { SudokuItem } from "./plugins/sudokuItems";

export interface liveCount {
  t: string;
  cnt: number;
}

export interface liveGame {
  t: string;
  game_id: string;
  game_move?: string;
  index?: string;
}

export interface userStore {
  username: string;
  serverOnline: boolean;
  theme: string;
  gameCount: number;
  requestId?: string;
}

export interface sudokuStore {
  game: {
    _id: string;
    score: [number, number];
    min: number;
    date_created: number;
    started_with: string;
    players: [string, string];
    result: [number, number];
    status: number;
    clock: {
      clock: number;
      last_click: string;
    }
  };
}

export interface clientGame {
  myLine: string;
  sudokuItems: Array<SudokuItem>;
  selected: number;
  otherItems: Array<number>;
  myNumber: number;
  keyboardMap: Map<string, number>;
  clock: Clock;
  hurry: boolean;
}

export interface createGameStore {
  modalActive: boolean;
  minute: Minute;
  main: Ref<null | HTMLElement>;
  id: string;
}

export type Minute = 1 | 2 | 3;
