use crate::actors;
use actix::prelude::*;
use std::io::{self, BufRead};
use std::process::{Command, Stdio};

#[test]
fn uci() {
    let srv = actors::server::Server::new().start();

    // let game = actors::game::Game::new(1, srv, "user1", "user2").start();

    assert!(true)
}
