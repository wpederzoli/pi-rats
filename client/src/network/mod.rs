use bevy::prelude::*;
use common::RoomAction;

use crate::GameState;

use self::room::connect_to_server;

mod room;

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
