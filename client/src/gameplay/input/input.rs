use bevy::prelude::{Component, MouseButton, Vec2};

#[derive(Component)]
pub struct InputSystem {
    pub cursor_pos: Vec2,
    left_click: bool,
    right_click: bool,
}

impl InputSystem {
    pub fn new() -> Self {
        InputSystem {
            cursor_pos: Vec2::new(0., 0.),
            left_click: false,
            right_click: false,
        }
    }

    pub fn left_click(&self) -> bool {
        self.left_click
    }

    pub fn right_click(&self) -> bool {
        self.right_click
    }

    pub fn set_left_click(&mut self, state: bool) {
        self.left_click = state;
    }

    pub fn set_right_click(&mut self, state: bool) {
        self.right_click = state;
    }
}
