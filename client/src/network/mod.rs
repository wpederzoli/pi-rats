use actix_codec::Framed;
use awc::{ws, ws::Codec, BoxedSocket, Client};
use bevy::prelude::*;
use futures_util::{SinkExt as _, StreamExt as _};

pub struct NetworkPlugin;

pub struct CreatePartyEvent;

#[derive(Resource)]
pub struct Connection {
    ws: Framed<BoxedSocket, Codec>,
}

unsafe impl Sync for Connection {}
unsafe impl Send for Connection {}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreatePartyEvent>().add_system(create_party);
    }
}

#[actix_rt::main]
async fn create_party(mut create_event: EventReader<CreatePartyEvent>, mut commands: Commands) {
    for _ in create_event.iter() {
        let (_resp, mut connection) = Client::new()
            .ws("ws://127.0.0.1:8080/ws/")
            .connect()
            .await
            .unwrap();

        connection
            .send(ws::Message::Text("Echo".into()))
            .await
            .unwrap();
        let response = connection.next().await.unwrap().unwrap();

        let con = Connection { ws: connection };
        commands.insert_resource(con);

        println!("response: {:?}", response);
    }
}

async fn read_socket(mut ws: ResMut<Connection>) {
    ws.ws
        .send(ws::Message::Text("New message".into()))
        .await
        .unwrap();
}
