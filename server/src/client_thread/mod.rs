use crate::{server_thread::ServerThread};
use std::time::Instant;
use actix::prelude::*;


mod actor;
pub mod api;
pub mod logic;
pub mod web_socket_stuff;
mod types;



pub struct ClientThread {
    pub addr: Option<Addr<ClientThread>>,
    pub server_addr: Addr<ServerThread>,
    pub id: Option<usize>,
    pub hb: Instant,

    // data der skal registreres på socket'en
    pub username: Option<String>,

    // meta
    pub in_game: bool,
}