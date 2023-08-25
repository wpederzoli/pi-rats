use bevy::{
    prelude::{Bundle, Color, Component, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use super::map::map_component::MapComponent;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    graphics: SpriteBundle,
}

impl PlayerBundle {
    pub fn new() -> Self {
        PlayerBundle {
            graphics: SpriteBundle::default(),
        }
    }
}

impl MapComponent for PlayerBundle {
    fn init(&mut self, x: f32, y: f32) -> Self {
        self.graphics.sprite.color = Color::WHITE;
        self.graphics.transform.translation = Vec3::new(x, y, 3.);
        PlayerBundle {
            graphics: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(20., 20.)),
                    ..default()
                },
                transform: Transform::from_xyz(x + 10., y - 10., 4.),
                ..default()
            },
        }
    }
}
