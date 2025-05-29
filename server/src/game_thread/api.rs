use actix::prelude::*;
use crate::{server_thread::api::server_thread_api as ServerThreadAPI, std_format_msgs::OutgoingWsMsg};

use super::GameThread;
use game_thread_api::*;

pub mod game_thread_api {
    use super::*;
    
    
    #[derive(Message)]
    #[rtype(result="bool")]
    pub enum CommandsAPI {
        RequestGameState(usize),
        PlayMove(String),
    }
    
}



impl Handler<CommandsAPI> for GameThread {
    type Result = bool;

    fn handle(&mut self, msg: CommandsAPI, ctx: &mut Self::Context) -> Self::Result {
        use CommandsAPI::*;
        
        match msg {
            RequestGameState(client_id) => {
                let fen = self.chessboard.to_fen();

                let msg = OutgoingWsMsg::content("state", fen);
                self.server_addr.do_send(ServerThreadAPI::ToClientBrowserAPI::MessageToClientID(client_id, msg));
            },
            PlayMove(move_name) => {
                if !self.chessboard.is_move_name_valid(&move_name) { return false }

                self.chessboard.play_notation(&move_name);
                // jeg vil gerne fortælle spillerne at vi der lige er blevet lavet et træk
                self.notify_player_of_turn();
            }
        };

        true
    }
}