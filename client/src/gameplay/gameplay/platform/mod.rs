use bevy::{
    prelude::{Bundle, Color, Component, Transform, Vec2},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use super::map::map_component::MapComponent;
//We can remove width and height from component
#[derive(Component, Clone, Copy)]
pub struct Platform {
    walkable: bool,
    shootable: bool,
}

#[derive(Bundle)]
pub struct PlatformBundle {
    #[bundle]
    graphics: SpriteBundle,
    platform: Platform,
}

impl PlatformBundle {
    pub fn new(width: f32, height: f32, walkable: bool, shootable: bool) -> Self {
        let mut sprite_bundle = SpriteBundle::default();
        sprite_bundle.sprite.custom_size = Some(Vec2::new(width, height));

        PlatformBundle {
            graphics: sprite_bundle,
            platform: Platform {
                walkable,
                shootable,
            },
        }
    }
}

impl MapComponent for PlatformBundle {
    fn init(&mut self, x: f32, y: f32) -> Self {
        let size = self.graphics.sprite.custom_size.unwrap();

        let graphics = SpriteBundle {
            sprite: Sprite {
                color: Color::OLIVE,
                custom_size: Some(Vec2::new(size.x, size.y)),
                ..default()
            },
            transform: Transform::from_xyz(x + size.x / 2., y - size.y / 2., 3.),
            ..default()
        };

        self.graphics = graphics.clone();

        PlatformBundle {
            graphics,
            platform: self.platform,
        }
    }
}
