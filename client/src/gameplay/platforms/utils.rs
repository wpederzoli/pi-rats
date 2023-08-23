use bevy::prelude::Vec2;

use super::platform_cell::CELL_SIZE;

pub fn cell_hovered(cell: Vec2, pos: Vec2) -> bool {
    cell.x - CELL_SIZE / 2. < pos.x
        && cell.x + CELL_SIZE / 2. > pos.x
        && cell.y - CELL_SIZE / 2. < pos.y
        && cell.y + CELL_SIZE / 2. > pos.y
}
