use bevy::prelude::{info, Commands, EventReader, NextState, Plugin, ResMut, Resource};
use bevy_matchbox::MatchboxSocket;

use crate::GameState;

pub mod room;

pub struct NetworkPlugin;

#[derive(Resource)]
pub struct GameNetwork {
    pub player_one: bool,
    pub player_two: bool,
}

impl GameNetwork {
    pub fn new(player_one: bool, player_two: bool) -> Self {
        GameNetwork {
            player_one,
            player_two,
        }
    }
}

pub struct CreateRoomEvent;
pub struct JoinRoomEvent;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CreateRoomEvent>()
            .add_event::<JoinRoomEvent>()
            .add_systems((start_matchbox_socket, join_matchbox_socket));
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
        commands.insert_resource(GameNetwork::new(true, false));
        next_state.set(GameState::Waiting);
    }
}

fn join_matchbox_socket(
    mut commands: Commands,
    mut events: EventReader<JoinRoomEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in events.iter() {
        println!("event");
        let room_url = "ws://127.0.0.1:8080/ws/";
        info!("connecting to matchbox server: {:?}", room_url);
        commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
        commands.insert_resource(GameNetwork::new(true, true));
        next_state.set(GameState::Waiting);
    }
}
