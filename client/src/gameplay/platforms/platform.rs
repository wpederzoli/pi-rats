use bevy::{
    prelude::{default, Color, Commands, Component, Entity, Query, Transform, Vec2},
    sprite::{Sprite, SpriteBundle},
};

use super::{
    platform_cell::{PlatformCellBundle, PlatformSquare, PlatformSquareKind, CELL_SIZE},
    utils::cell_hovered,
};

#[derive(Component)]
pub struct Platform {
    moveable: bool,
    shootable: bool,
}

impl Platform {
    const PLATFORM_WIDTH: u8 = 3;
    const PLATFORM_HEIGHT: u8 = 5;

    pub fn new(
        start_position: Vec2,
        moveable: bool,
        shootable: bool,
        commands: &mut Commands,
    ) -> Self {
        let kind = if moveable {
            PlatformSquareKind::Walkable
        } else {
            PlatformSquareKind::Shootable
        };

        for x in 1..=Self::PLATFORM_WIDTH {
            for y in 1..=Self::PLATFORM_HEIGHT {
                let cell = PlatformCellBundle::new(
                    start_position.x + (CELL_SIZE * x as f32),
                    start_position.y + (CELL_SIZE * y as f32),
                    kind,
                );
                commands.spawn(cell);
            }
        }

        Platform {
            moveable,
            shootable,
        }
    }

    pub fn highlight_hovered(
        &self,
        mouse_pos: Vec2,
        cells: &mut Query<(&PlatformSquare, &Transform, &mut Sprite)>,
    ) {
        for (_, transform, mut sprite) in cells.iter_mut() {
            if cell_hovered(
                Vec2::new(transform.translation.x, transform.translation.y),
                mouse_pos,
            ) {
                sprite.color = Color::SEA_GREEN;
            } else {
                sprite.color = Color::OLIVE;
            }
        }
    }
}
