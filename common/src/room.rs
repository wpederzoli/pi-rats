use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RoomAction {
    CreateRoom(String),
    RoomCreated,
    WaitingPlayers,
    JoinRoom(String),
    JoinedRoom,
    Invalid(String),
    PlayerJoined,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GameMessage {
    PlayerJoined,
    TestMessage,
}
