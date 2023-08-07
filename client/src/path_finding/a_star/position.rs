#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: i8,
    pub y: i8,
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
            Self::DownRight => Position::new(position.x + 1, position.y + 1),
            Self::DownLeft => Position::new(position.x - 1, position.y + 1),
            Self::Right => Position::new(position.x + 1, position.y),
            Self::Left => Position::new(position.x - 1, position.y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_valid_neighbor() {
        let point_a = Position::new(0, 0);
        let point_b = Position::new(0, 1);

        assert_eq!(point_a.is_neighbor(&point_b), true);
    }

    #[test]
    fn check_invalid_neighbor() {
        let point_a = Position::new(0, 0);
        let point_b = Position::new(2, 3);

        assert_eq!(point_a.is_neighbor(&point_b), false);
    }

    #[test]
    fn get_direction_position() {
        let position = Position::new(0, 0);
        let up = Position::new(0, -1);
        let down = Position::new(0, 1);
        let left = Position::new(-1, 0);
        let right = Position::new(1, 0);
        let upleft = Position::new(-1, -1);
        let upright = Position::new(1, -1);
        let downleft = Position::new(-1, 1);
        let downright = Position::new(1, 1);

        let result_up = MoveDirections::Up.get_direction(&position);
        let result_down = MoveDirections::Down.get_direction(&position);
        let result_left = MoveDirections::Left.get_direction(&position);
        let result_right = MoveDirections::Right.get_direction(&position);
        let result_upleft = MoveDirections::UpLeft.get_direction(&position);
        let result_upright = MoveDirections::UpRight.get_direction(&position);
        let result_downleft = MoveDirections::DownLeft.get_direction(&position);
        let result_downright = MoveDirections::DownRight.get_direction(&position);

        assert_eq!(result_up, up);
        assert_eq!(result_down, down);
        assert_eq!(result_left, left);
        assert_eq!(result_right, right);
        assert_eq!(result_upright, upright);
        assert_eq!(result_downright, downright);
        assert_eq!(result_upleft, upleft);
        assert_eq!(result_downleft, downleft);
    }
}
