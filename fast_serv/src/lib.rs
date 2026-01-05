use log::LevelFilter;
use mysql::{Pool, params, prelude::Queryable};
use prelude::*;
use std::{error::Error, io::Write, net::TcpListener};

mod client_thread;
mod database_inserts;
mod engine_thread;
mod errors;
mod parsing;
mod prelude;
mod server_thread;
mod types;

pub fn new_user(username: &str, psw: &str, pool: &Pool) -> Result<(), Box<dyn Error>> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "INSERT INTO User (username) VALUES (:username)",
        params! {
            "username" => username,
        },
    )?;

    let psw_hash = pwhash::bcrypt::hash(&psw)?;

    conn.exec_drop(
        "insert into Password (owner, hash) values (:owner, :hash)",
        params! {
            "owner" => username,
            "hash" => psw_hash,
        },
    )?;

    Ok(())
}

pub fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root@localhost:3306/ChessDB";
    let pool = Pool::new(url)?;

    let server = TcpListener::bind("127.0.0.1:4000").unwrap();
    info!("Listening on ws://127.0.0.1:4000");

    let server_api = Server::new();

    for stream in server.incoming() {
        let websocket = tungstenite::accept(stream.unwrap()).unwrap();
        Client::new(websocket, server_api.clone(), pool.clone());
    }

    Ok(())
}

pub fn init_logger(filter: LevelFilter) {
    let mut builder = env_logger::Builder::from_default_env();

    builder
        .format(|buf, record| {
            let file = record.file().unwrap();
            let line = record.line().unwrap();
            let level = record.level();
            let args = record.args();
            let target = record.target();

            let extra_padding = 6 - level.as_str().len();
            let pad = " ".repeat(extra_padding);

            let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            use log::Level::*;

            // term colors cheatsheet https://gist.github.com/DNA/ebb9258089e9e1dfd08c58695b3cd6f1
            match record.level() {
                Info => writeln!(buf, "\x1b[2;37m[\x1b[97m{time} \x1b[0;32m{level}\x1b[0m{pad}{target}\x1b[2;37m]\x1b[0m {args}"),
                Debug => writeln!(buf, "\x1b[2;37m[\x1b[0m{time} \x1b[0;34m{level}\x1b[0m{pad}\x1b[37m{file}:{line}\x1b[2;37m] \x1b[0m{args}"),
                Error => writeln!(buf, "\x1b[2;37m[\x1b[0m{time} \x1b[0;31m{level}\x1b[0m{pad}\x1b[37m{file}:{line}\x1b[2;37m] \x1b[0m{args}"),
                Warn => writeln!(buf, "\x1b[2;37m[\x1b[0m{time} \x1b[0;33m{level}\x1b[0m{pad}{target}\x1b[2;37m] \x1b[0m{args}"),
                Trace => writeln!(buf, "\x1b[2;37m[\x1b[0m{time} \x1b[0;36m{level}\x1b[0m{pad}{target}\x1b[2;37m] \x1b[0m{args}"),
            }
        })
        .filter(None, filter)
        .init();
}
