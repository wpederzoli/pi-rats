pub enum RoomAction {
    CreateRoom,
    RoomCreated(String),
    Invalid,
}

impl RoomAction {
    const CREATE_ROOM: &'static str = "CREATE_ROOM";
    const ROOM_CREATED: &'static str = "ROOM_CREATED";
    const INVALID: &'static str = "INVALID";

    pub fn get(txt: &str) -> Self {
        match txt {
            t if t == Self::CREATE_ROOM => RoomAction::CreateRoom,
            t if t == Self::ROOM_CREATED => RoomAction::RoomCreated("".to_string()),
            _ => RoomAction::Invalid,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::CreateRoom => Self::CREATE_ROOM.to_string(),
            Self::RoomCreated(_) => Self::ROOM_CREATED.to_string(),
            Self::Invalid => Self::INVALID.to_string(),
        }
    }
}
