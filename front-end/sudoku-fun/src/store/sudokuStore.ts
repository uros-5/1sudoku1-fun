import { defineStore } from "pinia";
import { sudokuStore, userStore } from "../storeTypes";

export type myStore = sudokuStore & userStore;

export const useSudokuStore = defineStore("useSudokuStore", {
    state: (): myStore => {
        return emptyStore()
    },
    actions: {
        setUsername(username: string) {
            this.$state.username = username;
        }

    }
});

function emptyStore(): myStore {
    return {
        username: "",
        gameCount: 0,
        serverOnline: false,
        theme: "dark",
        game: {
            _id: "",
            date_created: 0,
            min: 0,
            players: ["",""],
            result: [0,0],
            score: [0,0],
            started_with: "",
            status: -1
        }
    }
}