use bevy::{
    prelude::{Camera, GlobalTransform, IntoSystemConfig, OnUpdate, Plugin, Query, With},
    window::{PrimaryWindow, Window},
};

mod input;

use crate::{GameState, MainCamera};

pub use self::input::InputSystem;

pub struct InputSystemPlugin;

impl Plugin for InputSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(update_cursor.in_set(OnUpdate(GameState::GamePlay)));
    }
}

fn update_cursor(
    mut input_system: Query<&mut InputSystem>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let mut input = input_system.single_mut();
    let (camera, camera_transform) = camera.single();

    if let Some(world_position) = window
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        input.cursor_pos = world_position;
    }
}
