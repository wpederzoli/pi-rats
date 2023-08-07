use std::str::from_utf8;

use awc::{ws, Client};
use bevy::prelude::info;
use common::RoomAction;
use futures_util::{SinkExt as _, StreamExt as _};

pub async fn communicate_with_server(room_action: RoomAction) -> RoomAction {
    let (_res, mut ws) = Client::new()
        .ws("ws://127.0.0.1:8080/ws/")
        .connect()
        .await
        .unwrap();

    let json_msg = serde_json::to_string(&room_action).unwrap();
    ws.send(ws::Message::Text(json_msg.into())).await.unwrap();

    loop {
        if let Ok(ws_msg) = ws.next().await.unwrap() {
            match ws_msg {
                ws::Frame::Text(msg) => match from_utf8(&msg) {
                    Ok(txt) => serde_json::from_str(txt.clone()).unwrap(),
                    _ => info!("Unexpected message: {:?}", msg),
                },
                _ => info!("Failed to process response"),
            }
        }
    }
}
