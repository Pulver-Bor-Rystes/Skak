const PORT: u16 = 4000;


// Add HttpRequest and HttpResponse
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

mod server;
mod user_api;
mod security;
mod com;
mod tests;
use self::server::MyWebSocket;

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<h1>Hej Rasmus</h1>")
}

// WebSocket handshake and start `MyWebSocket` actor.
async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://localhost:4000");

    HttpServer::new(|| {
        App::new()
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