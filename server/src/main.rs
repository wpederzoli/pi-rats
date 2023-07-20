use actix::{Actor, StreamHandler};
use actix_cors::Cors;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin())
            .service(home)
            .route("/ws/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

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

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("hi")
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("req: {:?} response: {:?}", req, resp);
    resp
}

// use actix::{Actor, StreamHandler};
// use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
// use actix_web_actors::ws;
//
// /// Define HTTP actor
// struct MyWs;
//
// impl Actor for MyWs {
//     type Context = ws::WebsocketContext<Self>;
// }
//
// /// Handler for ws::Message message
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
//             Ok(ws::Message::Text(text)) => ctx.text(text),
//             Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
//             _ => (),
//         }
//     }
// }
//
// async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
//     let resp = ws::start(MyWs {}, &req, stream);
//     println!("{:?}", resp);
//     resp
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }
