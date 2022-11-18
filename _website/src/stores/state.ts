import { writable } from "svelte/store";
import type { Writable } from "svelte/store";

export const user_data = writable({
    username: "",
    logged_in: false,
    login_failed: false,
})

type Game = {
    subscribed: string[], // where: [0] = white, [1] = black
    state: string // pgn format
}


export interface FriendsInfo {
    "combined": string[]
    "online": string[]
    "offline": string[]
}


export const friends: Writable<string[]> = writable(["?"]);

export const my_friends: Writable<FriendsInfo> = writable({
    "combined": [],
    "online": [],
    "offline": []
});

export const inv_received: Writable<string[]> = writable([]);
export const inv_send: Writable<string[]> = writable([]);



export const games: Writable<string[]> = writable([]);
export const G_inv_recieved: Writable<string[]> = writable([]);
export const G_inv_send: Writable<string[]> = writable([]);