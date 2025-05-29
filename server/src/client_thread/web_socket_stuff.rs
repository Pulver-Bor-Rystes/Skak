use std::{future::Future, time::Instant};
use actix::prelude::*;
use actix_web_actors::ws::{self, WebsocketContext};
use crate::std_format_msgs::{IncomingWsMsg, OutgoingWsMsg};
use super::ClientThread;


impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientThread {
    fn handle(&mut self, msg: Result<actix_web_actors::ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let cloned_text = text.clone();
                let parsed: Result<IncomingWsMsg, serde_json::Error> = serde_json::from_str(&text);

                if let Ok(parsed) = parsed {
                    if !self.client_endpoint(text.into(), parsed, ctx) {
                        println!("[ERR 711]: Request did not get handled\n{:?} 🤷‍♀️", cloned_text);
                        ctx.text(OutgoingWsMsg::error("request not handled", cloned_text.to_string()).serialize());
                    }
                }
                else {
                    let error_msg = format!("[ERR 712]: Could not parse\n{:?}", parsed);
                    println!("{}", error_msg);
                    ctx.text(OutgoingWsMsg::error("parsing error", error_msg).serialize());
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {}

    fn finished(&mut self, _ctx: &mut Self::Context) {}
}