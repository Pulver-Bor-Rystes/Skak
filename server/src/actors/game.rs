use crate::{
    actors::server::{self, Server},
    std_format_msgs::OutgoingWsMsg,
};
use actix::prelude::*;
use std::time::Duration;

use super::server::game_api::GameAPI;

pub struct Game {
    id: usize,

    srv: Addr<Server>,
    white: String,
    black: String,

    turn: bool,

    time: TimeFormat,

    fen: String,
}

#[derive(Clone)]
pub struct TimeFormat {
    name: String,
    description: String,
    initial: Duration,
    increment: Duration,
}

impl TimeFormat {
    pub fn formats() -> Vec<TimeFormat> {
        vec![
            TimeFormat {
                name: "M5s3".to_string(),
                description: "5min game with a 3sec increment each turn".to_string(),
                initial: Duration::from_secs(5 * 60),
                increment: Duration::from_secs(5),
            },
            TimeFormat {
                name: "M10".to_string(),
                description: "10min game".to_string(),
                initial: Duration::from_secs(10 * 60),
                increment: Duration::from_secs(0),
            },
        ]
    }

    pub fn from(_timeformat: &str) -> TimeFormat {
        TimeFormat::default()
    }

    pub fn default() -> TimeFormat {
        // TimeFormat::formats()[0]
        TimeFormat {
            name: "M5s3".to_string(),
            description: "5min game with a 3sec increment each turn".to_string(),
            initial: Duration::from_secs(5 * 60),
            increment: Duration::from_secs(5),
        }
    }
}

impl Game {
    fn _is_move_valid(_move_name: &str) -> bool {
        todo!()
    }

    fn broadcast_state(&self) {
        self.srv.do_send(server::SendMessage::To(
            vec![self.white.clone(), self.black.clone()],
            OutgoingWsMsg::content("state", self.fen.clone()),
        ));
    }

    fn your_turn(&self) {
        let turn = match self.turn {
            true => self.white.clone(),
            false => self.black.clone(),
        };

        self.srv.do_send(GameAPI::YourTurn(
            self.id,
            turn,
            self.fen.clone(),
            Duration::from_secs(1),
        ));
    }

    fn _make_move(&mut self, _move_name: &str) {
        // chess.load_fen(&self.fen)
        // let r = chess.play_move(move_name)
        /*

        if r {
            self.
        }


        */
        todo!()
    }

    pub fn new(id: &usize, srv: &Addr<Server>, p1: &str, p2: &str, format: &TimeFormat) -> Game {
        Game {
            id: id.clone(),
            srv: srv.clone(),
            white: p2.to_string(),
            black: p1.to_string(),
            turn: true,
            time: format.clone(),
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        }
    }
}

impl Actor for Game {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("GAME: started");

        self.broadcast_state();
        self.your_turn();
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.srv.do_send(server::Cleanup::Game(self.id))
    }
}

#[derive(Message)]
#[rtype(result = "bool")]
pub enum API {
    GetState(String), // Username
    /// 1. move name
    Move(String),
}

impl Handler<API> for Game {
    type Result = bool;

    fn handle(&mut self, msg: API, _: &mut Self::Context) -> Self::Result {
        match msg {
            API::GetState(username) => {
                let fen = self.fen.clone();
                self.srv.do_send(server::SendMessage::To(
                    vec![username],
                    OutgoingWsMsg::content("state", fen),
                ));
            }
            API::Move(move_name) => {
                println!("playing move: {}", move_name);
            }
        }

        true
    }
}
