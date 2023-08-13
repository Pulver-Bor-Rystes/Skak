use std::time::Instant;

use crate::communication::server::{SendMessage, UpdateSessionData};

use super::server::Server;
use actix::prelude::*;
use actix_web_actors::ws;
use serde::Serialize;

pub struct Session {
    server_addr: Addr<Server>,
    /// Id bliver givet af serveren
    id: usize,
    hb: Instant,
}

impl Session {
    pub fn new(server_addr: Addr<Server>) -> Self {
        Session {
            server_addr,
            id: 0, // Nul for nu, når en klient forbinder bliver den her værdi overskrevet!
            hb: Instant::now(),
        }
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!(" > A new socket connection was established!");

        let addr = ctx.address();
        self.server_addr
            .send(UpdateSessionData::Connect(addr.clone()))
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => {
                        // Gemmer id som serveren har tildelt os!
                        act.id = res.unwrap();
                    }
                    // noget gik galt med serveren (burde ikke ske)
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!(" > ! A socket conn did stop");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(
        &mut self,
        msg: Result<actix_web_actors::ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
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
                println!(" -- > recv: {}", text);
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

    fn started(&mut self, _ctx: &mut Self::Context) {}

    fn finished(&mut self, _ctx: &mut Self::Context) {}
}

/// En event som når modtages sender en besked direkte til klienten!
#[derive(Message, Debug)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub enum DeployMessage<M: Serialize + std::marker::Send + std::fmt::Debug> {
    IntoJson(M),
    String(String),
}

impl<M> Handler<DeployMessage<M>> for Session
where
    M: Serialize + std::marker::Send + std::fmt::Debug,
{
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: DeployMessage<M>, ctx: &mut Self::Context) -> Self::Result {
        println!("sending msg: {:?} to client browser!", msg);

        // I'm about to actually send this to the client browser!
        let msg = match msg {
            DeployMessage::IntoJson(msg) => {
                serde_json::to_string(&msg).expect("json could not be parsed")
            }
            DeployMessage::String(msg) => msg,
        };

        ctx.text(msg);

        Ok(true)
    }
}
