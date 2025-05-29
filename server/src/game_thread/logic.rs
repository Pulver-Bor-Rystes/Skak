
use actix::prelude::*;
use chess_machine_lib::chess::chess_types::{ChessBoard, ChessColor, NamingConvention};
use crate::{server_thread::ServerThread, server_thread::api::server_thread_api as ServerThreadAPI};
use super::{types::TimeFormat, GameThread};

impl GameThread {
    pub fn new(id: usize, server_thread: Addr<ServerThread>, p1: &str, p2: &str, time_format: TimeFormat) -> Self {
        let mut chessboard = ChessBoard::default();
        chessboard.set_naming_convention(NamingConvention::LongAlgebraicNotation);

        GameThread {
            id,
            white_username: p1.into(),
            black_username: p2.into(),
            chessboard,
            server_addr: server_thread,
            time_format
        }
    }


    pub fn on_spawn(&self) {
        self.notify_player_of_turn();
    }


    // pub fn broadcast_state(&self) {
    //     let p1 = self.white_username.clone();
    //     let p2 = self.black_username.clone();

    //     self.server_addr.do_send(ServerThreadAPI::ToClientBrowserAPI::MessageToUsername(p1, self.chessboard.to_fen()));
    //     self.server_addr.do_send(ServerThreadAPI::ToClientBrowserAPI::MessageToUsername(p2, self.chessboard.to_fen()));
    // }


    pub fn notify_player_of_turn(&self) {
        let username: String = match self.chessboard.turn {
            ChessColor::White => self.white_username.clone(),
            ChessColor::Black => self.black_username.clone(),
        };


        println!("\n -----> TURN: {}", username);


        self.server_addr.do_send(ServerThreadAPI::ClientCommandsAPI::NotifyYourTurn(self.id, username, self.chessboard.to_fen()));
    }
}
