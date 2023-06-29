use bevy::prelude::*;

use super::{input::InputSystem, Player};

const CANNONBALL_LAYER: f32 = 3.;
const CANNONBALL_SPEED: f32 = 10.;

#[derive(Component)]
pub struct Cannonball;

pub struct ShootCannon;

pub fn spawn_cannonball(position: Vec2) -> (SpriteBundle, Cannonball) {
    (
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(20., 20.)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, CANNONBALL_LAYER),
            ..default()
        },
        Cannonball,
    )
}

pub fn shoot_cannon(
    mut listener: EventReader<ShootCannon>,
    mut commands: Commands,
    mut player: Query<(&Transform, &mut Player)>,
) {
    for _ in listener.iter() {
        let (transform, mut player) = player.single_mut();
        commands.spawn(spawn_cannonball(Vec2::new(
            transform.translation.x,
            transform.translation.y,
        )));
        player.cannon_ready = false;
    }
}

pub fn move_cannonball(
    mut cannonball: Query<&mut Transform, With<Cannonball>>,
    target: Query<&InputSystem>,
) {
    for input in target.iter() {
        for mut position in cannonball.iter_mut() {
            if position.translation.x > input.target.x {
                position.translation.x -= CANNONBALL_SPEED;
            }
            if position.translation.x < input.target.x {
                position.translation.x += CANNONBALL_SPEED;
            }
            if position.translation.y > input.target.y {
                position.translation.y -= CANNONBALL_SPEED;
            }
            if position.translation.y < input.target.y {
                position.translation.y += CANNONBALL_SPEED;
            }
        }
    }
}
