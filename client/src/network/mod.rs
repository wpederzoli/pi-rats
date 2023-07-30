use awc::{ws, Client};
use bevy::prelude::*;
use futures_util::{SinkExt as _, StreamExt as _};

use crate::GameState;

pub struct NetworkPlugin;
pub struct CreatePartyEvent;
pub struct JoinRoomEvent;

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

        ws.send(ws::Message::Text("create party".into()))
            .await
            .unwrap();

        loop {
            if let Ok(ws_msg) = ws.next().await.unwrap() {
                //if message OK ROOM CREATED
                println!("ws: {:?}", ws_msg);
                join_ev.send(JoinRoomEvent);
                break;
            }
        }
    }
}
