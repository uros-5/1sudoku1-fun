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
}

export interface sudokuStore {
    game: {
        _id: string;
        score: [number,number];
        min: number;
        date_created: number;
        started_with: string;
        players: [string, string];
        result: [number, number];
        status: number
    }

}
