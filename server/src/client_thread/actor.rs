use actix_web_actors::ws;

use super::*;


impl Actor for ClientThread {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!(" > A new socket connection was established!");

        self.addr = Some(ctx.address())
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {

    }
}
