use bevy::{
    prelude::{
        AssetServer, BuildChildren, Color, Commands, Component, Entity, EventReader, Handle, Input,
        KeyCode, Local, Query, Res, TextBundle, With,
    },
    text::{Font, Text, TextSection, TextStyle},
    ui::{Size, UiRect, Val},
    window::ReceivedCharacter,
};

use super::ui::{create_container, create_text, MENU_SIZE};

const INPUT_FIELD_HEIGHT: f32 = 100.;

#[derive(Component)]
pub struct InputField(String);

pub fn create_input_field(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let container = create_container(
        &UiRect::default(),
        &Color::DARK_GRAY,
        &Size::new(Val::Px(MENU_SIZE), Val::Px(INPUT_FIELD_HEIGHT)),
    );
    let text = create_text("New party name: ", font.clone());
    let input_f = create_container(
        &UiRect::default(),
        &Color::ANTIQUE_WHITE,
        &Size::new(Val::Px(MENU_SIZE), Val::Px(INPUT_FIELD_HEIGHT)),
    );
    commands
        .spawn(container)
        .with_children(|parent| {
            parent.spawn(text);
        })
        .with_children(|parent| {
            parent.spawn(input_f).with_children(|parent| {
                let input_field = InputField(String::from(""));
                parent
                    .spawn(create_text(input_field.0.as_str(), font.clone()))
                    .insert(input_field);
            });
        })
        .id()
}

pub fn handle_input(
    mut ev_char: EventReader<ReceivedCharacter>,
    keyboard: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut input_text: Query<&mut Text, With<InputField>>,
    asset_server: Res<AssetServer>,
) {
    for mut text in input_text.iter_mut() {
        for ev in ev_char.iter() {
            if keyboard.just_pressed(KeyCode::Back) {
                //Use InputField value insted of local string
                string.pop();
            }
            if !ev.char.is_control() {
                string.push(ev.char);
                let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                text.sections = vec![TextSection::new(
                    string.clone(),
                    TextStyle {
                        font,
                        font_size: 20.,
                        color: Color::BLACK,
                    },
                )];
            }
        }
    }
}
