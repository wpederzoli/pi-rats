use bevy::{
    prelude::{BuildChildren, Color, Commands, Entity, Handle, NodeBundle},
    text::Font,
    ui::{BackgroundColor, Size, Style, UiRect, Val},
    utils::default,
};

use super::ui::{create_container, create_text};

pub fn create_input_field(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let container = create_container(&UiRect::default(), &Color::DARK_GRAY);
    let text = create_text("New party name: ", font);
    let input_f = create_container(&UiRect::default(), &Color::ANTIQUE_WHITE);
    commands
        .spawn(container)
        .with_children(|parent| {
            parent.spawn(text);
        })
        .with_children(|parent| {
            parent.spawn(input_f);
        })
        .id()
}

pub fn spawn_modal(position: &UiRect) -> NodeBundle {
    NodeBundle {
        style: Style {
            position: *position,
            size: Size {
                width: Val::Px(500.),
                height: Val::Px(100.),
            },
            ..default()
        },
        background_color: BackgroundColor(Color::LIME_GREEN),
        ..default()
    }
}
