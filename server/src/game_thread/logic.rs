
use actix::prelude::*;
use chess_machine_lib::chess::chess_types::{ChessBoard, ChessColor, NamingConvention};
use crate::{server_thread::ServerThread, std_format_msgs::OutgoingWsMsg};
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
}
