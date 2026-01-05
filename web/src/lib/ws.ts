import { writable, type Writable } from "svelte/store";
import { browser } from "$app/environment";

// enable to be sure that nothing goes wrong!
const DEBUG_MSG = true;

type Message = {
  topic: string;
  content: any;
};

class Socket {
  private ws: WebSocket | null;
  open: boolean = false;
  private listeners: Map<string, (result: boolean, content: any) => void> =
    new Map();
  private queue: string[] = [];
  private after_login_queue: string[] = [];

  private reopen_attempts = 0;

  constructor() {
    this.listeners = new Map();
    this.ws = null;

    this.new_socket();

    setInterval(() => {
      if (this.open) {
        this.queue.push("ping");
        this.send_queue();
      }
    }, 100);

    this.on("pong", (_) => {});
  }

  private new_socket() {
    this.reopen_attempts += 1;
    if (!browser) {
      this.ws = null;
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
      let msg = this.extract_msg(evt);
      let handled = false;

      // iterate over listeners
      this.listeners.forEach((cb, topic) => {
        if (topic === msg.topic) {
          handled = true;
          try {
            JSON.parse(msg.content);
          } catch (e) {
            console.error(
              "AN ERROR OCCURRED\n" + "TOPIC: " + topic + "\nCONTENT:",
              msg.content + "\n\nERROR:",
              JSON.stringify(e),
            );

            return;
          }

          cb(msg.result, JSON.parse(msg.content));
        }
      });

      if (!handled) {
        console.error("unhandled event:", msg);
      }
    };

    socket.onclose = () => {
      this.open = false;
      console.error("Socket closed");
    };

    this.ws = socket;
  }

  query(api: string, callback_fn: (result: boolean, content: any) => void) {
    let actual_api = "query:" + api;

    this.send_after(actual_api, {});
    this.on(actual_api, callback_fn);
  }

  send(topic: string, content: any) {
    this.queue.push(
      JSON.stringify({
        topic,
        content,
      }),
    );

    this.send_queue();
  }

  send_after(topic: string, content: any) {
    this.after_login_queue.push(
      JSON.stringify({
        topic,
        content,
      }),
    );

    this.send_after_login_queue();
  }

  private send_after_login_queue(count = 0) {
    // Forsøger igen
    if (!this.open) {
      setTimeout(() => this.send_after_login_queue(++count), 50);
      return;
    }

    // pushing to normal queue
    while (this.after_login_queue.length > 0) {
      let new_item = this.after_login_queue.pop();
      this.queue.push(new_item!);
    }

    // sending normal queue
    this.send_queue();
  }

  private send_queue(count = 0) {
    if (this.open) {
      while (this.queue.length > 0) {
        this.ws?.send(this.queue.pop()!);
      }
    } else {
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

  on(topic: string, cb: (result: boolean, content: any) => void) {
    if (this.listeners.has(topic) && DEBUG_MSG) {
      console.warn("Replacing listener for topic:", topic);
    }
    this.listeners.set(topic, cb);
  }

  private extract_msg(evt: MessageEvent<any>) {
    let msg: {
      topic: string;
      result: boolean;
      content: any;
    };

    try {
      msg = JSON.parse(evt.data);
    } catch (e) {
      console.error(e);
      msg = {
        topic: "",
        result: false,
        content: evt.data,
      };
    }

    return msg;
  }
}

export const socket = writable(new Socket());
