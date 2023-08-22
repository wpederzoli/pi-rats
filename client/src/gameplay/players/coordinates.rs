use bevy::prelude::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Coordinates {
    pub destination: Option<Vec2>,
    pub target: Option<Vec2>,
}

impl Default for Coordinates {
    fn default() -> Self {
        Coordinates {
            destination: None,
            target: None,
        }
    }
}

impl Coordinates {
    pub fn set_destination(&mut self, pos: Vec2) {
        self.destination = Some(pos);
    }

    pub fn set_target(&mut self, pos: Vec2) {
        self.target = Some(pos);
    }
}
