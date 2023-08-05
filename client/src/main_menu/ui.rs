use bevy::{
    prelude::{BuildChildren, ButtonBundle, Color, Commands, Handle, NodeBundle, TextBundle},
    text::{Font, TextStyle},
    ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Size, Style, UiRect, Val},
    utils::default,
};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::{
    main_menu::{MenuButton, MenuButtonType},
    MainMenu,
};

const MENU_POSITION: UiRect = UiRect::new(
    Val::Px((WINDOW_WIDTH / 2.) - MENU_SIZE / 2.),
    Val::Px(0.),
    Val::Px((WINDOW_HEIGHT / 2.) - MENU_SIZE / 2.),
    Val::Px(0.),
);

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const MENU_SIZE: f32 = 400.;
pub const BACKGROUND_COLOR: Color = Color::BLACK;

pub fn create_main_menu(commands: &mut Commands, font: Handle<Font>) {
    let container = create_container(
        &MENU_POSITION,
        &BACKGROUND_COLOR,
        &Size::new(Val::Px(MENU_SIZE), Val::Px(MENU_SIZE)),
    );
    let button = create_button(NORMAL_BUTTON_COLOR);
    let create_txt = create_text("Create Party", font.clone());
    let join_text = create_text("Join Party", font);

    commands
        .spawn(container)
        .with_children(|parent| {
            parent
                .spawn(button.clone())
                .with_children(|p| {
                    p.spawn(create_txt);
                })
                .insert(MenuButton {
                    button_type: MenuButtonType::CreateParty,
                });
            parent
                .spawn(button)
                .with_children(|p| {
                    p.spawn(join_text);
                })
                .insert(MenuButton {
                    button_type: MenuButtonType::JoinParty,
                });
        })
        .insert(MainMenu);
}

pub fn create_container(position: &UiRect, color: &Color, size: &Size) -> NodeBundle {
    NodeBundle {
        background_color: BackgroundColor(*color),
        style: Style {
            position: *position,
            size: *size,
            align_items: AlignItems::Center,
            padding: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(35.), Val::Px(0.)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }
}

pub fn create_button(color: Color) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Px(65.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Px(15.)),
            ..default()
        },
        background_color: color.into(),
        ..default()
    }
}

pub fn create_text(text: &str, font: Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    )
}
