use bevy::prelude::*;

pub mod cannon;
pub mod input;
// mod path_finding;

use crate::{path_finding::Path, WINDOW_HEIGHT, WINDOW_WIDTH};

use self::{
    cannon::{move_cannonball, shoot_cannon, ShootCannon},
    input::{input_system, InputSystem},
};
pub const PLAYER_LAYER: f32 = 2.;
pub const PLAYER_SPEED: f32 = 2.;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub cannon_ready: bool,
    pub finding_path: bool,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootCannon>()
            .add_startup_system(setup)
            .add_system(input_system)
            .add_system(shoot_cannon)
            .add_system(move_cannonball);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba_u8(0, 255, 217, 255),
                custom_size: Some(Vec2::new(50., 70.)),
                ..default()
            },
            transform: Transform::from_xyz(
                (-WINDOW_WIDTH / 2.) + 120.,
                WINDOW_HEIGHT / 2. - 120.,
                PLAYER_LAYER,
            ),
            ..default()
        },
        Player {
            cannon_ready: true,
            finding_path: false,
        },
        InputSystem::default(),
        Path::default(),
    ));
}
