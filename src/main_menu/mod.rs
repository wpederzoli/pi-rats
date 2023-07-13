use bevy::prelude::*;

mod main_menu;

use crate::{GameState, WINDOW_HEIGHT, WINDOW_WIDTH};

use self::main_menu::{
    create_button, create_container, create_text, MenuButton, MenuButtonType, MENU_SIZE,
};

pub struct MainMenuPlugin;

#[derive(Component)]
struct MainMenu;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::MainMenu)))
            .add_system(button_system.in_set(OnUpdate(GameState::MainMenu)));
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_position = UiRect::new(
        Val::Px((WINDOW_WIDTH / 2.) - MENU_SIZE / 2.),
        Val::Px(0.),
        Val::Px((WINDOW_HEIGHT / 2.) - MENU_SIZE / 2.),
        Val::Px(0.),
    );

    commands
        .spawn(create_container(&menu_position))
        .with_children(|parent| {
            parent
                .spawn(create_button(NORMAL_BUTTON))
                .with_children(|parent| {
                    parent.spawn(create_text(
                        "Start",
                        asset_server.load("fonts/FiraSans-Bold.ttf"),
                    ));
                })
                .insert(MenuButton {
                    button_type: MenuButtonType::StartButton,
                });
        })
        .with_children(|parent| {
            parent
                .spawn(create_button(NORMAL_BUTTON))
                .with_children(|parent| {
                    parent.spawn(create_text(
                        "Create Party",
                        asset_server.load("fonts/FiraSans-Bold.ttf"),
                    ));
                })
                .insert(MenuButton {
                    button_type: MenuButtonType::CreateParty,
                });
        })
        .insert(MainMenu);
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match button.button_type {
                    MenuButtonType::StartButton => next_state.set(GameState::GamePlay),
                    _ => (),
                }
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = BackgroundColor(HOVERED_BUTTON);
            }
            _ => *color = BackgroundColor(NORMAL_BUTTON),
        }
    }
}

fn cleanup_menu(mut commands: Commands, main_menu: Query<Entity, With<MainMenu>>) {
    let main_menu = main_menu.single();
    commands.entity(main_menu).despawn_recursive();
}
