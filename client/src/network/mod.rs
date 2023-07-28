use awc::{ws, Client};
use bevy::prelude::*;
use futures_util::{SinkExt as _, StreamExt as _};
use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    thread,
};

pub struct NetworkPlugin;
pub struct CreatePartyEvent;

#[derive(Resource)]
pub struct Channel(Option<Mutex<Sender<ws::Message>>>);

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreatePartyEvent>()
            .insert_resource(Channel(None))
            .add_system(create_party)
            .add_system(test_ws);
    }
}

fn create_party(mut create_event: EventReader<CreatePartyEvent>, mut ch: ResMut<Channel>) {
    for _ in create_event.iter() {
        let (tx, rx) = channel::<ws::Message>();
        ch.0 = Some(Mutex::new(tx));
        thread::spawn(move || setup_network(rx));
    }
}

fn test_ws(ch: ResMut<Channel>) {
    if let Some(ch) = &ch.0 {
        if let Ok(send) = ch.lock() {
            let _ = send.send(ws::Message::Text("test".into()));
        }
    }
}

#[actix_rt::main]
async fn setup_network(rx: Receiver<ws::Message>) {
    let a = actix_rt::spawn(async move {
        let (_res, mut ws) = Client::new()
            .ws("ws://127.0.0.1:8080/ws/")
            .connect()
            .await
            .unwrap();

        loop {
            if let Ok(r) = rx.recv() {
                println!("received: {:?}", r);
                let _ = ws.send(r).await.unwrap();
            }

            if let Ok(msg) = ws.next().await.unwrap() {
                println!("message ws: {:?}", msg);
            }
        }
    });

    let _ = a.await;
}
