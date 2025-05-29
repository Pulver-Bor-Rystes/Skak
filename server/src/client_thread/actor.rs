use actix_web_actors::ws;

use crate::server_thread;

use super::*;


impl Actor for ClientThread {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address());
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.server_addr.do_send(server_thread::api::CommandsAPI::RemoveClient(self.id.unwrap()));
    }
}
