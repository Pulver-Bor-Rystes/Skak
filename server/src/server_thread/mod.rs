use std::collections::HashMap;
use actix::prelude::*;
use rand::rngs::ThreadRng;
use crate::{client_thread::ClientThread, engine_thread::EngineThread, game_thread::GameThread};


pub type Username = String;
pub type ID = usize;


mod actor;
pub mod api;
mod logic;
mod types;





pub struct ServerThread {
    clients: HashMap<ID, (Username, Addr<ClientThread>)>,
    engines: HashMap<String, Addr<EngineThread>>,
    games: HashMap<usize, (Username, Username, Addr<GameThread>)>,

    rng: ThreadRng,
    ids: Vec<usize>,
}



