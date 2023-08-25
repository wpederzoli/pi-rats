use super::position::Position;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub position: Position,
    pub f_score: f32,
    pub g_score: f32,
    pub h_score: f32,
    pub parent: Option<Box<Node>>,
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
