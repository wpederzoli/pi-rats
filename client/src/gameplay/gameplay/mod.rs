use bevy::{
    prelude::{
        Camera, Commands, GlobalTransform, Input, MouseButton, Plugin, Query, Res, Vec2, With,
    },
    window::Window,
};

use crate::MainCamera;

pub struct GamePlay;

mod background;
mod map;
mod platform;
mod players;

impl Plugin for GamePlay {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup).add_system(handle_input);
    }
}

fn setup(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    commands.spawn(background::BackgroundBundle::new(
        window.width(),
        window.height(),
    ));

    let map = map::Map::new(
        10,
        10,
        Vec2::new(window.width() / 10., window.height() / 10.),
    );

    map.spawn(
        &mut commands,
        &mut platform::PlatformBundle::new(map.cell_size.x * 3., map.cell_size.y * 5.),
        -4,
        2,
    );

    map.spawn(&mut commands, &mut players::PlayerBundle::new(), -4, 2);

    map.spawn(
        &mut commands,
        &mut platform::PlatformBundle::new(map.cell_size.x * 3., map.cell_size.y * 5.),
        1,
        2,
    );
}

fn handle_input(
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_input: Res<Input<MouseButton>>,
    mut platform: Query<&mut Platform>,
) {
    let (camera, camera_transform) = camera.single();

    if let Some(world_position) = window
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {}

    // if mouse_input.pressed(MouseButton::Left) {
    //     if let Some(world_position) = window
    //         .single()
    //         .cursor_position()
    //         .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    //         .map(|ray| ray.origin.truncate())
    //     {
    //         println!("mouse position: {}", world_position);
    //     }
    // }
}
