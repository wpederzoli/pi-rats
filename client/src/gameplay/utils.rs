use bevy::prelude::{Query, Transform, Vec2};

use super::platforms::platform_cell::{PlatformSquare, PlatformSquareKind};

pub fn graph_from_cells(cells: &Query<(&PlatformSquare, &Transform)>) -> Vec<Vec2> {
    let mut graph = Vec::new();

    for (cell, transform) in cells.iter() {
        if cell.kind == PlatformSquareKind::Walkable {
            graph.push(Vec2::new(transform.translation.x, transform.translation.y));
        }
    }

    graph
}
