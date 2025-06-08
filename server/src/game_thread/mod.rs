use actix::prelude::*;
use chess_machine_lib::chess::chess_types::ChessBoard;
use crate::{info, server_thread::{self}};

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
        info!(" >> [GAME] Started");
        self.on_actor_spawn();
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let p1 = self.white_username.clone();
        let p2 = self.black_username.clone();
        
        self.server_addr.do_send(server_thread::api::ClientCommandsAPI::LeaveGame(p1));
        self.server_addr.do_send(server_thread::api::ClientCommandsAPI::LeaveGame(p2));
        self.server_addr.do_send(server_thread::api::CommandsAPI::RemoveGame(self.id)); // remove game from server
    }
}