import { useBackgroundSvgStore } from "@/store/backgroundSvg";
import { useRequestStore } from "@/store/createGameStore";
import Sockette from "sockette";
import { useSudokuStore } from "../store/sudokuStore";
import { backend, wsUrl } from "./getBackend";

export const ws = new Sockette(wsUrl(), {
  timeout: 1200,
  maxAttempts: 15,
  onopen: (e) => {
    onopen(e);
  },
  onmessage: (e) => {
    onmessage(e);
  },
  onreconnect: (e) => {
    onreconnect(e);
  },
  onmaximum: (e) => {},
  onclose: (e) => {},
  onerror: (e) => {},
});

let unsendMessages: any[] = [];

export function SEND(msg: any) {
  try {
    ws.send(JSON.stringify(msg));
  } catch (error) {
    unsendMessages.push(msg);
  }
}

function onopen(e: any) {
  SEND({ t: "username" });
  unsendMessages.forEach((v) => {
    SEND(v);
  })
  unsendMessages = [];
}

function onmessage(e: any) {
  const msg = JSON.parse(e.data);

  const sudokuStore = useSudokuStore();
  const requestStore = useRequestStore();
  const bgStore = useBackgroundSvgStore();


  switch (msg.t) {
    case "games_count":
      sudokuStore.setGameCount(msg.cnt);
      break;
    case "live_game_created":
      requestStore.setId(msg.game_id);
      sudokuStore.setId("");
      sudokuStore.setNewRequest(msg.game_id);
      break;
    case "live_game_accepted":
      sudokuStore.redirectTo(msg.game_id);
      break;
    case "live_game_resigned":
      sudokuStore.setResigned(msg.player,msg.score);
      break;
    case "live_game":
      sudokuStore.setGame(msg.game);
      break;
    case "live_game_line":
      sudokuStore.setLine(msg.line);
      break;
    case "live_game_winner":
      break;
    case "username":
      sudokuStore.setUsername(msg.username);
      break;
    case "request_url":
      requestStore.setId(msg.url);
      sudokuStore.setId("");
      break;
    case "game_url":
      sudokuStore.setId(msg.url);
      requestStore.setId("");
      break;
    case "game_finished":
      sudokuStore.finishGame(msg.score);
      sudokuStore.setId("");
      break; 
  }
}
function onreconnect(e: any) {}
