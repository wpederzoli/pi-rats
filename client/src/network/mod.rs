use bevy::prelude::{info, Commands, EventReader, NextState, Plugin, ResMut};
use bevy_matchbox::MatchboxSocket;

use crate::GameState;

pub mod room;

pub struct NetworkPlugin;

pub struct CreateRoomEvent;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CreateRoomEvent>()
            .add_system(start_matchbox_socket);
    }
}

fn start_matchbox_socket(
    mut commands: Commands,
    mut events: EventReader<CreateRoomEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in events.iter() {
        println!("event");
        let room_url = "ws://127.0.0.1:8080/ws/";
        info!("connecting to matchbox server: {:?}", room_url);
        commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
        next_state.set(GameState::GamePlay);
    }
}
