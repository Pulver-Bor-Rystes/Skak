use crate::{client_thread::ClientThread, engine_thread::EngineThread, game_thread::GameThread};
use actix::prelude::*;
use rand::rngs::ThreadRng;
use sqlx::{MySql, Pool};
use std::collections::HashMap;

pub type Username = String;
pub type ID = usize;

mod actor;
pub mod api;
mod database;
mod logic;
mod types;

pub struct ServerThread {
    clients: HashMap<ID, (Username, Addr<ClientThread>)>,
    sql_conn: Pool<MySql>,
    engines: HashMap<String, Addr<EngineThread>>,
    games: HashMap<usize, (Username, Username, Addr<GameThread>)>,

    rng: ThreadRng,
    ids: Vec<usize>,
}
