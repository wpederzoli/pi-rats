use bevy::prelude::Vec2;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    x: i8,
    y: i8,
}

impl Position {
    pub fn new(x: i8, y: i8) -> Self {
        Position { x, y }
    }

    pub fn is_neighbor(&self, other: &Position) -> bool {
        *other == MoveDirections::Up.get_direction(&self)
            || *other == MoveDirections::UpLeft.get_direction(&self)
            || *other == MoveDirections::UpRight.get_direction(&self)
            || *other == MoveDirections::Down.get_direction(&self)
            || *other == MoveDirections::DownRight.get_direction(&self)
            || *other == MoveDirections::DownLeft.get_direction(&self)
            || *other == MoveDirections::Left.get_direction(&self)
            || *other == MoveDirections::Right.get_direction(&self)
    }
}

enum MoveDirections {
    Up,
    UpRight,
    UpLeft,
    Down,
    DownRight,
    DownLeft,
    Right,
    Left,
}

impl MoveDirections {
    pub fn get_direction(&self, position: &Position) -> Position {
        match self {
            Self::Up => Position::new(position.x, position.y - 1),
            Self::UpRight => Position::new(position.x + 1, position.y - 1),
            Self::UpLeft => Position::new(position.x - 1, position.y - 1),
            Self::Down => Position::new(position.x, position.y + 1),
            Self::DownRight => Position::new(position.x + 1, position.y - 1),
            Self::DownLeft => Position::new(position.x - 1, position.y - 1),
            Self::Right => Position::new(position.x + 1, position.y),
            Self::Left => Position::new(position.x - 1, position.y),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Node {
    position: Position,
    f_score: f32,
    g_score: f32,
    h_score: f32,
    parent: Option<Box<Node>>,
}

impl From<Position> for Node {
    fn from(position: Position) -> Self {
        Node {
            position,
            f_score: 0.,
            g_score: 0.,
            h_score: 0.,
            parent: None,
        }
    }
}

pub fn a_star(start: &Position, goal: &Position, graph: &Vec<Position>) -> Vec<Position> {
    let mut to_visit = vec![Node::from(*start)];
    let mut visited: Vec<Node> = Vec::new();

    let mut graph = get_graph_nodes(graph);

    while let Some(current_node) = to_visit.pop() {
        let current = if to_visit.len() > 0 {
            get_lowest_f_score(&to_visit)
        } else {
            current_node
        };

        visited.push(current.clone());
        to_visit.retain(|node| *node != current);

        if current.position == *goal {
            let mut current_tile = get_node_at(goal, &mut graph);
            let mut path = Vec::new();
            while let Some(node) = current_tile {
                path.push(node.position);
                current_tile = node.parent.as_deref_mut().map(|node| &mut *node);
            }

            path.reverse();
            return path;
        }

        for neighbor in get_neighbors(&current, &graph) {
            let neighbor = get_node_at(&neighbor.position, &mut graph).unwrap();
            neighbor.parent = Some(Box::new(current.clone()));

            neighbor.g_score =
                current.g_score + get_distance(&current.position, &neighbor.position);
            neighbor.h_score = get_distance(&goal, &neighbor.position);
            neighbor.f_score = neighbor.g_score + neighbor.h_score;

            if let Some(node) = to_visit
                .iter_mut()
                .find(|node| node.position == neighbor.position)
            {
                if node.f_score < neighbor.f_score {
                    continue;
                }
            }

            if let Some(node) = visited
                .iter_mut()
                .find(|node| node.position == neighbor.position)
            {
                if node.f_score < neighbor.f_score {
                    continue;
                }
            }

            to_visit.push(neighbor.clone());
        }

        visited.push(current);
    }

    Vec::new()
}

fn get_graph_nodes(graph: &Vec<Position>) -> Vec<Node> {
    let mut node_graph = Vec::new();
    for point in graph {
        node_graph.push(Node::from(*point));
    }

    node_graph
}

fn get_lowest_f_score(graph: &Vec<Node>) -> Node {
    let mut lowest = graph[0].clone();
    for point in graph {
        if point.f_score < lowest.f_score {
            lowest = point.clone();
        }
    }

    lowest
}

fn get_neighbors(node: &Node, graph: &Vec<Node>) -> Vec<Node> {
    let mut neighbors = Vec::new();
    for point in graph {
        if point.position.is_neighbor(&node.position) {
            neighbors.push(point.clone());
        }
    }

    neighbors
}

fn get_distance(a: &Position, b: &Position) -> f32 {
    let dx = (b.x as isize - a.x as isize) as f32;
    let dy = (b.y as isize - a.y as isize) as f32;
    let distance_squared = dx.powi(2) + dy.powi(2);

    f32::trunc(distance_squared.sqrt() * 10.) / 10.
}

fn get_node_at<'a>(position: &Position, graph: &'a mut Vec<Node>) -> Option<&'a mut Node> {
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

#[cfg(test)]
mod tests {

    use bevy::prelude::Vec2;

    use super::*;

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
    fn check_valid_neighbor() {
        let point_a = Position::new(0, 0);
        let point_b = Position::new(0, 1);

        assert_eq!(point_a.is_neighbor(&point_b), true);
    }

    fn check_invalid_neighbor() {
        let point_a = Position::new(0, 0);
        let point_b = Position::new(2, 3);

        assert_eq!(point_a.is_neighbor(&point_b), false);
    }

    #[test]
    fn get_valid_neighbors() {
        let graph = vec![
            Node::from(Position::new(0, 0)),
            Node::from(Position::new(1, 0)),
            Node::from(Position::new(2, 0)),
        ];
        let node = graph[0].clone();
        let neighbors = get_neighbors(&node, &graph);

        assert_eq!(neighbors, vec![graph[1].clone()]);
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

    #[test]
    fn find_shortest_path() {
        let start = Position::new(0, 0);
        let goal = Position::new(2, 0);
        let graph = vec![start, Position::new(1, 0), goal];

        let result = a_star(&start, &goal, &graph);
        let path = vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(2, 0),
        ];

        assert_eq!(path, result);
    }
}
