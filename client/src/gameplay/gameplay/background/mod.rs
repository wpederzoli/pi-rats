use bevy::{
    prelude::{default, Bundle, Color, Vec2},
    sprite::{Sprite, SpriteBundle},
};

#[derive(Bundle)]
pub struct BackgroundBundle {
    #[bundle]
    sprite: SpriteBundle,
}

impl BackgroundBundle {
    pub fn new(width: f32, height: f32) -> Self {
        BackgroundBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                ..default()
            },
        }
    }
}
