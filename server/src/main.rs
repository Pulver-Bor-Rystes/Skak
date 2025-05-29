const PORT: u16 = 4000;

// Add HttpRequest and HttpResponse
use actix::*;
use actix_web::*;
use actix_web_actors::ws;
use client_thread::ClientThread;
use server_thread::ServerThread;
use web::Data;


mod server_thread;
mod client_thread;
mod game_thread;
mod engine_thread;

pub mod auth;
pub mod types;
pub mod validate;
mod std_format_msgs;

// WebSocket handshake and start `MyWebSocket` actor.
async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    server_addr: web::Data<Addr<ServerThread>>,
) -> Result<HttpResponse, Error> {
    let server_ref = server_addr.get_ref().clone();
    ws::start(ClientThread::new(server_ref), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // clears the console
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("starting HTTP server at http://localhost:4000");

    let server = ServerThread::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(server.clone()))
            .service(web::resource("/api/ws").route(web::get().to(websocket))) // Add the WebSocket route
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
