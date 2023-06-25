use bevy::{
    prelude::{Input, *},
    window::PrimaryWindow,
};

use crate::MainCamera;

use super::Player;

#[derive(Component)]
pub struct InputSystem {
    pub destination: Vec2,
}

pub fn input_system(
    mut player: Query<&mut InputSystem, With<Player>>,
    mouse_input: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let mut input = player.single_mut();
    let (camera, camera_transform) = camera.single();

    if let Some(world_position) = window
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if mouse_input.pressed(MouseButton::Left) {
            input.destination = world_position;
        }
    }
}