use bevy::{
    prelude::{default, Bundle, Color, Component, Transform, Vec2},
    sprite::{Sprite, SpriteBundle},
};

#[derive(Bundle)]
pub struct PlatformCellBundle {
    #[bundle]
    pub sprite: SpriteBundle,
    pub cell: PlatformSquare,
}

#[derive(Component)]
pub struct PlatformSquare {
    kind: PlatformSquareKind,
}

#[derive(Debug, Clone, Copy)]
pub enum PlatformSquareKind {
    Walkable,
    Shootable,
}

pub const CELL_SIZE: f32 = 100.;

impl PlatformSquare {
    pub fn new(kind: PlatformSquareKind) -> Self {
        PlatformSquare { kind }
    }
}

impl PlatformCellBundle {
    pub fn new(x: f32, y: f32, kind: PlatformSquareKind) -> Self {
        PlatformCellBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 2.),
                ..default()
            },
            cell: PlatformSquare::new(kind),
        }
    }
}
