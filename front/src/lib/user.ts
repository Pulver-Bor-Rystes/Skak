import { writable } from "svelte/store";

export const user = writable({
    username: '',
    logged_in: false,
});