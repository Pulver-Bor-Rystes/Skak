use actix::prelude::*;
use chess_machine_lib::chess::chess_types::ChessBoard;
use crate::server_thread::{self, api::server_thread_api as ServerThreadAPI};

pub mod api;
pub mod types;
mod logic;


pub struct GameThread {
    id: usize,

    server_addr: Addr<server_thread::ServerThread>,
    white_username: String,
    black_username: String,

    chessboard: ChessBoard,
    time_format: types::TimeFormat,
}


impl Actor for GameThread {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("GAME: started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let p1 = self.white_username.clone();
        let p2 = self.black_username.clone();
        
        self.server_addr.do_send(ServerThreadAPI::ClientCommandsAPI::LeaveGame(p1));
        self.server_addr.do_send(ServerThreadAPI::ClientCommandsAPI::LeaveGame(p2));
    }
}