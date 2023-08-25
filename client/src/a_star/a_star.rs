pub enum MoveDirection {
    Top,
    LeftTop,
    RightTop,
    Down,
    LeftDown,
    RightDown,
    Right,
    Left,
}

//Return array of move diretions or empty
//Starting from position
//Eg. I'm at point (200., 40.) and I want to get to (300., 40.)
//I would get [MoveDirection::Right]
