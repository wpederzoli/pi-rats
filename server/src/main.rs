use actix::{Actor, StreamHandler};
use actix_cors::Cors;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use common::RoomAction;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin())
            .route("/ws/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

struct ServerWs;

impl Actor for ServerWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ServerWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => match RoomAction::get(text.to_string().as_str()) {
                RoomAction::CreateRoom => {
                    //Create a room with a UUID and respond with RoomCreated action
                    //Add to map of room_name - UUID
                    //containing the newly created roomId
                    ctx.text("create room")
                }
                _ => ctx.text("failed"),
            },
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(ServerWs {}, &req, stream);
    println!("req: {:?} response: {:?}", req, resp);
    resp
}
