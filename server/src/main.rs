use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, get, Responder};
use actix_web_actors::ws;

mod users;
mod security;
mod tests;


// use crate::security::HashedPassword;

const PORT: u16 = 4000;

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}


#[get("/")]
async fn index(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<h1>Hej Rasmus</h1>")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("running server on port {}", PORT);

    let res = users::signup("rasmus", "1234");
    println!("{:?}", res);

    HttpServer::new(|| App::new()
        .route("/ws/", web::get().to(ws_index))
        .service(index)
    )
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}