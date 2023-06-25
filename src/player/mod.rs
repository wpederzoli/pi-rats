use bevy::prelude::*;

pub mod input;
#[path = "../platforms/mod.rs"]
pub mod platforms;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use self::input::{input_system, InputSystem};
pub const PLAYER_LAYER: f32 = 2.;
pub const PLAYER_SPEED: f32 = 2.;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(input_system);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GREEN,
                custom_size: Some(Vec2::new(50., 70.)),
                ..default()
            },
            transform: Transform::from_xyz(
                (-WINDOW_WIDTH / 2.) + 120.,
                WINDOW_HEIGHT / 2. - 120.,
                2.,
            ),
            ..default()
        },
        Player,
        InputSystem {
            destination: Vec2::new((-WINDOW_WIDTH / 2.) + 120., WINDOW_HEIGHT / 2. - 120.),
        },
    ));
}
