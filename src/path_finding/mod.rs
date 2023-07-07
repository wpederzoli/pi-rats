use bevy::prelude::*;

use crate::{
    path_finding::a_star::{a_star, vec2_to_position},
    platforms::{cell::CELL_SIZE, MovementPlatform},
    WINDOW_HEIGHT, WINDOW_WIDTH,
};

mod a_star;

pub struct PathFindingPlugin;

pub struct FindPath {
    pub start: Vec2,
    pub goal: Vec2,
}

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FindPath>().add_system(find_path);
    }
}

fn find_path(mut event: EventReader<FindPath>, cells: Query<&Transform, With<MovementPlatform>>) {
    for e in event.iter() {
        let map_start = Vec2::new(
            (-WINDOW_WIDTH / 2.) + CELL_SIZE,
            WINDOW_HEIGHT / 2. - CELL_SIZE,
        );
        println!("find a path: start: {:?}, goal: {:?}", e.start, e.goal);
        let start = vec2_to_position(&e.start, &map_start, CELL_SIZE);
        let goal = vec2_to_position(&e.goal, &map_start, CELL_SIZE);

        let mut nodes_graph = Vec::new();
        for cell in cells.iter() {
            nodes_graph.push(vec2_to_position(
                &Vec2::new(cell.translation.x, cell.translation.y),
                &map_start,
                CELL_SIZE,
            ));
        }

        let path = a_star(&start, &goal, &nodes_graph);
        println!("path: {:?}", path);
    }
}
