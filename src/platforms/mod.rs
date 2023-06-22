use bevy::prelude::*;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

const PLATFORM_WIDTH: f32 = 300.;
const PLATFORM_HEIGHT: f32 = 600.;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    let x_size = 3.;
    let y_size = 5.;
    for i in 1..=x_size as u32 {
        for j in 1..=y_size as u32 {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::ORANGE_RED,
                    custom_size: Some(Vec2::new(PLATFORM_WIDTH / x_size, PLATFORM_HEIGHT / y_size)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    (-WINDOW_WIDTH / 2.) + (PLATFORM_WIDTH / x_size) * i as f32,
                    (WINDOW_HEIGHT / 2.) - (PLATFORM_HEIGHT / y_size) * j as f32,
                    1.,
                ),
                ..default()
            });
        }
    }
}
