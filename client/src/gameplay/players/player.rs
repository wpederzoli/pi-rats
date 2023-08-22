use bevy::{
    prelude::{Bundle, Color, Component, Transform, Vec2},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use super::coordinates::Coordinates;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Player {
    pub coordinates: Coordinates,
}

impl PlayerBundle {
    pub fn new(x: f32, y: f32) -> Self {
        PlayerBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(20., 20.)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 5.),
                ..default()
            },
            player: Player {
                coordinates: Coordinates::default(),
            },
        }
    }
}

impl Player {
    pub fn update(&mut self, transform: &mut Transform) {
        if let Some(destination) = self.coordinates.destination {
            transform.translation.x += 1.;
        }
    }
}
