use bevy::{
    prelude::*,
    window::{close_on_esc, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use platforms::PlatformsPlugin;
use player::PlayerPlugin;

mod platforms;
mod player;

pub const WINDOW_WIDTH: f32 = 1280.;
pub const WINDOW_HEIGHT: f32 = 720.;

#[derive(Component)]
pub struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pi-Rats".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_plugin(PlatformsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..default()
        },
        ..default()
    });
}
