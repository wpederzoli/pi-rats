use bevy::prelude::*;

use crate::{GameState, WINDOW_HEIGHT, WINDOW_WIDTH};

use self::cell::{update_cell, GameCell, CELL_SIZE};

pub mod cell;

pub const PLATFORM_LAYER: f32 = 1.;
pub const PLATFORM_WIDTH: u8 = 3;
pub const PLATFORM_HEIGHT: u8 = 5;

#[derive(Component)]
pub struct MovementPlatform;
#[derive(Component)]
pub struct TargetPlatform;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::GamePlay)))
            .add_system(update_cell.in_set(OnUpdate(GameState::GamePlay)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..default()
        },
        ..default()
    });
    for i in 1..=PLATFORM_WIDTH {
        for j in 1..=PLATFORM_HEIGHT {
            commands
                .spawn(create_platform(
                    (-WINDOW_WIDTH / 2.) + CELL_SIZE * i as f32,
                    (WINDOW_HEIGHT / 2.) - CELL_SIZE * j as f32,
                    Vec2::new(CELL_SIZE, CELL_SIZE),
                ))
                .insert(MovementPlatform);
        }
    }
    for i in 1..=PLATFORM_WIDTH {
        for j in 1..=PLATFORM_HEIGHT {
            commands
                .spawn(create_platform(
                    (WINDOW_WIDTH / 2.) - CELL_SIZE * i as f32,
                    (WINDOW_HEIGHT / 2.) - CELL_SIZE * j as f32,
                    Vec2::new(CELL_SIZE, CELL_SIZE),
                ))
                .insert(TargetPlatform);
        }
    }
}

fn create_platform(x: f32, y: f32, size: Vec2) -> (SpriteBundle, GameCell) {
    (
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba_u8(12, 123, 234, 255),
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_xyz(x, y, PLATFORM_LAYER),
            ..default()
        },
        GameCell,
    )
}
