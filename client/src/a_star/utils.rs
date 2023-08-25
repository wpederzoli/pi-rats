use bevy::prelude::Vec2;

use super::{node::Node, position::Position};

pub fn graph_to_nodes(graph: &Vec<Vec2>, cell_size: f32) -> Vec<Node> {
    let mut node_graph = Vec::new();
    for point in graph {
        let point_position = point_to_cell_position(point, cell_size);
        node_graph.push(Node::from(Position::new(
            (point_position.x * cell_size) as i32,
            (point_position.y * cell_size) as i32,
            cell_size as i32,
        )));
    }

    node_graph
}

pub fn get_lowest_f_score(graph: &Vec<Node>) -> Node {
    let mut lowest = graph[0].clone();
    for point in graph {
        if point.f_score < lowest.f_score {
            lowest = point.clone();
        }
    }

    lowest
}

pub fn get_neighbors(node: &Node, graph: &Vec<Node>) -> Vec<Node> {
    let mut neighbors = Vec::new();
    for point in graph {
        if point.position.is_neighbor(&node.position) {
            neighbors.push(point.clone());
        }
    }

    neighbors
}

pub fn get_distance(a: &Position, b: &Position) -> f32 {
    let dx = (b.x as isize - a.x as isize) as f32;
    let dy = (b.y as isize - a.y as isize) as f32;
    let distance_squared = dx.powi(2) + dy.powi(2);

    f32::trunc(distance_squared.sqrt() * 10.) / 10.
}

pub fn get_node_at<'a>(position: &Position, graph: &'a mut Vec<Node>) -> Option<&'a mut Node> {
    for node in graph.iter_mut() {
        if node.position == *position {
            return Some(node);
        }
    }

    None
}

pub fn point_to_cell_position(point: &Vec2, cell_size: f32) -> Vec2 {
    let x = (point.x / cell_size).round();
    let y = (point.y / cell_size).round();

    Vec2::new(x - 1., y - 1.)
}

pub fn position_to_vec2(position: &Position, start: &Vec2, cell_size: f32) -> Vec2 {
    let x = start.x + position.x as f32 * cell_size;
    let y = start.y - position.y as f32 * cell_size;

    Vec2::new(x, y)
}


