use bevy::{
    prelude::{default, Bundle, Color, Commands, Component, Entity, Query, Transform, Vec2, With},
    sprite::{Sprite, SpriteBundle},
};

use crate::player::input::Target;

use super::{cell::CELL_SIZE, PLATFORM_HEIGHT, PLATFORM_LAYER, PLATFORM_WIDTH};

#[derive(Component)]
pub struct GameCell;

#[derive(Bundle)]
pub struct GameCellBundle {
    #[bundle]
    spb: SpriteBundle,
}

impl GameCellBundle {
    pub fn new(position: Vec2) -> Self {
        GameCellBundle {
            spb: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba_u8(12, 123, 234, 255),
                    custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, position.y, PLATFORM_LAYER),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct Platform {
    cells: Vec<(GameCellBundle, Entity)>,
    pub walkable: bool,
    pub shootable: bool,
}

impl<'a> Platform {
    pub fn new(position: Vec2, walkable: bool, shootable: bool, commands: &mut Commands) -> Self {
        let cells = init_cells(commands, position);

        Platform {
            cells,
            walkable,
            shootable,
        }
    }

    pub fn select_by_position(
        &mut self,
        position: Vec2,
        cells: &mut Query<(&mut Sprite, &Transform), With<GameCell>>,
    ) -> Option<Target> {
        for (mut sprite, pos) in cells.iter_mut() {
            let pos = pos.translation;

            if pos.x - CELL_SIZE / 2. <= position.x
                && pos.x + CELL_SIZE / 2. >= position.x
                && pos.y - CELL_SIZE / 2. <= position.y
                && pos.y + CELL_SIZE / 2. >= position.y
            {
                sprite.color = Color::RED;
                return Some(Target {
                    position: Some(Vec2::new(pos.x, pos.y)),
                });
            }
        }

        None
    }
}

fn init_cells(commands: &mut Commands, start_position: Vec2) -> Vec<(GameCellBundle, Entity)> {
    let mut cells = Vec::new();
    for x in 1..=PLATFORM_WIDTH {
        for y in 1..=PLATFORM_HEIGHT {
            let cell = commands.spawn(GameCellBundle::new(Vec2::new(
                start_position.x + (CELL_SIZE * x as f32),
                start_position.y + (CELL_SIZE * y as f32),
            )));
            cells.push((
                GameCellBundle::new(Vec2::new(
                    start_position.x * x as f32 + CELL_SIZE,
                    start_position.y * y as f32 + CELL_SIZE,
                )),
                cell.id(),
            ));
        }
    }

    cells
}
