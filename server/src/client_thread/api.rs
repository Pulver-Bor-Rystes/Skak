use super::ClientThread;
use actix::prelude::*;
use serde::Serialize;


pub mod client_thread_api {
    use actix::prelude::*;
    use serde::Serialize;

    #[derive(Message)]
    #[rtype(result="usize")]
    pub enum IdentifierAPI {
        Set(usize),
    }
    
    
    #[derive(Message)]
    #[rtype(result="bool")]
    pub enum GameAPI {
        SetInGame(bool),
    }


    #[derive(Message)]
    #[rtype(result="bool")]
    pub enum BrowserAPI<M> where M: Serialize + std::marker::Send, {
        Message(M),
    }
}



impl Handler<client_thread_api::IdentifierAPI> for ClientThread {
    type Result = usize;
    
    fn handle(&mut self, msg: client_thread_api::IdentifierAPI, _ctx: &mut Self::Context) -> Self::Result {
        use client_thread_api::IdentifierAPI::*;
        
        match msg { 
            Set(id) => { self.id = Some(id); id },
        }
    }
}


impl Handler<client_thread_api::GameAPI> for ClientThread {
    type Result = bool;
    
    fn handle(&mut self, msg: client_thread_api::GameAPI, _ctx: &mut Self::Context) -> Self::Result {
        use client_thread_api::GameAPI::*;
        
        match msg { 
            SetInGame(value) => self.in_game = value,
        };

        true
    }
}


impl<M> Handler<client_thread_api::BrowserAPI<M>> for ClientThread where M: Serialize + std::marker::Send + std::fmt::Debug, {
    type Result = bool;
    
    fn handle(&mut self, msg: client_thread_api::BrowserAPI<M>, ctx: &mut Self::Context) -> Self::Result {
        use client_thread_api::BrowserAPI::*;
        
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