use super::*;


#[derive(Message)]
#[rtype(result = "String")]
pub enum EngineThreadAPI {
    Search(String, Duration),
}



impl Handler<EngineThreadAPI> for EngineThread {
    type Result = String;

    fn handle(&mut self, msg: EngineThreadAPI, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            EngineThreadAPI::Search(position, duration) => self.search(position, duration),
        }
    }
}
