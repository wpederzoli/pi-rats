use bevy::prelude::*;

use crate::player::{cannon::ShootCannon, input::InputSystem, Player, PLAYER_SPEED};

#[derive(Component)]
pub struct GameCell;

pub const CELL_SIZE: f32 = 100.;

pub fn update_cell(
    mut cell: Query<(&Transform, &mut Sprite, Entity), (With<GameCell>, Without<Player>)>,
    mut player: Query<(&mut Transform, &InputSystem, &Player)>,
    mut commands: Commands,
    mut events: EventWriter<ShootCannon>,
) {
    let (mut player_pos, input, player) = player.single_mut();

    for (transform, mut sprite, entity) in cell.iter_mut() {
        if let Some(destination) = input.destination {
            if is_selected(&transform.translation, &destination) {
                commands.entity(entity).insert(selection_overlay(
                    &Color::rgba_u8(0, 255, 0, 200),
                    &transform.translation,
                ));

                match &mut player_pos.translation {
                    mut pos if pos.x > transform.translation.x => pos.x -= PLAYER_SPEED,
                    mut pos if pos.x < transform.translation.x => pos.x += PLAYER_SPEED,
                    mut pos if pos.y > transform.translation.y => pos.y -= PLAYER_SPEED,
                    mut pos if pos.y < transform.translation.y => pos.y += PLAYER_SPEED,
                    _ => (),
                }
            }
        }

        if let Some(target) = input.target {
            if is_selected(&transform.translation, &target) {
                commands.entity(entity).insert(selection_overlay(
                    &Color::rgba_u8(200, 10, 50, 200),
                    &transform.translation,
                ));

                if player.cannon_ready {
                    events.send(ShootCannon);
                }
                return;
            }
        }
        sprite.color = Color::rgba_u8(117, 92, 71, 255); //TODO: Not run every frame.
    }
}

fn is_selected(cell_position: &Vec3, destination: &Vec2) -> bool {
    cell_position.x - CELL_SIZE / 2. <= destination.x
        && cell_position.x + CELL_SIZE / 2. >= destination.x
        && cell_position.y - CELL_SIZE / 2. <= destination.y
        && cell_position.y + CELL_SIZE / 2. >= destination.y
}

fn selection_overlay(color: &Color, position: &Vec3) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: *color,
            custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
            ..default()
        },
        transform: Transform::from_xyz(position.x, position.y, position.z),
        ..default()
    }
}
