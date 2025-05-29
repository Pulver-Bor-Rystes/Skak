import { writable } from "svelte/store";
import { browser } from "$app/environment";
import { socket } from "./ws";


export const logged_in = writable(false);
export const username = writable('not logged in');
export const cookie = writable('');

export const c_did_init = writable(false);
export const board_id = writable(-1);
export const color = writable(false);

export const active_players = writable([]);
export const engines = writable([]);

socket.subscribe(socket => {
    socket.on("active_users", ({ result, content }) => {
        if (!result) {
            console.error("active_players went wrong", content);
            return;
        }
        active_players.set(content)

        socket.send("getbots", {});
    });

    socket.on("engines", ({ result, content }) => {
        if (!result) {
            console.error("engines went wrong", content);
            return;
        }

        console.log(result, content)
        engines.set(content)
    })
});

username.subscribe(u => {
    if (!browser) return;
    if (u == 'not logged in') return;
    localStorage.setItem('Username', u);
});

cookie.subscribe(c => {
    if (!browser) return;
    if (c == '') return;
    localStorage.setItem('Cookie', c);
});