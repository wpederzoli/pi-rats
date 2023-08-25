use bevy::{
    prelude::*,
    window::{close_on_esc, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use gameplay::GamePlayPlugin;
use main_menu::MainMenuPlugin;
use network::websocket::WebSocketPlugin;
use path_finding::PathFindingPlugin;

mod a_star;
mod gameplay;
mod main_menu;
mod network;
mod path_finding;

mod platforms;
mod player;

pub const WINDOW_WIDTH: f32 = 1280.;
pub const WINDOW_HEIGHT: f32 = 720.;

#[derive(Component)]
pub struct MainCamera;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Waiting,
    GamePlay,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pi-Rats".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .with_scale_factor_override(1.),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_state::<GameState>()
        .add_startup_system(setup_camera)
        .add_plugins(GamePlayPlugin)
        .add_plugin(WebSocketPlugin)
        // .add_plugin(PathFindingPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MainMenuPlugin)
        .add_system(close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
