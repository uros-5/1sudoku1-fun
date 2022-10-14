import { defineStore } from "pinia";
import { clientGame, sudokuStore, userStore } from "../storeTypes";
import { keyboardMap } from "@/plugins/keyboardMap";
import { generateSudokuItems } from "@/plugins/sudokuItems";
import router from "@/router";
import { SEND } from "@/plugins/webSockets";
import startSound from "@/assets/sounds/start.ogg";
import lowTime from "@/assets/sounds/low_time.ogg";
import { Clock } from "@/plugins/clock";

const DIRECTIONS = { 11: -1, 12: -9, 13: 1, 14: 9, 15: 0 };
const FINISHED = [0, 1, 2, 4, 5];
type direction = 11 | 12 | 13 | 14;

export type myStore = sudokuStore & userStore & clientGame;

export const useSudokuStore = defineStore("useSudokuStore", {
  state: (): myStore => {
    return emptyStore();
  },
  actions: {
    // USER STORE 

    // set new username
    setUsername(username: string) {
      this.$state.username = username;
    },

    // set current game count
    setGameCount(cnt: number) {
      this.$state.gameCount = cnt;
    },

    // set my new request id
    setNewRequest(id: string) {
      this.$state.requestId = id;
    },

    // SUDOKU STORE

    // set game from ws
    setGame(store: sudokuStore & { _id: string }) {
      this.$state.game = store.game;
      let msg = { t: "live_game_line", game_id: store._id };
      SEND(msg);
      this.setClock();
    },

    // set clock
    setClock() {
      let now = new Date();
      let self = this;
      let elapsed = now.getTime() - new Date(this.$state.game.clock.last_click).getTime();
      let clock = this.$state.game.clock.clock - elapsed;
      this.$state.clock.setTime(clock);
      this.$state.clock.onTick(this.$state.clock.renderTime);
      this.$state.clock.onHurry(() => {
        self.$state.hurry = true;
        new Audio(lowTime).play();
      });

      this.$state.clock.start()
    },



    // delete all fields
    deleteAll() {
      let started = this.$state.game.started_with;
      this.$state.myLine = String(started);
      SEND({ t: "delete_all", game_id: this.$state.game._id, game_move: "x" });
    },

    // resign
    resign() {
      SEND({ t: "resign", game_id: this.$state.game._id, game_move: "resigned" });
      this.$state.hurry = false;
    },

    // set my line
    setLine(line: string) {
      this.$state.myLine = line;
    },

    // redirect game request to new game
    redirectTo(id: string) {
      this.startAudio();
      router.push(`/game/${id}`);
      SEND({ t: "live_game", game_id: id });
    },

    // update one field
    update(index: number, newValue: number) {
      if (this.canIUpdate(index)) {
        if (newValue == 0 || newValue < 0) {
          this.$state.myLine = replaceAt(this.$state.myLine, index, ".");
          SEND({ t: "delete_one", game_id: this.$state.game._id, game_move: `${index}_x` });
        } else {
          this.$state.myLine = replaceAt(
            this.$state.myLine,
            index,
            newValue.toString()
          );
          SEND({ t: "make_move", game_id: this.$state.game._id, game_move: `${index}_${newValue}` });
        }
      }
    },

    // check if i can update
    canIUpdate(index: number): boolean {
      if (index == -1) {
        return false;
      }
      else if (this.$state.game.started_with.at(index) == ".") {
        return true;
      } else {
        return false;
      }
    },

    // resign game
    setResigned(player: number, score: [number, number]) {
      this.$state.game.score = score;
      this.$state.game.status = player + 4;
      this.clock.pause(false);
      this.$state.hurry = false;
      this.startAudio();
      this.goToHome();
    },

    // set winner
    setWinner(player: number, score: [number, number]) {
      this.$state.game.score = score;
      this.$state.game.status = player;
      this.clock.pause(false);
      this.$state.hurry = false;
      this.startAudio();
      this.goToHome();
    },

    goToHome() {
      let self = this;
      setTimeout(() => {
        self.$state.game.status = 3;
        let sudokuItems = self.$state.sudokuItems;
        let username = String(self.$state.username);
        self.$reset();
        self.setUsername(username);
        self.$state.sudokuItems = sudokuItems;
        router.push('/');
      }, 3000);
    },

    setId(url: string) {
      this.$state.game._id = url;
    },

    finishGame(score: [number, number]) {
      this.$state.game.score = score;
      if (score[0] == score[1]) {
        this.$state.game.status = 2
      }
      else if (score[0] > score[1]) {
        this.$state.game.status = 0;
      }
      else if (score[1] > score[0]) {
        this.$state.game.status = 1;
      }
      this.startAudio();
      this.goToHome();
    },

    myIndex(): number {
      let players = this.$state.game.players;
      if (players[0] == this.$state.username) {
        return 0;
      }
      return 1;
    },

    isHurry(): boolean {
      return this.$state.hurry;
    },

    gameFinished(): boolean {
      if (FINISHED.includes(this.$state.game.status)) {
        return true;
      }
      return false;
    },

    defaultLine(): string {
      return this.$state.game.started_with;
    },

    id(): string {
      return this.$state.game._id;
    },

    async setSudokuItems() {
      this.$state.sudokuItems = await generateSudokuItems();
    },

    setKeyboard() {
      this.$state.keyboardMap = keyboardMap();
    },

    setMyNumber(n: number) {
      this.$state.myNumber = n;
      this.update(this.$state.selected, this.$state.myNumber);
      this.$state.myNumber = -1;
    },

    deselect() {
      this.setMyNumber(0);
      this.$state.selected = -1;
      this.otherItems = [];
    },

    erase() {
      this.update(this.$state.selected, 0);
    },

    buttonActions() {
      return [this.erase, this.resign, this.deselect, this.deleteAll];
    },

    click(event: Event) {
      let target = event.target as HTMLDivElement;
      let field = target.attributes.getNamedItem("data-field");
      if (field) {
        this.select(field.value);
        if (this.$state.myNumber == -1) { return; }
        else if (this.$state.myNumber || this.$state.myNumber == 0) {
          this.update(parseInt(field.value), this.$state.myNumber);
          this.$state.myNumber = -1;
        }
      }
    },

    select(index: string) {
      let value = parseInt(index);
      this.$state.selected = value;
      let block = this.fieldBlock(value);
      let vertical = this.fieldVertical(value);
      let horizontal = this.fieldHorizontal(value);
      this.$state.otherItems = [...vertical, ...horizontal, ...block];
    },

    fieldBlock(index: number): number[] {
      let block = this.$state.sudokuItems.at(index)?.block;
      let items: number[] = [];
      this.$state.sudokuItems.forEach((value, index2) => {
        if (value.block == block && index2 != index) {
          items.push(index2);
        }
      });
      return items;
    },

    fieldVertical(index: number): number[] {
      let vertical = this.$state.sudokuItems.at(index)?.vertical;
      let items: number[] = [];
      this.$state.sudokuItems.forEach((value, index2) => {
        if (value.vertical == vertical && index2 != index) {
          items.push(index2);
        }
      });
      return items;
    },

    fieldHorizontal(index: number) {
      let itemValue = this.$state.sudokuItems.at(index)?.horizontal;
      let items: number[] = [];
      this.$state.sudokuItems.forEach((value, index2) => {
        if (value.horizontal == itemValue && index2 != index) {
          items.push(index2);
        }
      });
      return items;
    },

    fieldNumbers(index: number): number[] {
      let itemValue = this.$state.sudokuItems.at(index)?.value;
      let items: number[] = [];
      this.$state.sudokuItems.forEach((value, index2) => {
        if (value.value == itemValue && index2 != index) {
          items.push(index2);
        }
      });
      return items;
    },

    startAudio() {
      new Audio(startSound).play()
    },

    keyDown(event: KeyboardEvent) {
      const someNums = [11, 12, 13, 14, 15];
      let item = this.$state.keyboardMap.get(event.key);
      if (item || item == 0) {
        if (!someNums.includes(item)) {
          this.$state.myNumber = item;
          if (this.$state.selected > -1 || this.$state.myNumber == 0) {
            this.update(this.$state.selected, item);
            this.$state.myNumber = -1;
          }
        } else {
          this.$state.myNumber = -1;
          this.moveSelected(parseInt(String(item)));
        }
      }
    },

    contextMenu(event: Event) {
      let target = event.target as HTMLDivElement;
      let field = target.attributes.getNamedItem("data-field");
      if (field) {
        this.update(parseInt(field.value), 0);
      }
    },

    moveSelected(dir: number) {
      if (this.$state.selected == -1) {
        return;
      } else if (dir == 15) {
        return;
      }
      let calc = (this.$state.selected +
        DIRECTIONS[dir as direction]) as number;
      if (calc >= 0 && calc < 81) {
        this.select(String(calc));
      }
    },

    myNumber(): number {
      return this.$state.myNumber;
    },

    myLineAt(i: number): string {
      return this.$state.myLine.at(i) as string;
    },


    // STYLE PART

    bottomBorder(index: number): boolean {
      if (index >= 18 && index <= 18 + 8) {
        return true;
      } else if (index >= 45 && index <= 45 + 8) {
        return true;
      }
      return false;
    },

    currentOpacity(index: number): string {
      let item = this.$state.otherItems.find((item) => item == index);
      let opacity = 1;
      let bg = "none";
      if (item || item == 0) {
        opacity = 0.7;
        bg = "rgba(172, 201, 178, 0.4)";
      }
      return `--currentOpacity: ${opacity}; background-color: ${bg};`;
    },

    isEmpty(index: number): boolean {
      if (this.$state.myLine.at(index) == ".") {
        return true;
      } else {
        return false;
      }
    },

    gameCount(): number {
      return this.$state.gameCount;
    }
  },
});

function emptyStore(): myStore {
  let userStore = emptyUserStore();
  let sudokuStore = emptyGame();
  let clientGame = emptyClientGame();
  return { ...userStore, game: sudokuStore, ...clientGame }
}

function emptyGame(): any {
  return {
    _id: "",
    date_created: 0,
    min: 0,
    players: ["", ""],
    result: [0, 0],
    score: [0, 0],
    started_with: "",
    status: 3,
    clock: { clock: 0, last_click: "" }
  }
}

function emptyUserStore(): userStore {
  return {
    username: "",
    gameCount: 0,
    serverOnline: false,
    theme: "dark",
  }
}

function emptyClientGame(): clientGame {
  return {
    myLine: ".................................................................................",
    sudokuItems: [],
    selected: -1,
    otherItems: [],
    myNumber: -1,
    keyboardMap: new Map(),
    clock: new Clock(0, 0, 0, '0'),
    hurry: false
  }
}

function replaceAt(
  myString: string,
  index: number,
  replacement: string
): string {
  return (
    myString.substring(0, index) +
    replacement +
    myString.substring(index + replacement.length)
  );
}
