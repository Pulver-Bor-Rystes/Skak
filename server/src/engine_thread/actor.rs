use crate::server_thread;

use super::*;


impl Actor for EngineThread {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.enable_uci();
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!(" -> {} is stopping", self.name);
        self.server_addr.do_send(server_thread::api::CommandsAPI::RemoveEngine(self.name.clone()));

        Running::Stop
    }
}