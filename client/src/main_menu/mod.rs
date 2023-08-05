use bevy::prelude::*;

mod main_menu;
mod modal;
mod ui;

use crate::{
    network::{CreatePartyEvent, JoinRoomEvent},
    GameState, WINDOW_HEIGHT, WINDOW_WIDTH,
};

use self::{
    main_menu::{MenuButton, MenuButtonType},
    modal::{create_input_field, handle_input},
    ui::{create_main_menu, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
};

pub struct MainMenuPlugin;

#[derive(Component)]
struct MainMenu;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::MainMenu)))
            .add_system(button_system.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(handle_input.in_set(OnUpdate(GameState::MainMenu)));
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    create_main_menu(&mut commands, font);
}

//TODO: Move logic to main menu
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut join_event: EventWriter<JoinRoomEvent>,
    asset_server: Res<AssetServer>,
    menu: Query<Entity, With<MainMenu>>,
) {
    for menu in menu.iter() {
        for (interaction, mut color, button) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    match button.button_type {
                        MenuButtonType::StartButton => next_state.set(GameState::GamePlay),
                        MenuButtonType::CreateParty => {
                            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                            let input_field = create_input_field(&mut commands, font);
                            commands.entity(menu).push_children(&[input_field]);
                        }
                        MenuButtonType::JoinParty => {
                            join_event.send(JoinRoomEvent("test room".to_string()))
                        }
                    }
                    *color = PRESSED_BUTTON_COLOR.into();
                }
                Interaction::Hovered => {
                    *color = BackgroundColor(HOVERED_BUTTON_COLOR);
                }
                _ => *color = BackgroundColor(NORMAL_BUTTON_COLOR),
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, main_menu: Query<Entity, With<MainMenu>>) {
    let main_menu = main_menu.single();
    commands.entity(main_menu).despawn_recursive();
}
