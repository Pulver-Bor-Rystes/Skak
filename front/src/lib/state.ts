import { socket } from "./ws";
import init from "chess_machine_lib";
import { writable } from "svelte/store";
import { browser } from "$app/environment";


export const logged_in = writable(false);
export const username = writable('not logged in');
export const cookie = writable('');

export const c_did_init = writable(false);
export const board_id = writable(-1);
export const color = writable(false);
export const is_in_game = writable(false);

export const active_players = writable([]);
export const engines = writable([]);
export const rating = writable(undefined);

username.subscribe(u => {
    if (!browser) return;
    if (u == 'not logged in') return;
    localStorage.setItem('Username', u);
});

cookie.subscribe(c => {
    if (!browser) return;
    if (c == '' || c === undefined) return;
    console.log("saving cookie:", c);
    localStorage.setItem('Cookie', c);
});


board_id.subscribe(id => is_in_game.set(id >= 0));


export const smart_init = (): Promise<void> => {
    return new Promise(resolve => {
        c_did_init.subscribe(async value => {
            if (value) {
                resolve();
            }
            else {
                await init();
                c_did_init.set(true);
            }
        });
    });
    
    
}