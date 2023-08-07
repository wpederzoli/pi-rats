use bevy::prelude::Vec2;

use super::{position::Position, Node};

pub fn get_graph_nodes(graph: &Vec<Position>) -> Vec<Node> {
    let mut node_graph = Vec::new();
    for point in graph {
        node_graph.push(Node::from(*point));
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
    for node in graph {
        if node.position == *position {
            return Some(node);
        }
    }

    None
}

pub fn vec2_to_position(vec2: &Vec2, start: &Vec2, cell_size: f32) -> Position {
    let x = ((vec2.x - start.x) / cell_size) as i8;
    let y = -((vec2.y - start.y) / cell_size) as i8;

    Position::new(x, y)
}

pub fn position_to_vec2(position: &Position, start: &Vec2, cell_size: f32) -> Vec2 {
    let x = start.x + position.x as f32 * cell_size;
    let y = start.y - position.y as f32 * cell_size;

    Vec2::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_vec2_from_position() {
        let start_point = Vec2::new(-10., 10.);
        let expected_pos = Position::new(1, 0);
        let vec2 = Vec2::new(-5., 10.);
        let cell_size = 5.;

        let pos = position_to_vec2(&expected_pos, &start_point, cell_size);

        assert_eq!(pos, vec2);
    }

    #[test]
    fn get_position_from_vec2() {
        let start_point = Vec2::new(-10., 10.);
        let vec2 = Vec2::new(5., 0.);
        let expected_position = Position::new(3, 2);
        let cell_size = 5.;

        let pos = vec2_to_position(&vec2, &start_point, cell_size);

        assert_eq!(pos, expected_position);
    }

    #[test]
    fn get_node_from_position() {
        let point_x = Position::new(2, 0);
        let mut graph = vec![
            Node::from(Position::new(0, 0)),
            Node::from(Position::new(1, 0)),
            Node::from(Position::new(2, 0)),
        ];

        let node = get_node_at(&point_x, &mut graph).unwrap().clone();
        let goal_node = graph[2].clone();

        assert_eq!(node, goal_node);
    }

    #[test]
    fn calculate_distance() {
        let point_a = Position::new(0, 0);
        let point_b = Position::new(1, 1);

        let distance = get_distance(&point_a, &point_b);

        assert_eq!(distance, 1.4);
    }

    #[test]
    fn get_valid_neighbors() {
        let graph = vec![
            Node::from(Position::new(0, 0)),
            Node::from(Position::new(0, 1)),
            Node::from(Position::new(1, 0)),
            Node::from(Position::new(1, 1)),
            Node::from(Position::new(2, 0)),
        ];
        let node = graph[0].clone();
        let neighbors = get_neighbors(&node, &graph);

        assert_eq!(
            neighbors,
            vec![graph[1].clone(), graph[2].clone(), graph[3].clone()]
        );
    }

    #[test]
    fn get_lowest_f() {
        let graph = vec![
            Node {
                position: Position::new(0, 0),
                f_score: 2.,
                g_score: 0.,
                h_score: 0.,
                parent: None,
            },
            Node {
                position: Position::new(1, 0),
                f_score: 1.,
                g_score: 0.,
                h_score: 0.,
                parent: None,
            },
            Node::from(Position::new(2, 0)),
        ];

        let lowest_f = get_lowest_f_score(&graph);

        assert_eq!(lowest_f, graph[2]);
    }

    #[test]
    fn graph_to_nodes() {
        let graph = vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(0, 2),
        ];
        let nodes_grap = get_graph_nodes(&graph);
        let expected_graph = vec![
            Node::from(graph[0]),
            Node::from(graph[1]),
            Node::from(graph[2]),
        ];

        assert_eq!(nodes_grap, expected_graph);
    }
}
