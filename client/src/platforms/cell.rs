use bevy::prelude::*;

use crate::{
    path_finding::FindPath,
    player::{cannon::ShootCannon, input::InputSystem, player::PlayerControl, Player},
};

use super::{MovementPlatform, TargetPlatform};

#[derive(Component)]
pub struct GameCell;

pub const CELL_SIZE: f32 = 100.;

// pub fn update_cell(
//     mut cell: Query<(&Transform, &mut Sprite, Entity), (With<MovementPlatform>, Without<Player>)>,
//     mut target_cell: Query<
//         (&Transform, &mut Sprite, Entity),
//         (
//             With<TargetPlatform>,
//             Without<MovementPlatform>,
//             Without<Player>,
//         ),
//     >,
//     mut player: Query<(&mut Transform, &mut InputSystem, &mut Player)>,
//     mut commands: Commands,
//     mut shoot_event: EventWriter<ShootCannon>,
//     mut find_path_event: EventWriter<FindPath>,
// ) {
//     let (player_pos, mut input, mut player) = player.single_mut();
//
//     for (transform, mut sprite, entity) in cell.iter_mut() {
//         if let Some(position) = input.destination.position {
//             if is_selected(&transform.translation, &position) {
//                 let cell = commands
//                     .entity(entity)
//                     .insert(selection_overlay(
//                         &Color::rgba_u8(0, 255, 0, 200),
//                         &transform.translation,
//                     ))
//                     .id();
//
//                 input.destination.id = Some(cell);
//
//                 if !player.finding_path {
//                     find_path_event.send(FindPath {
//                         start: Vec2::new(player_pos.translation.x, player_pos.translation.y),
//                         goal: Vec2::new(transform.translation.x, transform.translation.y),
//                     });
//                     player.finding_path = true;
//                 }
//             }
//         }
//         sprite.color = Color::rgba_u8(117, 92, 71, 255);
//     }
//
//     for (transform, mut sprite, entity) in target_cell.iter_mut() {
//         if let Some(position) = input.target.position {
//             if is_selected(&transform.translation, &position) {
//                 let cell_id = commands
//                     .entity(entity)
//                     .insert(selection_overlay(
//                         &Color::rgba_u8(200, 10, 50, 200),
//                         &transform.translation,
//                     ))
//                     .id();
//
//                 input.target.id = Some(cell_id);
//
//                 if player.cannon_ready {
//                     shoot_event.send(ShootCannon);
//                 }
//                 return;
//             }
//         }
//         sprite.color = Color::rgba_u8(117, 92, 71, 255);
//     }
// }

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

pub fn target_cell(commands: &mut Commands, cells: (&mut Transform, &mut Sprite), position: Vec2) {}
