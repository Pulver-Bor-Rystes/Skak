use actix::prelude::*;
use serde::Serialize;
use crate::{server_thread, std_format_msgs::OutgoingWsMsg};
use super::GameThread;



#[derive(Message)]
#[rtype(result="bool")]
pub enum CommandsAPI {
    RequestGameState(usize),
    PlayMove(String),
}

#[derive(Serialize, Debug, Clone)]
pub struct StateInfo {
    pub white: String,
}


impl Handler<CommandsAPI> for GameThread {
    type Result = bool;

    fn handle(&mut self, msg: CommandsAPI, _ctx: &mut Self::Context) -> Self::Result {
        use CommandsAPI::*;
        
        match msg {
            RequestGameState(client_id) => {
                // Fortæller brugeren hvem der er hvid
                self.server_addr.do_send(server_thread::api::ToClientBrowserAPI::MessageToClientID(client_id, OutgoingWsMsg::content("game:info", StateInfo {
                    white: self.white_username.clone(),
                })));

                // Notify turn
                self.server_addr.do_send(server_thread::api::ToClientBrowserAPI::MessageToClientID(client_id, OutgoingWsMsg::content("game:fen_state", self.chessboard.to_fen())));
            },
            PlayMove(move_name) => {
                if !self.chessboard.is_move_name_valid(&move_name) { return false }

                self.chessboard.play_notation(&move_name);
                // jeg vil gerne fortælle spillerne at der lige er blevet lavet et træk
                self.send_fen_state(None);
            }
        };

        true
    }
}