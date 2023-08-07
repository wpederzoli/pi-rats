use bevy::{
    prelude::{
        AssetServer, BuildChildren, Button, Changed, Color, Commands, Component,
        DespawnRecursiveExt, Entity, EventReader, EventWriter, Handle, Input, KeyCode, Query, Res,
        With,
    },
    text::{Font, Text, TextSection, TextStyle},
    ui::{BackgroundColor, Interaction, PositionType, Size, Style, UiRect, Val},
    utils::default,
    window::ReceivedCharacter,
};

use crate::network::{CreatePartyEvent, JoinRoomEvent};

use super::{
    ui::{create_button, create_container, create_text, MENU_SIZE, PRESSED_BUTTON_COLOR},
    MainMenu,
};

const INPUT_FIELD_HEIGHT: f32 = 100.;
const INPUT_FIELD_CONTAINER_HEIGHT: f32 = 300.;

pub enum ModalButtonType {
    Create,
    Join,
    Cancel,
}

#[derive(Component)]
pub struct InputField(String);

#[derive(Component)]
pub struct ModalMenu(bool);

#[derive(Component)]
pub struct ModalButton(ModalButtonType);

pub fn create_input_field(
    commands: &mut Commands,
    font: Handle<Font>,
    modal_type: ModalButtonType,
) -> Entity {
    let input_container_style = Style {
        position: UiRect::default(),
        position_type: PositionType::Absolute,
        flex_direction: bevy::ui::FlexDirection::Column,
        align_items: bevy::ui::AlignItems::Center,
        size: Size::new(Val::Px(MENU_SIZE), Val::Px(INPUT_FIELD_CONTAINER_HEIGHT)),
        ..default()
    };
    let container = create_container(&Color::DARK_GRAY, &input_container_style);
    let text = match modal_type {
        ModalButtonType::Create => create_text("New Party Name: ", font.clone()),
        _ => create_text("Join Party Name: ", font.clone()),
    };
    let input_field_style = Style {
        position: UiRect::default(),
        position_type: PositionType::Relative,
        align_items: bevy::ui::AlignItems::Center,
        justify_content: bevy::ui::JustifyContent::Center,
        size: Size::new(Val::Px(MENU_SIZE), Val::Px(INPUT_FIELD_HEIGHT)),
        ..default()
    };
    let input_container = create_container(&Color::ANTIQUE_WHITE, &input_field_style);

    let cancel_btn = create_button(Color::BLACK);
    let cancel_txt = create_text("Cancel", font.clone());
    let accept_btn = create_button(Color::BLACK);
    let accept_txt = create_text("Accept", font.clone());

    commands
        .spawn(container)
        .with_children(|parent| {
            parent.spawn(text);
        })
        .with_children(|parent| {
            parent.spawn(input_container).with_children(|parent| {
                let input_field = InputField(String::from(""));
                parent
                    .spawn(create_text(input_field.0.as_str(), font.clone()))
                    .insert(input_field);
            });
        })
        .with_children(|parent| {
            parent
                .spawn(accept_btn)
                .with_children(|parent| {
                    parent.spawn(accept_txt);
                })
                .insert(ModalButton(modal_type));
        })
        .with_children(|parent| {
            parent
                .spawn(cancel_btn)
                .with_children(|parent| {
                    parent.spawn(cancel_txt);
                })
                .insert(ModalButton(ModalButtonType::Cancel));
        })
        .insert(ModalMenu(true))
        .id()
}

pub fn handle_input(
    mut ev_char: EventReader<ReceivedCharacter>,
    keyboard: Res<Input<KeyCode>>,
    mut input_text: Query<(&mut Text, &mut InputField)>,
    asset_server: Res<AssetServer>,
) {
    for (mut text, mut input_field) in input_text.iter_mut() {
        for ev in ev_char.iter() {
            if keyboard.just_pressed(KeyCode::Back) {
                input_field.0.pop();
            }
            if !ev.char.is_control() {
                if input_field.0.len() < 20 {
                    input_field.0.push(ev.char);
                }
            }
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            text.sections = vec![TextSection::new(
                input_field.0.clone(),
                TextStyle {
                    font,
                    font_size: 30.,
                    color: Color::BLACK,
                },
            )];
        }
    }
}

pub fn modal_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ModalButton),
        (Changed<Interaction>, With<Button>),
    >,
    menu: Query<Entity, With<ModalMenu>>,
    mut main_menu: Query<&mut MainMenu>,
    mut input_text: Query<&mut InputField>,
    mut commands: Commands,
    mut create_party: EventWriter<CreatePartyEvent>,
    mut join_party: EventWriter<JoinRoomEvent>,
) {
    for menu in menu.iter() {
        let mut main_menu = main_menu.single_mut();

        for (interaction, mut color, button) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    let mut text = input_text.single_mut();
                    match button.0 {
                        ModalButtonType::Create => {
                            create_party.send(CreatePartyEvent(text.0.clone()))
                        }
                        ModalButtonType::Join => join_party.send(JoinRoomEvent(text.0.clone())),
                        ModalButtonType::Cancel => {
                            commands.entity(menu).despawn_recursive();
                            text.0 = String::from("");
                            main_menu.0 = true;
                        }
                    }
                    *color = PRESSED_BUTTON_COLOR.into();
                }
                Interaction::Hovered => {
                    *color = BackgroundColor(Color::rgba_u8(57, 51, 51, 255));
                }
                _ => *color = BackgroundColor(Color::BLACK),
            }
        }
    }
}
