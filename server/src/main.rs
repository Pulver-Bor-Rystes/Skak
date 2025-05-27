const PORT: u16 = 4000;

// Add HttpRequest and HttpResponse
use actix::*;
use actix_web::*;
use actix_web_actors::ws;
use refactor::{server::server_actor::Server, sockets::SocketSession};
use web::Data;

mod refactor;


// mod actors;
mod std_format_msgs;
// mod tests;

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<h1>Hej Rasmus</h1>")
}

// WebSocket handshake and start `MyWebSocket` actor.
async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    server_addr: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let server_ref = server_addr.get_ref().clone();
    // ws::start(Session::new(server_ref), &req, stream)
    ws::start(SocketSession::new(server_ref), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // clears the console
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("starting HTTP server at http://localhost:4000");

    let server = Server::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(server.clone()))
            // .app_data(Data::new(juules_engine.clone()))
            // .app_data(Data::new(stockfish_engine.clone()))
            .service(web::resource("/").to(index))
            // Add the WebSocket route
            .service(web::resource("/api/ws").route(web::get().to(websocket)))
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
