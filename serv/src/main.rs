use actix_web::{web::{self, Data}, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use thiserror;
use crate::threads::{client::new_socket_conn, server::Server};

mod logging;
mod threads;


#[derive(Serialize)]
pub struct TopicContent<T = serde_json::Value> {
    topic: String,
    result: bool,
    content: T,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Topic {
    topic: String
}


#[derive(Deserialize, Debug)]
pub struct Content<T = serde_json::Value> {
    content: T,
}


#[derive(Deserialize, Debug)]
pub struct Auth {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct Invite {
    to: String,
}



#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Invalid username: {0}")]
    InvalidUsername(String),

    #[error("Wrong password")]
    WrongPassword,

    #[error("Wrong cookie")]
    WrongCookie,

    #[error("Unable to signup")]
    UnableToSignup,

    #[error("Only god knows what went wrong")]
    Idk,

    #[error("Not logged in")]
    NotLoggedIn,

    #[error("Invite failed")]
    InviteFailed,

    #[error("Topic not handled")]
    TopicNotHandled,

}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = format!("mysql://root:@localhost/ChessDB");
    let db_conn: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to MySQL\n");

    let latest_id: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));

    let stx = Server::new();
    
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_conn.clone()))
            .app_data(Data::new(stx.clone()))
            .app_data(Data::new(latest_id.clone()))
            .route("/api/ws", web::get().to(new_socket_conn))
        })
        .bind(("127.0.0.1", 4000))?
        .run()
        .await
}
