use actix::prelude::*;
use actix_web_actors::ws::{self, WebsocketContext};
use serde::Serialize;
use std::time::{Duration, Instant};

use crate::com::{Failure, Success, WSMessage};
use crate::user_api;
use crate::user_api::types::LoadedUser;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct SocketContext<'a> {
    // meta data
    pub topic: String,
    msg: String, // ikke public, da beskeden skal parses og ikke ændres!
    ctx: &'a mut WebsocketContext<Socket>,
    socket: &'a mut Socket,
}

impl<'a> SocketContext<'a> {
    pub fn new(socket: &'a mut Socket, ctx: &'a mut WebsocketContext<Socket>, msg: &str) -> Self {
        let parsed: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&msg);

        let mut topic = "no topic".to_string();

        if let Ok(parsed) = parsed {
            if let Some(parsed_topic) = parsed["topic"].as_str() {
                topic = parsed_topic.to_owned();
            }
        }

        Self {
            topic,
            msg: msg.to_string(),
            ctx,
            socket,
        }
    }

    pub fn ok<T: Serialize>(&mut self, data: T) {
        let msg = serde_json::to_string(&WSMessage {
            topic: self.topic.to_owned(),
            data: Success::new(data),
        });

        self.send(msg);
    }
    pub fn error<T: Serialize>(&mut self, data: T) {
        let msg = serde_json::to_string(&WSMessage {
            topic: self.topic.to_owned(),
            data: Failure::new(data),
        });

        self.send(msg);
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn send(&mut self, msg: Result<String, serde_json::Error>) {
        self.ctx.text(msg.unwrap());
    }
}

/// Selve socket forbindelsen til en klient
pub struct Socket {
    hb: Instant,
    user: Option<LoadedUser>,
}

impl Socket {
    pub fn new() -> Self {
        Self {
            hb: Instant::now(),
            user: None,
        }
    }

    fn is_logged_in(&self) -> bool {
        self.user.is_some()
    }

    // This function will run on an interval, every 5 seconds to check
    // that the connection is still alive. If it's been more than
    // 10 seconds since the last ping, we'll close the connection.
    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for Socket {
    type Context = ws::WebsocketContext<Self>;

    // Start the heartbeat process for this connection
    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
    }
}

// The `StreamHandler` trait is used to handle the messages that are sent over the socket.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Socket {
    // The `handle()` function is where we'll determine the response
    // to the client's messages. So, for example, if we ping the client,
    // it should respond with a pong. These two messages are necessary
    // for the `hb()` function to maintain the connection status.
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // Ping/Pong will be used to make sure the connection is still alive
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            // Text will echo any text received back to the client (for now)
            Ok(ws::Message::Text(msg)) => {
                let mut ctx = SocketContext::new(self, ctx, &msg.to_string());

                user_api::interface::handle(&mut ctx);

                // if !c.sent() {
                //     c.send(WSMessage::something_went_wrong(msg))
                // }
                // ctx.text("{\"topic\": \"page\", \"data\": \"Hej Rasmus\"}")
            }
            // Close will close the socket
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
