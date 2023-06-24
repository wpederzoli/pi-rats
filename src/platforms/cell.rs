use bevy::prelude::*;

use crate::player::{input::InputSystem, Player, PLAYER_LAYER};

#[derive(Component)]
pub struct GameCell;

pub const CELL_SIZE: f32 = 100.;

pub fn update_cell(
    cell: Query<&Transform, (With<GameCell>, Without<Player>)>,
    mut player: Query<(&mut Transform, &InputSystem), With<Player>>,
) {
    let (mut player_pos, input) = player.single_mut();

    for transform in cell.iter() {
        if transform.translation.x - CELL_SIZE / 2. <= input.destination.x
            && transform.translation.x + CELL_SIZE / 2. >= input.destination.x
            && transform.translation.y - CELL_SIZE / 2. <= input.destination.y
            && transform.translation.y + CELL_SIZE / 2. >= input.destination.y
        {
            player_pos.translation = Vec3::new(
                transform.translation.x,
                transform.translation.y,
                PLAYER_LAYER,
            );
        }
    }
}
