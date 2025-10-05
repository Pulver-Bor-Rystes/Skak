use std::{collections::HashMap, sync::{mpsc::{self, Receiver, Sender}, Arc, RwLock}};

use actix_web::{rt, web::{self, Data}, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use thiserror;

mod logging;

struct Conn {
    id: Option<usize>,
}

#[derive(Deserialize)]
struct Topic {
    topic: String
}


#[derive(Deserialize, Debug)]
struct Content<T = serde_json::Value> {
    topic: String,
    content: T,
}


#[derive(Deserialize, Debug)]
struct Auth {
    username: String,
    password: String,
}


#[derive(Serialize, Debug)]
enum Output {
    Cookie(String),
    WrongPassword,
    WasNotHandled,
    CouldNotSignup,
    SignupSuccesful,
}

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Invalid username: {0}")]
    InvalidUsername(String),

    #[error("Wrong password {0}")]
    WrongPassword(String),

    #[error("Only god knows what went wrong")]
    Idk,
}

async fn distribute(text: &str, client: &Arc<RwLock<Conn>>, db: &Pool<MySql>, server_api_tx: &Sender<ServerApi>) -> anyhow::Result<Output> {
    let t: Topic = serde_json::from_str(text)?;

    let r = match t.topic.as_str() {
        "login" => login(text, db).await?,
        "signup" => signup(text, db).await?,
        "invite" => invite(text, db).await?,
        _ => Output::WasNotHandled,
    };


    Ok(r)
}


async fn invite(text: &str, db: &Pool<MySql>) -> anyhow::Result<Output> {
    
}

async fn login(text: &str, db: &Pool<MySql>) -> anyhow::Result<Output> {
    let auth: Content<Auth> = serde_json::from_str(text)?;

    let db_psw = sqlx::query!(
        "select hash from Password where owner = ? limit 1;",
        auth.content.username
    )
    .fetch_one(db)
    .await?;

    if pwhash::bcrypt::verify(auth.content.password.clone(), &db_psw.hash) {
        let cookie = "random_cookie_string".to_string();
        let cookie_hash = pwhash::bcrypt::hash(&cookie)?;

        sqlx::query("insert into Cookie (hash, owner) values (?, ?)")
            .bind(cookie.clone())
            .bind(auth.content.username)
            .execute(db)
            .await?;

        return Ok(Output::Cookie(cookie));
    }

    Ok(Output::WrongPassword)
}


async fn signup(text: &str, db: &Pool<MySql>) -> anyhow::Result<Output> {
    let content: Content<Auth> = serde_json::from_str(text)?;

    let hash = pwhash::bcrypt::hash(content.content.password)?;
    
    sqlx::raw_sql("start transaction;").execute(db).await?;


    let user = sqlx::query("insert into User (username) values (?)")
        .bind(content.content.username.clone())
        .execute(db)
        .await;

    let psw = sqlx::query("insert into Password (hash, owner) values (?, ?)")
        .bind(hash)
        .bind(content.content.username)
        .execute(db)
        .await;


    if user.is_err() || psw.is_err() {
        error!("an error occured.\nrolling back...\nuser: {user:#?}, \npassword: {psw:#?}");
        sqlx::raw_sql("rollback").execute(db).await?;
        return Ok(Output::CouldNotSignup);
    }
    else {
        sqlx::raw_sql("commit").execute(db).await?;
        return Ok(Output::SignupSuccesful);
    }
}


async fn new_socket_conn(req: HttpRequest, stream: web::Payload, db: Data<Pool<MySql>>, server_api_tx: Data<Sender<ServerApi>>) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    let (client_api_tx, client_api_rx) = mpsc::channel::<ClientApi>();
    let mut client =
        Arc::new(
            RwLock::new(
                Conn {
                    id: None
                }
            )
        );

    

    // tager sig af alle udefrakommende api kald aka. socket requests
    let client1 = client.clone();
    rt::spawn(async move {
        let _ = server_api_tx.send(ServerApi::ClientNew("g".into(), client_api_tx.clone()));

        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    if text == "ping" { session.text("pong").await.unwrap(); continue }
                  
                    let res = distribute(&text.to_string(), &client1, db.get_ref(), server_api_tx.get_ref()).await;

                    match res {
                        Ok(v) => session.text(serde_json::to_string(&v).expect("åh nej")).await.unwrap(),
                        Err(err) => {
                            error!("{text}\n{err:?}");
                        }
                    }
                }
                _ => {}
            }
        }
    });

    // tager sig af alle interne api kald
    std::thread::spawn(move || {
        loop {
            // info!("asdbshdbsbhd");
            match client_api_rx.try_recv() {
                Ok(msg) => match msg {
                    ClientApi::Id(id) => {
                        info!("setting id: {id}");
                        let mut client_w = client.write().unwrap();
                        client_w.id = Some(id);
                    },
                },
                Err(_) => {},
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}

enum ClientApi {
    Id(usize),
}

enum ServerApi {
    ClientNew(String, mpsc::Sender<ClientApi>),
    ClientLogin,
    ClientDestroy,
}

struct Server {
    /// String: username
    /// Sender<ClientApi> giver vel lidt sig selv
    clients: HashMap<String, Vec<Sender<ClientApi>>>,
    latest_id: usize,
}

fn server_thread(server_api_rx: Receiver<ServerApi>) {
    info!("hello from server thread");

    let mut server = Server {
        clients: HashMap::new(),
        latest_id: 0,
    };

    loop {
        let msg = server_api_rx.try_recv();
        if let Ok(msg) = msg {
            match msg {
                ServerApi::ClientNew(username, client_api_tx) => {
                    server.latest_id += 1;
                    let _ = client_api_tx.send(ClientApi::Id(server.latest_id));

                    // if the username is already reg, then insert into the vec
                    if let Some(vec) = server.clients.get_mut(&username) {
                        vec.push(client_api_tx);
                    }
                    else {
                        server.clients.insert(username, vec![client_api_tx]);
                    }
                },
                // ServerApi::ClientLogin => info!("\n\n ------- login ------- \n\n"),
                _ => {}
            }
        }
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    
    let db_url = format!("mysql://root:@localhost/ChessDB");
    let db_conn: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to MySQL\n");


    let (server_api_tx, server_api_rx) = mpsc::channel::<ServerApi>();

    std::thread::spawn(|| server_thread(server_api_rx));

    
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_conn.clone()))
            .app_data(Data::new(server_api_tx.clone()))
            .route("/api/ws", web::get().to(new_socket_conn))
        })
        .bind(("127.0.0.1", 4000))?
        .run()
        .await
}
