use bevy::{
    prelude::{Commands, Plugin, Query, Transform, Vec2},
    sprite::Sprite,
};

use self::{platform::Platform, platform_cell::PlatformSquare};

use super::input::InputSystem;

mod platform;
mod platform_cell;
mod utils;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup).add_system(handle_input);
    }
}

fn setup(mut commands: Commands) {
    let platform1 = Platform::new(Vec2::new(-200., 0.), true, false, &mut commands);
    let platform2 = Platform::new(Vec2::new(200., 0.), false, true, &mut commands);

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
