use serde_json::Error as JsonErr;
use std::time::Instant;

use crate::{
    actors::server::UpdateSessionData,
    std_format_msgs::{IncomingWsMsg, OutgoingWsMsg},
    user_api,
};

use super::server::Server;
use actix::prelude::*;
use actix_web_actors::ws::{self, WebsocketContext};
use serde::Serialize;

#[derive(Debug)]
pub struct Session {
    pub server_addr: Addr<Server>,
    /// Id bliver givet af serveren
    pub id: usize,
    pub username: Option<String>,
    hb: Instant,
}

impl Session {
    pub fn new(server_addr: Addr<Server>) -> Self {
        Session {
            server_addr,
            id: 0, // Nul for nu, når en klient forbinder bliver den her værdi overskrevet!
            username: None,
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
        // websocket forbindelsen stoppede
        self.server_addr
            .do_send(UpdateSessionData::Disconnect(self.id));
    }
}

/// En context så når en api endpoint skal svare tilbage, så kan de bede serveren om at gøre noget eller de
pub struct SessionContext<'a> {
    pub topic: String,
    pub msg: String,
    pub session: &'a mut Session,
    // pub srv: Addr<Server>,
    pub socket: &'a mut WebsocketContext<Session>,
    // pub client_id: usize,
    // pub client_username: String,
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
                let parsed: Result<IncomingWsMsg, JsonErr> = serde_json::from_str(&text);

                // vi vil gerne kende topic, så resten af vores api hurtigt kan
                // finde ud af om de skal håndterer beskeden!
                let topic = match parsed {
                    Ok(parsed) => parsed.topic,
                    Err(_) => "no topic".to_string(),
                };

                // let mut ac = SessionContext {
                //     topic,
                //     msg: text.to_string(),
                //     srv: self.server_addr.clone(),
                //     client: ctx,
                //     is_handled: false,
                //     client_id: self.id.clone(),
                //     client_username: self.ctx.clclie,
                // };

                let mut ac = SessionContext {
                    topic,
                    msg: text.to_string(),
                    session: self,
                    socket: ctx,
                };
                let mut is_handled = false;

                // en række funktioner som kan håndterer en request!
                let handlers = vec![user_api::interface::handle];

                // så snart en request er blevet håndteret bliver den ikke sendt videre!
                for handle_fn in handlers {
                    if is_handled {
                        break;
                    }
                    match handle_fn(&mut ac) {
                        Some(_) => is_handled = true,
                        None => {}
                    }
                }

                if !is_handled {
                    println!("message was not handled: {:?}", text);
                    ac.socket
                        .text(OutgoingWsMsg::error(ac.topic, "was not handled").serialize());
                }

                // user_api::interface::handle(&mut ac);
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
}

impl<M> Handler<DeployMessage<M>> for Session
where
    M: Serialize + std::marker::Send + std::fmt::Debug,
{
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: DeployMessage<M>, ctx: &mut Self::Context) -> Self::Result {
        // I'm about to actually send this to the client browser!
        let msg = match msg {
            DeployMessage::IntoJson(msg) => {
                serde_json::to_string(&msg).expect("json could not be parsed")
            }
        };

        ctx.text(msg);

        Ok(true)
    }
}
