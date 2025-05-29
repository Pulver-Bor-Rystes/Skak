use actix::prelude::*;
use crate::{server_thread::api::server_thread_api as ServerThreadAPI, std_format_msgs::OutgoingWsMsg};

use super::GameThread;
use game_thread_api::*;

pub mod game_thread_api {
    use super::*;
    
    
    impl Message for CommandsAPI { type Result = bool; }
    pub enum CommandsAPI {
        RequestGameState(usize),
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
                self.server_addr.do_send(ServerThreadAPI::ToClientBrowserAPI::Message(client_id, msg));

                true
            },
        }
    }
}