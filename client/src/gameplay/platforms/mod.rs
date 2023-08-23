use bevy::{
    prelude::{
        Commands, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin, Query,
        Transform, Vec2,
    },
    sprite::Sprite,
    window::Window,
};

use crate::GameState;

use self::{
    platform::Platform,
    platform_cell::{PlatformSquare, CELL_SIZE},
};

use super::input::InputSystem;

mod platform;
mod platform_cell;
mod utils;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Waiting)))
            .add_system(handle_input.in_set(OnUpdate(GameState::GamePlay)));
    }
}

fn setup(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();

    let platform1 = Platform::new(
        Vec2::new(
            -((window.width() / 2.) - CELL_SIZE / 2.),
            -Platform::height() / 2.,
        ),
        true,
        false,
        &mut commands,
    );
    let platform2 = Platform::new(
        Vec2::new(
            (window.width() / 2.) - Platform::width() - CELL_SIZE * 1.5,
            -Platform::height() / 2.,
        ),
        false,
        true,
        &mut commands,
    );

    commands.spawn(platform1);
    commands.spawn(platform2);
}

fn handle_input(
    mut input: Query<&mut InputSystem>,
    mut platforms: Query<&mut Platform>,
    mut cells: Query<(&PlatformSquare, &Transform, &mut Sprite)>,
) {
    let input = input.single_mut();

    for platform in platforms.iter_mut() {
        platform.highlight_hovered(input.cursor_pos, &mut cells);
    }
}
