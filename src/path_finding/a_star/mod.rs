use self::{
    helpers::{get_distance, get_graph_nodes, get_lowest_f_score, get_neighbors, get_node_at},
    position::Position,
};
pub mod helpers;
pub mod position;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
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

#[cfg(test)]
mod tests {
    use super::*;

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
