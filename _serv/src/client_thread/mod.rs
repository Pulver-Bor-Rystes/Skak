use std::time::Instant;
use actix::prelude::*;
use actix_web_actors::ws;
use sqlx::{MySql, Pool};

use crate::server_thread::{ServerAPI, ServerThread};


// mod actor;
// pub mod api;
pub mod logic;
pub mod web_socket_stuff;
// mod types;

#[derive(Message)]
#[rtype(result="()")]
pub enum ClientAPI {
    SetID(usize),
}


pub struct ClientThread {
    server_addr: Addr<ServerThread>,
    sql_conn: Pool<MySql>,
    pub id: Option<usize>,
    pub hb: Instant,

    // data der skal registreres på socket'en
    pub username: Option<String>,

    // meta
    pub in_game: bool,
}


impl Handler<ClientAPI> for ClientThread {
    type Result = ();

    fn handle(&mut self, msg: ClientAPI, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ClientAPI::SetID(id) => self.id = Some(id),
        }
    }
}


impl Actor for ClientThread {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.server_addr.do_send(ServerAPI::ClientNew(ctx.address()));
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        if let Some(id) = self.id {
            self.server_addr.do_send(ServerAPI::RemoveClient(id));
        }
    }
}
