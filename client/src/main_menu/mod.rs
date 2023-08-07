use bevy::prelude::*;

mod main_menu;
mod modal;
mod ui;

use crate::GameState;

use self::{
    main_menu::{button_system, MainMenu},
    modal::{handle_input, modal_button_system},
    ui::create_main_menu,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            setup_menu.in_schedule(OnEnter(GameState::MainMenu)),
            cleanup_menu.in_schedule(OnExit(GameState::MainMenu)),
            button_system.in_set(OnUpdate(GameState::MainMenu)),
            modal_button_system.in_set(OnUpdate(GameState::MainMenu)),
            handle_input.in_set(OnUpdate(GameState::MainMenu)),
        ));
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    create_main_menu(&mut commands, font);
}

fn cleanup_menu(mut commands: Commands, main_menu: Query<Entity, With<MainMenu>>) {
    let main_menu = main_menu.single();
    commands.entity(main_menu).despawn_recursive();
}
