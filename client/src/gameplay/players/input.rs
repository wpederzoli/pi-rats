use bevy::{
    prelude::{Component, EventReader, Input, MouseButton, Res, Vec2},
    window::CursorMoved,
};

#[derive(Component)]
pub struct InputSystem {
    pub mouse_position: Vec2,
    pub mouse_btn: Option<MouseButton>,
}

impl InputSystem {
    pub fn new() -> Self {
        InputSystem {
            mouse_position: Vec2::new(0., 0.),
            mouse_btn: None,
        }
    }

    fn get_mouse_pressed(&self, mouse: Res<Input<MouseButton>>) -> Option<MouseButton> {
        if mouse.just_pressed(MouseButton::Left) {
            return Some(MouseButton::Left);
        }

        if mouse.just_pressed(MouseButton::Right) {
            return Some(MouseButton::Right);
        }

        if mouse.just_released(MouseButton::Left) || mouse.just_released(MouseButton::Right) {
            return None;
        }

        None
    }

    pub fn update(
        &mut self,
        mut cursor_ev: EventReader<CursorMoved>,
        mouse: Res<Input<MouseButton>>,
    ) {
        for ev in cursor_ev.iter() {
            self.mouse_position = Vec2::new(ev.position.x, ev.position.y);
        }

        self.mouse_btn = self.get_mouse_pressed(mouse);
    }
}
