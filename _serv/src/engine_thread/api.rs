use actix::prelude::*;
use std::time::Duration;
use super::EngineThread;



type FenString = String;




#[derive(Message)]
#[rtype(result = "String")]
pub enum EngineThreadAPI {
    Search(FenString, Duration),
}


impl Handler<EngineThreadAPI> for EngineThread {
    type Result = String;

    fn handle(&mut self, msg: EngineThreadAPI, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            EngineThreadAPI::Search(position, duration) => self.search(position, duration),
        }
    }
}