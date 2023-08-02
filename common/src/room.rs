use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RoomAction {
    CreateRoom(String),
    RoomCreated,
    Invalid,
}
