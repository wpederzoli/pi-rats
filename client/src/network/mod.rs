use std::str::from_utf8;

use awc::{ws, Client};
use bevy::prelude::*;
use common::RoomAction;
use futures_util::{SinkExt as _, StreamExt as _};

use crate::GameState;

pub struct NetworkPlugin;
pub struct CreatePartyEvent(pub String);
pub struct JoinRoomEvent(pub String);

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreatePartyEvent>()
            .add_event::<JoinRoomEvent>()
            .add_system(create_party);
    }
}

pub async fn connect_to_server(room_action: RoomAction) -> RoomAction {
    let (_res, mut ws) = Client::new()
        .ws("ws://127.0.0.1:8080/ws/")
        .connect()
        .await
        .unwrap();

    let json_msg = serde_json::to_string(&room_action).unwrap();
    ws.send(ws::Message::Text(json_msg.into())).await.unwrap();

    let mut response = RoomAction::Invalid("".to_string());

    loop {
        if let Ok(ws_msg) = ws.next().await.unwrap() {
            match ws_msg {
                ws::Frame::Text(msg) => match from_utf8(&msg) {
                    Ok(txt) => {
                        response = serde_json::from_str(txt).unwrap();
                    }
                    _ => response = RoomAction::Invalid("Unable to create room".to_string()),
                },
                _ => response = RoomAction::Invalid("Something went wrong".to_string()),
            }
        }
        break;
    }

    response
}

#[actix_rt::main]
async fn create_party(
    mut create_ev: EventReader<CreatePartyEvent>,
    mut join_ev: EventReader<JoinRoomEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for create_ev in create_ev.iter() {
        let r = connect_to_server(RoomAction::CreateRoom(create_ev.0.clone())).await;
        match r {
            RoomAction::RoomCreated => next_state.set(GameState::Waiting),
            _ => info!("Room no created"),
        }
    }

    for join_ev in join_ev.iter() {
        let r = connect_to_server(RoomAction::JoinRoom(join_ev.0.clone())).await;
        match r {
            RoomAction::JoinedRoom => next_state.set(GameState::Waiting),
            _ => info!("Unable to join room"),
        }
    }
}
