use bevy::{
    prelude::{
        Commands, Input, IntoSystemAppConfig, IntoSystemConfig, MouseButton, OnEnter, OnUpdate,
        Plugin, Query, Res, Transform,
    },
    window::Window,
};

use crate::{a_star, platforms::cell::CELL_SIZE, GameState};

use self::player::{Player, PlayerBundle};
use super::{input::InputSystem, platforms};

mod coordinates;
mod player;

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Waiting)))
            .add_system(handle_input.in_set(OnUpdate(GameState::GamePlay)));
    }
}

fn setup(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();

    let player1 = PlayerBundle::new(
        (-window.width() / 2.) + CELL_SIZE + PlayerBundle::PLAYER_SIZE.x,
        (window.height() / 2.) - CELL_SIZE,
    );
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
        // let destination = platforms::utils::get_cell_center(input.cursor_pos);
        player.coordinates.set_destination(input.cursor_pos);
        // let a_star::find_path(player.coordinates.destination.unwrap());
    }

    if mouse.just_pressed(MouseButton::Right) && !input.right_click() {
        input.set_right_click(true);
        player.coordinates.set_target(input.cursor_pos);
    }

    player.update(&mut transform);
}
