use actix::prelude::*;
// use std::process::{Command, Stdio}

use crate::{actors::server::SendMessage, std_format_msgs::OutgoingWsMsg};

use super::server::{Cleanup, Server};

pub struct Game {
    id: usize,

    srv: Addr<Server>,
    white: String,
    black: String,

    fen: String,
}

impl Game {
    pub fn new(id: usize, srv: Addr<Server>, p1: String, p2: String) -> Game {
        Game {
            id,
            srv,
            white: p1,
            black: p2,
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        }
    }

    fn _is_move_valid() {
        // let output = Command::new("echo")
        //     .arg("Hello world")
        //     .stdout(Stdio::piped())
        //     .spawn()
        //     .expect("process did not start");

        // let echo_out = ""
    }
}

impl Actor for Game {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("GAME: started");
        let white = self.white.clone();
        let black = self.black.clone();
        let fen = self.fen.clone();

        self.srv.do_send(SendMessage::To(
            vec![white, black],
            OutgoingWsMsg::content("state", fen),
        ));
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.srv.do_send(Cleanup::Game(self.id))
    }
}

#[derive(Message)]
#[rtype(result = "bool")]
pub enum API {
    GetState(String), // Username
}

impl Handler<API> for Game {
    type Result = bool;

    fn handle(&mut self, msg: API, _: &mut Self::Context) -> Self::Result {
        match msg {
            API::GetState(username) => {
                let fen = self.fen.clone();
                self.srv.do_send(SendMessage::To(
                    vec![username],
                    OutgoingWsMsg::content("state", fen),
                ));
            }
        }

        true
    }
}
