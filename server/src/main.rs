use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix::{Actor, Addr, AsyncContext, StreamHandler};
use actix_cors::Cors;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use common::RoomAction;
use room::Room;
use uuid::Uuid;

mod room;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_data = web::Data::new(Mutex::new(ServerState::new()));
    let config_server = move || {
        App::new()
            .wrap(Cors::default().allow_any_origin())
            .app_data(server_data.clone())
            .route("/ws/", web::get().to(index))
    };
    HttpServer::new(config_server)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Debug)]
struct ServerState {
    rooms: HashMap<String, Room>,
}

impl ServerState {
    fn new() -> Self {
        ServerState {
            rooms: HashMap::new(),
        }
    }

    fn create_room(&mut self, room_name: String, client: Addr<ServerWs>) {
        let room_id = Uuid::new_v4().to_string();
        let mut room = Room::new(room_id);
        room.add_client(client);
        self.rooms.insert(room_name, room);
        println!("rooms: {:?}", self.rooms);
    }

    fn join_room(&mut self, room_name: String, client: Addr<ServerWs>) -> Result<(), String> {
        println!("Joining room {}", room_name);
        println!("rooms: {:?}", self.rooms);
        if let Some(room) = self.rooms.get_mut(&room_name) {
            room.add_client(client);
            Ok(())
        } else {
            Err(format!("Room '{}' does not exist", room_name))
        }
    }
}

pub struct ServerWs {
    state: Arc<Mutex<ServerState>>,
}

impl ServerWs {
    fn new(state: Arc<Mutex<ServerState>>) -> Self {
        ServerWs { state }
    }
}

impl Actor for ServerWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ServerWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => match serde_json::from_str(&text.to_string().as_str()) {
                Ok(action) => match action {
                    RoomAction::CreateRoom(room_name) => {
                        println!("create room");
                        self.state
                            .lock()
                            .unwrap()
                            .create_room(room_name, ctx.address());
                        let room_json = serde_json::to_string(&RoomAction::RoomCreated).unwrap();
                        ctx.text(room_json);
                    }
                    RoomAction::JoinRoom(room_name) => {
                        println!("join {}", room_name);
                        if let Err(err) = self
                            .state
                            .lock()
                            .unwrap()
                            .join_room(room_name, ctx.address())
                        {
                            ctx.text(serde_json::to_string(&RoomAction::Invalid(err)).unwrap());
                        } else {
                            ctx.text(serde_json::to_string(&RoomAction::JoinedRoom).unwrap());
                        }
                    }
                    _ => ctx.text("failed"),
                },
                _ => (),
            },
            _ => (),
        }
    }
}

async fn index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Mutex<ServerState>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(ServerWs::new(data.into_inner()), &req, stream);
    println!("req: {:?} response: {:?}", req, resp);
    resp
}
