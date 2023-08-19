use common::{GameMessage, RoomAction};
use std::collections::HashSet;

use actix::{Addr, Message};

use crate::ServerWs;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomMessage(pub RoomAction);

#[derive(Clone, Debug)]
pub struct Room {
    id: String,
    clients: HashSet<Addr<ServerWs>>,
}

impl Room {
    pub fn new(id: String) -> Self {
        let clients = HashSet::new();
        Room { id, clients }
    }

    pub fn add_client(&mut self, client: Addr<ServerWs>) {
        self.clients.insert(client);
    }

    pub fn send_message(&self, message: String) {
        for client in &self.clients {
            let room_msg =
                serde_json::from_str::<RoomAction>(message.as_str()).expect("Invalid room action");
            client.do_send(RoomMessage(room_msg));
        }
    }
}
