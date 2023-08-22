use bevy::{
    prelude::{default, Bundle, Color, Component, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};

use super::Player;

#[derive(Component)]
pub struct PlayerControl;

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    sprite: SpriteBundle,
    player: Player,
}

impl PlayerBundle {
    pub fn new(position: Vec3) -> Self {
        let sprite = SpriteBundle {
            sprite: Sprite {
                color: Color::rgba_u8(0, 255, 217, 255),
                custom_size: Some(Vec2::new(50., 70.)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, position.z),
            ..default()
        };
        let player = Player {
            cannon_ready: false,
            finding_path: false,
            steps: Vec::new(),
        };

        PlayerBundle { sprite, player }
    }
}
