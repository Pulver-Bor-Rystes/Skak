import { writable } from "svelte/store";

export const user_data = writable({
    username: "",
    logged_in: false,
    login_failed: false,
})