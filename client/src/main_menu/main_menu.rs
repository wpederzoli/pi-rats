use bevy::prelude::*;

#[derive(Component)]
pub struct MenuButton {
    pub button_type: MenuButtonType,
}

pub enum MenuButtonType {
    StartButton,
    CreateParty,
    JoinParty,
}
