use std::time::Instant;
use actix::prelude::*;
use actix_web_actors::ws::{self};
use crate::{info, std_format_msgs::{WsIn, WsOut, WsTopic}};
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
                let parsed: Result<WsTopic, serde_json::Error> = serde_json::from_str(&text);

                if text == "ping" {
                    self.hb = Instant::now();
                    ctx.text(WsOut::with_content("pong", "pong").serialize());
                    return;
                }

                if let Ok(parsed) = parsed {
                    let res = actix_web::rt::spawn(async move {
                        let txt = text.to_string();

                        ClientThread::nclient_endpoint(parsed, data, ctx).await;


                        // self.client_endpoint(parsed, txt2, ctx).await;
                    });
                    // if !self.client_endpoint(text.into(), parsed, ctx) {
                    //     info!("Request did not get handled\n{:?}", cloned_text);
                    //     ctx.text(WsOut::with_error("request not handled", cloned_text.to_string()).serialize());
                    // }
                }
                else {
                    let error_msg = format!("Could not parse\n{:?}", parsed);
                    info!("\nGIANT ERROR!\n{}\n{}", error_msg, cloned_text);
                    ctx.text(WsOut::with_error("parsing error", error_msg).serialize());
                }
            }
            ws::Message::Binary(_) => info!("Unexpected binary"),
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

    fn started(&mut self, ctx: &mut Self::Context) {
        let _ = ctx;
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {}
}
