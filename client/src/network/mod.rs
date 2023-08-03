use std::str::from_utf8;

use awc::{ws, Client};
use bevy::prelude::*;
use common::RoomAction;
use futures_util::{SinkExt as _, StreamExt as _};

pub struct NetworkPlugin;
pub struct CreatePartyEvent;
pub struct JoinRoomEvent(pub String);

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreatePartyEvent>()
            .add_event::<JoinRoomEvent>()
            .add_system(create_party);
    }
}

#[actix_rt::main]
async fn create_party(
    mut create_ev: EventReader<CreatePartyEvent>,
    mut join_ev: EventReader<JoinRoomEvent>,
) {
    for _ in create_ev.iter() {
        let (_res, mut ws) = Client::new()
            .ws("ws://127.0.0.1:8080/ws/")
            .connect()
            .await
            .unwrap();

        let create_room_json =
            serde_json::to_string(&RoomAction::CreateRoom("test room".into())).unwrap();
        ws.send(ws::Message::Text(create_room_json.into()))
            .await
            .unwrap();

        loop {
            if let Ok(ws_msg) = ws.next().await.unwrap() {
                match ws_msg {
                    ws::Frame::Text(msg) => match from_utf8(&msg) {
                        Ok(txt) => {
                            let action: RoomAction = serde_json::from_str(txt).unwrap();
                            match action {
                                RoomAction::RoomCreated => println!("room created"),
                                _ => {
                                    println!("not action: {:?}", action);
                                }
                            }
                        }
                        _ => {
                            println!("ws message: {:?}", msg);
                        }
                    },
                    _ => println!("failed to createy party"),
                }
                break;
            }
        }
    }

    for ev in join_ev.iter() {
        let (_res, mut ws) = Client::new()
            .ws("ws://127.0.0.1:8080/ws/")
            .connect()
            .await
            .unwrap();

        let join_room_json = serde_json::to_string(&RoomAction::JoinRoom(ev.0.clone())).unwrap();
        ws.send(ws::Message::Text(join_room_json.into()))
            .await
            .unwrap();

        loop {
            if let Ok(ws_msg) = ws.next().await.unwrap() {
                match ws_msg {
                    ws::Frame::Text(msg) => match from_utf8(&msg) {
                        Ok(txt) => {
                            let action: RoomAction = serde_json::from_str(txt).unwrap();
                            match action {
                                RoomAction::JoinedRoom => println!("join success!"),
                                _ => println!("join room failed {:?}", action),
                            }
                        }
                        _ => println!("ws message: {:?}", msg),
                    },
                    _ => println!("failed to join"),
                }
            }
            break;
        }
    }
}
