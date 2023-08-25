use bevy::{
    prelude::{Bundle, Color, Component, Transform, Vec2},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use super::map::map_component::MapComponent;
//We can remove width and height from component
#[derive(Component, Clone, Copy)]
pub struct Platform {
    width: f32,
    height: f32,
}

#[derive(Bundle)]
pub struct PlatformBundle {
    #[bundle]
    graphics: SpriteBundle,
    platform: Platform,
}

impl PlatformBundle {
    pub fn new(width: f32, height: f32) -> Self {
        PlatformBundle {
            graphics: SpriteBundle::default(),
            platform: Platform { width, height },
        }
    }
}

impl MapComponent for PlatformBundle {
    fn init(&mut self, x: f32, y: f32) -> Self {
        let graphics = SpriteBundle {
            sprite: Sprite {
                color: Color::OLIVE,
                custom_size: Some(Vec2::new(self.platform.width, self.platform.height)),
                ..default()
            },
            transform: Transform::from_xyz(
                x + self.platform.width / 2.,
                y - self.platform.height / 2.,
                3.,
            ),
            ..default()
        };

        self.graphics = graphics.clone();

        PlatformBundle {
            graphics,
            platform: self.platform,
        }
    }
}
