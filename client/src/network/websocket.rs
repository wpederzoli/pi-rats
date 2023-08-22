use std::{str::from_utf8, thread};

use actix_codec::Framed;
use awc::{ws::Codec, BoxedSocket, Client};
use bevy::prelude::*;
use common::RoomAction;
use futures_util::{SinkExt, StreamExt};

use crate::{
    player::{player::PlayerBundle, PLAYER_TWO_INITIAL_POS},
    GameState,
};

use super::room::{create_room, join_room, CreateRoomEvent, JoinRoomEvent, URL};

pub struct WebSocketPlugin;

pub struct WebSocket {
    ws: Framed<BoxedSocket, Codec>,
}

#[derive(Resource)]
pub struct Channel {
    sender: crossbeam_channel::Sender<String>,
    receiver: crossbeam_channel::Receiver<String>,
}

#[derive(Resource)]
pub struct WsChannel {
    pub sender: crossbeam_channel::Sender<String>,
    receiver: crossbeam_channel::Receiver<String>,
}

impl WebSocket {
    pub async fn new() -> Self {
        let (_res, ws) = Client::new().ws(URL).connect().await.unwrap();

        WebSocket { ws }
    }
}

impl Plugin for WebSocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateRoomEvent>()
            .add_event::<JoinRoomEvent>()
            .add_startup_system(setup)
            .add_system(create_room)
            .add_system(join_room)
            .add_system(read_messages);
    }
}

fn read_messages(
    ch: Res<Channel>,
    ws: Res<WsChannel>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(msg) = ch.receiver.try_recv() {
        println!("message received {:?}", msg);
        if let Ok(action) = serde_json::from_str::<RoomAction>(&msg) {
            match action {
                RoomAction::RoomCreated => {
                    let wait_for_players =
                        serde_json::to_string(&RoomAction::WaitingPlayers).unwrap();
                    ws.sender.send(wait_for_players).unwrap();
                    next_state.set(GameState::Waiting);
                }
                RoomAction::JoinedRoom => next_state.set(GameState::Waiting),
                RoomAction::PlayerJoined => {
                    commands.spawn(PlayerBundle::new(PLAYER_TWO_INITIAL_POS));
                    next_state.set(GameState::GamePlay);
                }
                _ => info!("invalid action"),
            }
        }
    }
}

#[actix_rt::main]
pub async fn setup(mut commands: Commands) {
    let (sender, receiver) = crossbeam_channel::unbounded::<String>();
    let (ws_sender, ws_receiver) = crossbeam_channel::unbounded::<String>();

    commands.insert_resource(Channel {
        sender: sender.clone(),
        receiver: receiver.clone(),
    });

    commands.insert_resource(WsChannel {
        sender: ws_sender.clone(),
        receiver: ws_receiver.clone(),
    });

    thread::spawn(move || {
        let rx = ws_receiver.clone();
        let tx = sender.clone();

        let rt = actix_rt::Runtime::new().unwrap();
        let h = rt.spawn(async move {
            let mut ws = WebSocket::new().await.ws;
            loop {
                if let Ok(ch_msg) = rx.recv() {
                    println!("received message in thread: {:?}", ch_msg);
                    if let Ok(msg) = serde_json::from_str::<RoomAction>(&ch_msg) {
                        match msg {
                            RoomAction::WaitingPlayers => println!("Waiting for players"),
                            _ => ws
                                .send(awc::ws::Message::Text(ch_msg.into()))
                                .await
                                .unwrap(),
                        }
                    }
                }

                if let Ok(ws_msg) = ws.next().await.unwrap() {
                    match ws_msg {
                        awc::ws::Frame::Text(msg) => match from_utf8(&msg) {
                            Ok(txt) => {
                                println!("text: {:?}", txt);
                                tx.send(txt.to_string()).unwrap();
                            }
                            _ => println!("no txt"),
                        },
                        _ => println!("msg fail"),
                    }
                }
            }
        });

        rt.block_on(h).unwrap();
    });
}
