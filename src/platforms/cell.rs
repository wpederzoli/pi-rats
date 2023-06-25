use bevy::prelude::*;

use crate::player::{input::InputSystem, Player, PLAYER_SPEED};

#[derive(Component)]
pub struct GameCell;

pub const CELL_SIZE: f32 = 100.;

pub fn update_cell(
    mut cell: Query<(&Transform, &mut Sprite), (With<GameCell>, Without<Player>)>,
    mut player: Query<(&mut Transform, &InputSystem), With<Player>>,
) {
    let (mut player_pos, input) = player.single_mut();

    for (transform, mut sprite) in cell.iter_mut() {
        if transform.translation.x - CELL_SIZE / 2. <= input.destination.x
            && transform.translation.x + CELL_SIZE / 2. >= input.destination.x
            && transform.translation.y - CELL_SIZE / 2. <= input.destination.y
            && transform.translation.y + CELL_SIZE / 2. >= input.destination.y
        {
            sprite.color = Color::rgba_u8(0, 255, 90, 255);
            match &mut player_pos.translation {
                mut pos if pos.x > transform.translation.x => pos.x -= PLAYER_SPEED,
                mut pos if pos.x < transform.translation.x => pos.x += PLAYER_SPEED,
                mut pos if pos.y > transform.translation.y => pos.y -= PLAYER_SPEED,
                mut pos if pos.y < transform.translation.y => pos.y += PLAYER_SPEED,
                _ => (),
            }
        } else {
            sprite.color = Color::rgba_u8(117, 92, 71, 255);
        }
    }
}
