use bevy::prelude::*;

use super::Player;

const CANNONBALL_LAYER: f32 = 3.;

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
