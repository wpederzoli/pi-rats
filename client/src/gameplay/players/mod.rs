use bevy::prelude::{Commands, Input, MouseButton, Plugin, Query, Res, Transform};

use self::player::{Player, PlayerBundle};
use super::input::InputSystem;

mod coordinates;
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
    mouse: Res<Input<MouseButton>>,
) {
    let (mut player, mut input, mut transform) = player.single_mut();

    if mouse.just_pressed(MouseButton::Left) && !input.left_click() {
        input.set_left_click(true);
        player.coordinates.set_destination(input.cursor_pos);
    }

    if mouse.just_pressed(MouseButton::Right) && !input.right_click() {
        input.set_right_click(true);
        player.coordinates.set_target(input.cursor_pos);
    }

    player.update(&mut transform);
}
