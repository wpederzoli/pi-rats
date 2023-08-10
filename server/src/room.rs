use common::GameMessage;
use std::collections::HashSet;

use actix::{Addr, Message};

use crate::ServerWs;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomMessage(pub GameMessage);

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

    pub fn send_message(self, message: String) {
        for client in self.clients {
            let room_msg = serde_json::from_str(message.as_str()).unwrap();
            client.do_send(RoomMessage(room_msg));
        }
    }
}
