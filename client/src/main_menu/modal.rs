use bevy::{
    prelude::{Color, NodeBundle},
    ui::{BackgroundColor, Size, Style, UiRect, Val},
    utils::default,
};

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
