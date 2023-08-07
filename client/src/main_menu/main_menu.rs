use bevy::prelude::*;

use super::{
    modal::{create_input_field, ModalButtonType},
    ui::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
};

#[derive(Component)]
pub struct MenuButton {
    pub button_type: MenuButtonType,
}
#[derive(Component)]
pub struct MainMenu(pub bool);

pub enum MenuButtonType {
    CreateParty,
    JoinParty,
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut menu: Query<(Entity, &mut MainMenu)>,
) {
    for (entity, mut menu) in menu.iter_mut() {
        if menu.0 {
            for (interaction, mut color, button) in &mut interaction_query {
                match *interaction {
                    Interaction::Clicked => {
                        match button.button_type {
                            MenuButtonType::CreateParty => {
                                let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                                let input_field = create_input_field(
                                    &mut commands,
                                    font,
                                    ModalButtonType::Create,
                                );
                                commands.entity(entity).push_children(&[input_field]);
                                menu.0 = false;
                            }
                            MenuButtonType::JoinParty => {
                                let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                                let input_field =
                                    create_input_field(&mut commands, font, ModalButtonType::Join);
                                commands.entity(entity).push_children(&[input_field]);
                                menu.0 = false;
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
}
