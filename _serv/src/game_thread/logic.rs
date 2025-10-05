
use actix::prelude::*;
use chess_machine_lib::chess::chess_types::{ChessBoard, ChessColor, NamingConvention};
use crate::{info, server_thread::{self, ServerThread}, std_format_msgs::OutgoingWsMsg};
use super::{api::StateInfo, types::TimeFormat, GameThread};

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


    pub fn on_actor_spawn(&self) {
        self.send_fen_state(None);


        // Fortæller hvis tur det er
        self.server_addr.do_send(server_thread::api::ToClientBrowserAPI::MessageToUsername(self.white_username.clone(), OutgoingWsMsg::content("game:info", StateInfo {
            white: self.white_username.clone(),
        })));

        self.server_addr.do_send(server_thread::api::ToClientBrowserAPI::MessageToUsername(self.black_username.clone(), OutgoingWsMsg::content("game:info", StateInfo {
            white: self.white_username.clone(),
        })));
    }

    /// Sender fen_state til den spiller som har turen, med mindre man beder den sende til en specifik bruger :)
    pub fn send_fen_state(&self, override_target: Option<String>) {
        let target_user: String = match (self.chessboard.turn, override_target) {
            (_, Some(override_target)) => override_target,
            (ChessColor::White, _) => self.white_username.clone(),
            (ChessColor::Black, _) => self.black_username.clone(),
        };

        self.server_addr.do_send(server_thread::api::ClientCommandsAPI::NotifyYourTurn(self.id, target_user, self.chessboard.to_fen()));
    }
}
