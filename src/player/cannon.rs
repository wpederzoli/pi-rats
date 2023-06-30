use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::platforms::{cell::CELL_SIZE, PLATFORM_LAYER};

use super::{input::InputSystem, Player};

const CANNONBALL_LAYER: f32 = 3.;
const CANNONBALL_SPEED: f32 = 10.;
const CANNONBALL_SIZE: Vec2 = Vec2::new(20., 20.);

#[derive(Component)]
pub struct Cannonball;

pub struct ShootCannon;

pub fn spawn_cannonball(position: Vec2) -> (SpriteBundle, Cannonball) {
    (
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(CANNONBALL_SIZE),
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
    mut cannonball: Query<(&mut Transform, Entity), With<Cannonball>>,
    mut input: Query<&mut InputSystem>,
    mut player: Query<&mut Player>,
    mut commands: Commands,
) {
    for mut input in input.iter_mut() {
        if let Some(target) = input.target {
            for (mut position, entity) in cannonball.iter_mut() {
                if let Some(_) = collide(
                    position.translation,
                    CANNONBALL_SIZE,
                    Vec3::new(target.x, target.y, PLATFORM_LAYER),
                    Vec2::new(CELL_SIZE, CELL_SIZE),
                ) {
                    commands.entity(entity).despawn();
                    input.target = None;
                    player.single_mut().cannon_ready = true;
                } else {
                    let mut pos = position.clone();
                    pos.look_at(
                        Vec3::new(target.x, target.y, CANNONBALL_LAYER),
                        Vec3::new(position.translation.x, position.translation.y, -1.),
                    );
                    position.translation = position.translation + pos.forward() * CANNONBALL_SPEED;
                }
            }
        }
    }
}
