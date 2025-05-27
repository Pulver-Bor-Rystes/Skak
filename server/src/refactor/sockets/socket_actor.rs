use super::*;


// Requirements
pub struct Request {
    topic: String,
    handler: fn(ctx: &mut SessionContext) -> Result<(), JsonError>,
    requires: Vec<Requirement>,
}

#[derive(Debug)]
pub enum Requirement {
    InGame,
    LoggedIn,
    NotLoggedIn,
}

use socket_api::SocketSessionAPI;
use Requirement::*;


impl Requirement {
    pub fn meet_demands(&self, ctx: &SessionContext) -> bool {
        match self {
            Self::InGame => ctx.session.game_addr.is_some(),
            Self::LoggedIn => ctx.is_logged_in(),
            Self::NotLoggedIn => !ctx.is_logged_in(),
        }
    }
}



impl SocketSession {
    pub fn new(server_addr: Addr<Server>) -> Self {
        SocketSession {
            server_addr,
            game_addr: None,
            id: 0, // Nul for nu, når en klient forbinder bliver den her værdi overskrevet!
            username: None,
            hb: Instant::now(),
        }
    }
}





impl Actor for SocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!(" > A new socket connection was established!");

        let addr = ctx.address();
        self.server_addr
            .send(server::UpdateSessionData::Connect(addr.clone()))
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
            .do_send(server::UpdateSessionData::Disconnect(self.id));
    }
}









impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SocketSession {
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

                let mut session_context = SessionContext {
                    topic: topic.clone(),
                    msg: text.to_string(),
                    session: self,
                    socket: ctx,
                };


                let mut is_handled = false;




                // en række funktioner som kan håndterer en request!
                let requests: Vec<Request> = [
                    Request { topic: "test".into(), handler: socket_endpoint::test, requires: [].into() },
                    Request { topic: "login".into(), handler: socket_endpoint::login, requires: [NotLoggedIn].into() },
                    Request { topic: "signup".into(), handler: socket_endpoint::signup, requires: [NotLoggedIn].into() },
                    Request { topic: "getstate".into(), handler: socket_endpoint::getstate, requires: [LoggedIn].into() },
                    Request { topic: "newgame".into(), handler: socket_endpoint::newgame, requires: [LoggedIn].into() },
                    Request { topic: "getbots".into(), handler: socket_endpoint::getbots, requires: [LoggedIn].into() },
                    Request { topic: "play_move".into(), handler: socket_endpoint::play_move, requires: [InGame].into() },
                ].into();


                for handle in requests {
                    if handle.topic != topic { continue }
                    is_handled = true;

                    // checking whether requirements are met and sending back relevant info to the user
                    let mut req_not_met = Vec::new();
                    for req in handle.requires {
                        if !req.meet_demands(&session_context) {
                            req_not_met.push(req);
                        }
                    }

                    if !req_not_met.is_empty() {
                        session_context.socket.text(OutgoingWsMsg::error(topic.clone(), format!("requirements not met: {:?}", req_not_met)).serialize());
                        break;
                    }

                    // Sending the error to the user for easier debugging
                    match (handle.handler)(&mut session_context) {
                        Ok(_) => {}
                        Err(err) => {
                            session_context.socket
                                .text(OutgoingWsMsg::error(topic.clone(), format!("err: {}", err)).serialize());
                        }
                    }
                }

                if !is_handled {
                    println!("message was not handled: {:?}", text);
                    session_context.socket
                        .text(OutgoingWsMsg::error(topic, "was not handled").serialize());
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

    fn started(&mut self, _ctx: &mut Self::Context) {}

    fn finished(&mut self, _ctx: &mut Self::Context) {}
}











impl<M> Handler<SocketAPI<M>> for SocketSession
where
    M: Serialize + std::marker::Send,
{
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: SocketAPI<M>, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            SocketAPI::SendDirectMessage(msg) => {
                // I'm about to actually send this to the client browser!
                let msg: String = serde_json::to_string(&msg).expect("json could not be parsed");
                ctx.text(msg);
            }
        }

        Ok(true)
    }
}


impl Handler<SocketSessionAPI> for SocketSession {
    type Result = bool;

    fn handle(&mut self, msg: SocketSessionAPI, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            SocketSessionAPI::UpdateGameAddr(addr) => self.game_addr = Some(addr),
            SocketSessionAPI::RemoveGameAddr => self.game_addr = None,
        }

        true
    }
}
