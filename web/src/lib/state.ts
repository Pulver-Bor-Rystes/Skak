import { writable, type Writable } from "svelte/store";
import { browser } from "$app/environment";
import init from "chess_machine_lib";

// user details
export const logged_in = writable(false); // true, hvis spilleren er logget ind.
export const username = writable(""); // holder styr på brugernavnet
export const cookie = writable(""); // vores cookie
export const rating: Writable<undefined | number> = writable(undefined);

// game details
const chessboard_did_init = writable(false);
export const games: Writable<Game[]> = writable([]);

export type Game = {
    hash: string;
    white: string;
    black: string;
    current_fen: string;
    move_history: string[];
};

// lists of data
export const active_players = writable([]);
export const friends: Writable<{ name: string; rating: number }[]> = writable([]);

export const friend_invites: Writable<{ hash: number; from: string; to: string }[]> = writable([]);
export const game_invites: Writable<{ hash: number; from: string; to: string }[]> = writable([]);

export const engines: Writable<{ name: string; rating: number }[]> = writable([]);

// ---------------------------------------- //
// -------------- Reactivity -------------- //
// ---------------------------------------- //
cookie.subscribe((c) => {
    if (!browser) return;
    if (c == "" || c === undefined) return;
    localStorage.setItem("Cookie", c);
});

// fordi vores wasm rust magi, har brug for at blive initialiseret, men ikke tusindgange.
// bruger vi den her funktion, som sørger for at det kun sker en gang!
export const chessboard_oneshot_init = (): Promise<void> => {
    return new Promise((resolve) => {
        chessboard_did_init.subscribe(async (value) => {
            if (value) {
                resolve();
            } else {
                await init();
                chessboard_did_init.set(true);
            }
        });
    });
};
