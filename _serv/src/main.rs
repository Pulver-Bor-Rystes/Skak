use std::sync::mpsc::Sender;
use ctrlc;

// Add HttpRequest and HttpResponse
use actix::*;
use actix_web::*;
use actix_web_actors::ws;
use client_thread::ClientThread;
use server_thread::ServerThread;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use web::Data;

use crate::server_thread::ServerAPI;

mod client_thread;
// mod engine_thread;
// mod game_thread;
mod server_thread;

mod logging;
mod std_format_msgs;
// pub mod types;
// pub mod validate;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("starting HTTP server at http://localhost:4000");

    // Build SQLx connection pool
    let db_url = format!("mysql://root:@localhost/ChessDB");
    let db_conn: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to MySQL\n\n -> Ensure that {DB} exists!");


    let server = ServerThread::new(db_conn.clone()).start();
    
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(server.clone()))
            .app_data(Data::new(db_conn.clone()))
            .service(web::resource("/api/ws").route(web::get().to(websocket)))
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}




// Hver gang vi får en ny websocket forbindelse starter vi en thread til den forbindelse!
async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    server_addr: web::Data<Addr<ServerThread>>,
    sql_conn: web::Data<Pool<MySql>>,
) -> Result<HttpResponse, Error> {
    let server_ref = server_addr.get_ref().clone();
    let sql_conn = sql_conn.get_ref().clone();

    ws::start(ClientThread::new(server_ref, sql_conn), &req, stream)
}
