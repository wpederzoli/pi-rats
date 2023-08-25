use bevy::prelude::Bundle;

pub trait MapComponent: Bundle {
    fn init(&mut self, x: f32, y: f32) -> Self;
}
