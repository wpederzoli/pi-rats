use std::str::from_utf8;

use awc::{ws, Client};
use bevy::prelude::*;
use common::RoomAction;
use futures_util::{SinkExt as _, StreamExt as _};

use crate::GameState;

pub struct NetworkPlugin;
pub struct CreatePartyEvent;
pub struct JoinRoomEvent(String);

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreatePartyEvent>()
            .add_event::<JoinRoomEvent>()
            .add_system(create_party)
            .add_system(join_room);
    }
}

fn join_room(
    mut join_ev: EventReader<JoinRoomEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in join_ev.iter() {
        println!("join room");
        next_state.set(GameState::Waiting);
    }
}

#[actix_rt::main]
async fn create_party(
    mut create_ev: EventReader<CreatePartyEvent>,
    mut join_ev: EventWriter<JoinRoomEvent>,
) {
    for _ in create_ev.iter() {
        let (_res, mut ws) = Client::new()
            .ws("ws://127.0.0.1:8080/ws/")
            .connect()
            .await
            .unwrap();

        let create_room_json = serde_json::to_string(&RoomAction::CreateRoom).unwrap();
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
                                RoomAction::RoomCreated(room_id) => println!("room id {}", room_id),
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
}
