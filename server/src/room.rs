use std::collections::HashSet;

use actix::Addr;

use crate::ServerWs;

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
}
