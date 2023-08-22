use bevy::prelude::MouseButton;

use super::{input::InputSystem, player::Player};

pub fn handle_mouse_clicks(player: &mut Player, input: &mut InputSystem) {
    if let Some(btn) = input.mouse_btn {
        match btn {
            MouseButton::Left => {
                if let None = player.coordinates.destination {
                    player.coordinates.set_destination(input.mouse_position);
                }
            }
            MouseButton::Right => {
                if let None = player.coordinates.target {
                    player.coordinates.set_target(input.mouse_position);
                }
            }
            _ => {}
        }
    }
}
