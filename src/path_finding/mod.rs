use std::f32::INFINITY;

use bevy::prelude::*;

use crate::{
    platforms::{
        cell::{GameCell, CELL_SIZE},
        MovementPlatform, PLATFORM_HEIGHT, PLATFORM_WIDTH,
    },
    player::Player,
};

pub struct PathFindingPlugin;

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FindPath>().add_system(find_path);
    }
}

pub struct FindPath {
    pub destiny: Vec2,
}

#[derive(Component)]
pub struct Path {
    steps: Vec<Vec2>,
}

impl Default for Path {
    fn default() -> Self {
        Path { steps: Vec::new() }
    }
}

impl Path {
    pub fn find_lowest_fscore(&self, nodes: Vec<PathNode>) -> Option<PathNode> {
        let mut lowest_score = INFINITY;
        let mut lowest_node: Option<PathNode> = None;

        for node in nodes {
            if node.f_score < lowest_score {
                lowest_score = node.f_score;
                lowest_node = Some(node);
            }
        }

        lowest_node
    }

    pub fn is_destination(&self, position: &Vec2, destination: &Vec2) -> bool {
        position.x == destination.x && position.y == destination.y
    }

    pub fn reconstruct_path(&mut self, node: PathNode) {
        let mut current_node = Some(Box::new(node));
        while let Some(ref box_node) = current_node {
            let node_ref = &*box_node;
            self.steps.insert(0, node_ref.position);
            current_node = node_ref.parent.clone();
        }
    }

    pub fn add_neighbors(&self, cell: &Vec3, pos: &Vec2, neighbors: &mut Vec<PathNode>) {
        if Neighbors::Right.is_neighbor(pos, &Vec2::new(cell.x, cell.y)) {
            neighbors.push(Neighbors::Right.get(pos))
        }
        if Neighbors::Left.is_neighbor(pos, &Vec2::new(cell.x, cell.y)) {
            neighbors.push(Neighbors::Left.get(pos))
        }
        if Neighbors::Top.is_neighbor(pos, &Vec2::new(cell.x, cell.y)) {
            neighbors.push(Neighbors::Top.get(pos))
        }
        if Neighbors::Bottom.is_neighbor(pos, &Vec2::new(cell.x, cell.y)) {
            neighbors.push(Neighbors::Bottom.get(pos))
        }
    }

    pub fn calculate_distance(&self, a: &Vec2, b: &Vec2) -> f32 {
        let x_diff = b.x - a.x;
        let y_diff = b.y - a.y;
        let distance = (x_diff.powi(2) + y_diff.powi(2)).sqrt();

        distance
    }

    pub fn calculate_heurisic_score(&self, node: &PathNode, destiny: &Vec2) -> f32 {
        (node.position.x - destiny.x).abs() + (node.position.y - destiny.y).abs()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PathNode {
    position: Vec2,
    g_score: f32,
    h_score: f32,
    f_score: f32,
    parent: Option<Box<PathNode>>,
}

impl PathNode {
    pub fn new(position: Vec2) -> Self {
        PathNode {
            position,
            g_score: 0.,
            h_score: 0.,
            f_score: 0.,
            parent: None,
        }
    }
}

enum Neighbors {
    Right,
    Left,
    Top,
    Bottom,
}

impl Neighbors {
    fn is_neighbor(&self, pos: &Vec2, cell_pos: &Vec2) -> bool {
        match self {
            Self::Right => pos.x + CELL_SIZE == cell_pos.x && cell_pos.y == pos.y,
            Self::Left => pos.x - CELL_SIZE == cell_pos.x && cell_pos.y == pos.y,
            Self::Top => pos.x == cell_pos.x && cell_pos.y == pos.y - CELL_SIZE,
            Self::Bottom => pos.x == cell_pos.x && cell_pos.y == pos.y + CELL_SIZE,
        }
    }
    fn get(&self, pos: &Vec2) -> PathNode {
        match self {
            Self::Right => PathNode::new(Vec2::new(pos.x + CELL_SIZE, pos.y)),
            Self::Left => PathNode::new(Vec2::new(pos.x - CELL_SIZE, pos.y)),
            Self::Top => PathNode::new(Vec2::new(pos.x, pos.y - CELL_SIZE)),
            Self::Bottom => PathNode::new(Vec2::new(pos.x, pos.y + CELL_SIZE)),
        }
    }
}

fn find_path(
    mut find_listener: EventReader<FindPath>,
    mut player: Query<(&Transform, &mut Path), With<Player>>,
    cells: Query<&Transform, (With<GameCell>, With<MovementPlatform>)>,
) {
    for event in find_listener.iter() {
        println!("find path");
        let mut frontiers: Vec<PathNode> = Vec::new();
        let mut visited: Vec<PathNode> = Vec::new();

        let (player_pos, mut path_finder) = player.single_mut();

        let start = PathNode::new(Vec2::new(
            player_pos.translation.x,
            player_pos.translation.y,
        ));

        frontiers.push(start);

        let max_tries = PLATFORM_WIDTH * PLATFORM_HEIGHT * PLATFORM_WIDTH * PLATFORM_HEIGHT;
        let mut tries = 0;

        while frontiers.len() > 0 {
            tries += 1;
            if let Some(current) = path_finder.find_lowest_fscore(frontiers.clone()) {
                let index = frontiers.iter().position(|el| *el == current).unwrap();
                frontiers.remove(index);

                visited.push(current.clone());

                if path_finder.is_destination(&current.position, &event.destiny) {
                    println!("is destination");
                    path_finder.reconstruct_path(current);
                    frontiers.clear();
                    return;
                }

                let mut neighbors: Vec<PathNode> = Vec::new();
                for cell in cells.iter() {
                    path_finder.add_neighbors(&cell.translation, &current.position, &mut neighbors);
                }

                for mut neighbor in neighbors {
                    println!("checking neighbors: {:?}", neighbor);
                    if visited.contains(&neighbor) {
                        continue;
                    }
                    let tentative_score = current.g_score
                        + path_finder.calculate_distance(&current.position, &neighbor.position);

                    if !frontiers.contains(&neighbor) {
                        frontiers.push(neighbor.clone());
                    }

                    neighbor.parent = Some(Box::new(current.clone()));
                    neighbor.g_score = tentative_score;
                    neighbor.h_score =
                        path_finder.calculate_heurisic_score(&neighbor, &event.destiny);
                    neighbor.f_score = neighbor.g_score + neighbor.h_score;
                }
            }

            if tries == max_tries {
                break;
            }
        }
    }
}
