use crate::std_format_msgs::OutgoingWsMsg;
use super::ClientThread;
use actix::prelude::*;
use serde::Serialize;


type FenString = String;


#[derive(Message)]
#[rtype(result="usize")]
pub enum IdentifierAPI {
    Set(usize),
}


#[derive(Message)]
#[rtype(result="bool")]
pub enum GameAPI {
    SetInGame(bool),
    YourTurn(FenString),
}


#[derive(Message)]
#[rtype(result="bool")]
pub enum BrowserAPI<M> where M: Serialize + std::marker::Send, {
    Message(M),
}



impl Handler<IdentifierAPI> for ClientThread {
    type Result = usize;
    
    fn handle(&mut self, msg: IdentifierAPI, _ctx: &mut Self::Context) -> Self::Result {
        use IdentifierAPI::*;
        
        match msg { 
            Set(id) => { self.id = Some(id); id },
        }
    }
}


impl Handler<GameAPI> for ClientThread {
    type Result = bool;
    
    fn handle(&mut self, msg: GameAPI, ctx: &mut Self::Context) -> Self::Result {
        use GameAPI::*;
        
        match msg { 
            SetInGame(value) => self.in_game = value,
            YourTurn(fen) => ctx.text(OutgoingWsMsg::content("state", fen).serialize()),
        };

        true
    }
}


impl<M> Handler<BrowserAPI<M>> for ClientThread where M: Serialize + std::marker::Send + std::fmt::Debug, {
    type Result = bool;
    
    fn handle(&mut self, msg: BrowserAPI<M>, ctx: &mut Self::Context) -> Self::Result {
        use BrowserAPI::*;
        
        match msg { 
            Message(msg) => {
                // println!(" About to send stuff over to client: {:#?}", msg);
                let msg: String = serde_json::to_string(&msg).expect("json could not be parsed");
                ctx.text(msg);
            },
        }

        true
    }
}