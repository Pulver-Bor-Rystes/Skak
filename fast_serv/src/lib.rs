use log::LevelFilter;
use mysql::{
    Params, Pool, params,
    prelude::{Queryable, TextQuery},
};
use prelude::*;
use rand::{Rng, distr::Alphanumeric};
use std::{error::Error, io::Write, net::TcpListener};

mod client_thread;
mod database_inserts;
mod engine_thread;
mod errors;
mod parsing;
mod prelude;
mod server_thread;
mod types;

const DB_NAME: &'static str = "ChessDB";

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

pub fn check_if_db_exists() -> Option<Pool> {
    let url = String::from("mysql://root@localhost:3306/") + DB_NAME;
    match Pool::new(url.as_str()) {
        Ok(p) => Some(p),
        Err(_) => None,
    }
}

pub fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let pool = run_db_check();
    let server = TcpListener::bind("127.0.0.1:4000").unwrap();
    info!("Listening on ws://127.0.0.1:4000");

    let server_api = Server::new();

    for stream in server.incoming() {
        let websocket = tungstenite::accept(stream.unwrap()).unwrap();
        Client::new(websocket, server_api.clone(), pool.clone());
    }

    Ok(())
}

fn read_index_file() -> isize {
    let index_content = std::fs::read_to_string("sql/index.txt").unwrap_or_else(|_err| {
        std::fs::write("sql/index.txt", "-1").expect("idk - burde virke :))");
        info!("SQL index file was created with index: -1");
        String::from("-1")
    });
    index_content.trim().parse::<isize>().unwrap()
}

pub fn load_sql_files() -> Vec<(String, String)> {
    // load the index file
    let last_index = read_index_file();

    let sql_folder = std::fs::read_dir("sql").expect("sql folder is missing");

    // files: Vec<name, sql_statement>
    let mut files = Vec::new();

    for entry in sql_folder {
        let path = entry.unwrap().path();
        if path.extension().unwrap() == "sql" {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let sql = std::fs::read_to_string(path).unwrap();

            files.push((name, sql));
        }
    }

    // sort file by index
    files.sort_by_key(|v| {
        v.0.split("_")
            .next()
            .expect("sql file not matching format! 'n_name.sql'") // nu er det ikke længere garanteret at det virker. Måske glemmer jeg formattet i fremtiden🤷‍♀️
            .parse::<usize>()
            .expect("sql file is supposed to start with an index nr!")
    });

    // filter alt fra, som er under eller lig med last_index
    files.retain(|v| {
        v.0.split("_")
            .next()
            .expect("sql file not matching format! 'n_name.sql'") // nu er det ikke længere garanteret at det virker. Måske glemmer jeg formattet i fremtiden🤷‍♀️
            .parse::<isize>()
            .expect("sql file is supposed to start with an index nr!")
            > last_index
    });

    files
}

pub fn run_db_check() -> Pool {
    // Forbinder til DB
    let url = "mysql://root@localhost:3306";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let mut sql_files = load_sql_files();

    let mut first_run_after_db_creation = false;

    // vi skal tjekke om databasen eksisterer
    // og oprette, hvis det giver mening
    if let None = check_if_db_exists() {
        // fjerner index.txt og genindlæser sql filerne
        let _ = std::fs::remove_file("sql/index.txt");
        sql_files = load_sql_files();

        info!("Running database init protocol");
        first_run_after_db_creation = true;

        // finder 0_xxx.sql file aka. den første
        let create_db_statement = sql_files.first().expect("missing sql statement!");
        let _ = conn.exec_drop(create_db_statement.1.clone(), Params::Empty);
        let _ = conn.as_mut().select_db(DB_NAME);

        // make an index file and save it
        let _ = std::fs::write("sql/index.txt", "0");
        info!("SQL index file was created!");

        // fjern det første element fra sql_files
        let _ = sql_files.remove(0);
    } else {
        let _ = conn.as_mut().select_db(DB_NAME);
    }

    // databasen eksisterer, så kan vi køre de resterende scripts
    // når sql filerne bliver loadet bliver dem der allerede er kørt automatisk sorteret fra!

    for (name, stmt) in &sql_files {
        let status = match conn.exec_drop(stmt, Params::Empty) {
            Err(e) => {
                error!("{e:?}");
                "👎"
            }
            Ok(_) => "👍",
        };

        info!("exec: {name} ... {status}");
    }

    // write the last value to index.txt
    if sql_files.len() > 0 {
        let length = sql_files
            .last()
            .unwrap()
            .0
            .split("_")
            .next()
            .expect("sql file not matching format! 'n_name.sql'") // nu er det ikke længere garanteret at det virker. Måske glemmer jeg formattet i fremtiden🤷‍♀️
            .parse::<usize>()
            .expect("sql file is supposed to start with an index nr!");

        let _ = std::fs::write("sql/index.txt", length.to_string());
    }

    let url = String::from("mysql://root@localhost:3306/") + DB_NAME;
    let pool = Pool::new(url.as_str()).unwrap();

    if first_run_after_db_creation {
        generate_admin_invite_code(&pool);
    }

    pool
}

fn generate_admin_invite_code(pool: &Pool) {
    let mut conn = pool.get_conn().unwrap();

    let code: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let code_hash = pwhash::bcrypt::hash(&code).unwrap();

    conn.exec_drop(
        "insert into User (username, rating) values ('__server__', 0);",
        Params::Empty,
    )
    .expect("Kunne ikke oprette server som en bruger");

    // conn.exec_drop("inse", params)
    let res = conn.exec_drop(
        "insert into InviteCode (privilege, code, created_by) values ('admin', :code, '__server__');",
        params! {
            "code" => code_hash,
        },
    );

    match res {
        Ok(_) => info!("Invite code generated: >>{code}<<"),
        Err(err) => error!("Invite code gen failed! {err:?}"),
    }
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
