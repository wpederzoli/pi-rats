use bevy::prelude::{info, Commands};
use bevy_matchbox::MatchboxSocket;

const URL: &str = "ws://127.0.0.1:8080/ws/";

pub fn create_room(commands: &mut Commands) {
    info!("connecting to matchbox server: {:?}", URL);
    commands.insert_resource(MatchboxSocket::new_ggrs(URL));
}
