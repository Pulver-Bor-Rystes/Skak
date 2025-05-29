import { writable, type Writable } from "svelte/store";
import { browser } from "$app/environment";

// enable to be sure that nothing goes wrong!
const DEBUG_MSG = false;


type Message = {
    topic: string,
    payload: Payload,
}


type Payload = {
    result: boolean,
    content: any,
}


class Socket {
    private ws: WebSocket | null;
    open: boolean = false;
    private listeners: Map<string, (payload: any) => void> = new Map();
    private queue: string[] = [];

    private reopen_attempts = 0;

    constructor() {
        this.listeners = new Map();
        this.ws = null;

        this.new_socket();
    }


    private new_socket() {
        this.reopen_attempts += 1;
        if (!browser) {
            this.ws = null
            return;
        }
        const proto = window.location.protocol.startsWith("https") ? "wss" : "ws";
        const wsUri = `${proto}://${window.location.host}/api/ws`;

        let socket = new WebSocket(wsUri);

        socket.onopen = () => {
            this.reopen_attempts = 0;
            this.open = true;
        };

        socket.onmessage = (evt) => {
            let msg = this.extract_msg(evt)

            let handled = false;

            // console.log("message received:", msg);

            // make an error if topic contains "no topic"
            if (msg.topic == "no topic") {
                console.error("server responded with topic: 'no topic' - Check your request\n", msg.payload)
                handled = true
            }

            
            // iterate over listeners
            this.listeners.forEach((cb, topic) => {
                if (topic === msg.topic) {
                    cb(msg.payload);
                    handled = true;
                }
            })

            if (!handled) {
                console.error("unhandled event:", msg);
            }
        }

        socket.onclose = () => {
            this.open = false;
            console.error("Socket closed");
        };

        this.ws = socket;
    }


    send(topic: string, content: any) {
        this.queue.push(JSON.stringify({
            topic,
            content,
        }));

        this.send_queue();
    }

    private send_queue(count = 0) {
        if (this.open) {
            while (this.queue.length > 0) {
                this.ws?.send(this.queue.pop()!);
            }
        }
        else {
            if (count > 100) {
                console.error("Socket is still not open");
                if (this.reopen_attempts < 1) {
                    this.new_socket();
                }
                return;
            }

            setTimeout(() => this.send_queue(count + 1), 10);
        }
    }

    on(topic: string, cb: (payload: Payload) => void) {
        if (this.listeners.has(topic) && DEBUG_MSG) {
            console.warn("Replacing listener for topic:", topic);
        }
        this.listeners.set(topic, cb);
    }



    private extract_msg(evt: MessageEvent<any>) {
        let msg: {
            topic: string,
            payload: any
        };

        try {
            msg = JSON.parse(evt.data);                
        }
        catch (e) {
            msg = {
                topic: "",
                payload: evt.data
            };
        }

        return msg
    }
}


export const socket = writable(new Socket);
