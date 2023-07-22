use bevy::prelude::{info, ResMut};
use bevy_matchbox::{prelude::SingleChannel, MatchboxSocket};

pub fn wait_for_players(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    if socket.get_channel(0).is_err() {
        println!("players ready");
        return;
    }

    socket.update_peers();
    let players = socket.players();

    if players.len() < 2 {
        return;
    }

    info!("All players joined");
}
