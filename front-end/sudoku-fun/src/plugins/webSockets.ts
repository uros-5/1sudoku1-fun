import Sockette from "sockette";
import { useSudokuStore } from "../store/sudokuStore";
import { backend, wsUrl } from "./getBackend";

const ws = new Sockette(wsUrl(), {
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
    onmaximum: (e) => { },
    onclose: (e) => { },
    onerror: (e) => { },
});

let unsendMessages: any[] = [];

export function SEND(msg: any) {
    try {
        ws.send(JSON.stringify(msg));
    } catch (error) {
        unsendMessages.push(msg);
    }
}

function onopen(e: any) { }

function onmessage(e: any) {
    const msg = JSON.parse(e.data);

    const sudokuStore = useSudokuStore();

    switch (msg.t) {
        case "games_count":
            break;
        case "live_created_game":
            break
        case "live_game_accepted":
            break;
        case "live_game_resigned": 
            break;
        case "live_game":
            break;
        case "live_game_line":
            break;
        case "live_game_winner":
            break;
        case "username":
            sudokuStore.setUsername(msg.username);
            break;
        case "make_move":
            break;
    }
}

function onreconnect(e: any) { }