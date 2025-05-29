use std::time::Duration;
use actix::prelude::*;

use crate::server_thread::ServerThread;

mod actor;
pub mod api;
mod logic;
mod types;


pub struct EngineThread {
    server_addr: Addr<ServerThread>,
    name: String,
    handle: std::process::Child,
    response_over: types::ResponseOverAfter,
}