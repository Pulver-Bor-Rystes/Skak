import { writable } from "svelte/store";
import type { Writable } from "svelte/store";
import io from 'socket.io-client'
// const socket = io("http://localhost:3000")

export const socket = writable(io())
export const logged_in = writable(false)