import { writable, type Writable } from "svelte/store";
import { browser } from "$app/environment";

class Socket {
    ws: WebSocket | null;
    open: boolean = false;
    listeners: Map<string, (data: any) => void> = new Map();
    queue: string[] = [];

    constructor() {
        if (!browser) {
            this.ws = null
            return;
        }
        const proto = window.location.protocol.startsWith("https") ? "wss" : "ws";
        const wsUri = `${proto}://${window.location.host}/api/ws`;

        let socket = new WebSocket(wsUri);

        socket.onopen = () => {
            this.open = true;
        };

        socket.onmessage = (evt) => {
            let data: {
                topic: string,
                data: any
            };

            try {
                data = JSON.parse(evt.data);                
            }
            catch (e) {
                data = {
                    topic: "",
                    data: evt.data
                };
            }

            let handled = false;
            
            // iterate over listeners
            this.listeners.forEach((cb, topic) => {
                if (topic === data.topic) {
                    cb(data.data);
                    handled = true;
                }
            })

            if (!handled) {
                console.error("unhandled event:", data);
            }
        }

        socket.onclose = () => {
            this.open = false;
            console.error("Socket closed");
        };

        this.ws = socket;
        this.listeners = new Map();
    }


    send(topic: string, data: any) {
        this.queue.push(JSON.stringify({
            topic,
            data
        }));

        this.send_queue();
    }

    send_queue(count = 0) {
        if (this.open) {
            while (this.queue.length > 0) {
                this.ws?.send(this.queue.pop()!);
            }
        }
        else {
            if (count > 100) {
                console.error("Socket is still not open");
                return;
            }

            setTimeout(() => this.send_queue(count + 1), 10);
        }
    }

    on(topic: string, cb: (data: any) => void) {
        if (this.listeners.has(topic)) {
            console.error("Listener already exists for topic:", topic);
        }
        this.listeners.set(topic, cb);
    }
}


export const socket = writable(new Socket);
