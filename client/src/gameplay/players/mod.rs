use bevy::{
    prelude::{Commands, EventReader, Input, MouseButton, Plugin, Query, Res, Transform},
    window::CursorMoved,
};

use self::{
    input::InputSystem,
    player::{Player, PlayerBundle},
};

mod coordinates;
mod gameplay_helpers;
mod input;
mod player;

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup).add_system(handle_input);
    }
}

fn setup(mut commands: Commands) {
    let player1 = PlayerBundle::new(-400., -100.);
    let player2 = PlayerBundle::new(200., 100.);

    commands.spawn((player1, InputSystem::new()));
    commands.spawn(player2);
}

fn handle_input(
    mut player: Query<(&mut Player, &mut InputSystem, &mut Transform)>,
    cursor_ev: EventReader<CursorMoved>,
    mouse: Res<Input<MouseButton>>,
) {
    let (mut player, mut input, mut transform) = player.single_mut();
    input.update(cursor_ev, mouse);

    gameplay_helpers::handle_mouse_clicks(&mut player, &mut input);
    player.update(&mut transform);
}
