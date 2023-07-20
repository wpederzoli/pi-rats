use bevy::prelude::*;

pub const MENU_SIZE: f32 = 400.;
const BACKGROUND_COLOR: Color = Color::BLACK;

#[derive(Component)]
pub struct MenuButton {
    pub button_type: MenuButtonType,
}

pub enum MenuButtonType {
    StartButton,
    CreateParty,
    JoinParty,
}

pub fn create_container(position: &UiRect) -> NodeBundle {
    NodeBundle {
        background_color: BackgroundColor(BACKGROUND_COLOR),
        style: Style {
            position: *position,
            size: Size::new(Val::Px(MENU_SIZE), Val::Px(MENU_SIZE)),
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
