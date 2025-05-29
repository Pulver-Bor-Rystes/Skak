use std::time::Duration;
use actix::prelude::*;

mod actor;
pub mod api;
mod logic;
mod types;


pub struct EngineThread {
    name: String,
    handle: std::process::Child,
    response_over: types::ResponseOverAfter,
}