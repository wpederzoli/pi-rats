use bevy::{
    prelude::{Input, *},
    window::PrimaryWindow,
};

use crate::{
    platforms::{platform::GameCell, platform::Platform},
    MainCamera,
};

use super::{player::PlayerControl, Player};

pub struct Target {
    pub position: Option<Vec2>,
}

impl Target {
    pub fn none() -> Self {
        Target { position: None }
    }
}

pub struct SelectCell(pub Vec2);

#[derive(Component)]
pub struct InputSystem {
    pub destination: Target,
    pub target: Target,
}

impl Default for InputSystem {
    fn default() -> Self {
        InputSystem {
            destination: Target { position: None },
            target: Target { position: None },
        }
    }
}

pub fn input_system(
    mut player: Query<&mut InputSystem>,
    mouse_input: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut cells: Query<(&mut Sprite, &Transform), With<GameCell>>,
    mut platforms: Query<&mut Platform>,
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
            input.destination.position = Some(world_position);
        }

        if mouse_input.pressed(MouseButton::Right) {
            if let None = input.target.position {
                for mut platform in platforms.iter_mut() {
                    if platform.shootable {
                        input.target = platform
                            .select_by_position(world_position, &mut cells)
                            .unwrap();
                    }
                }
            }
        }
    }
}
