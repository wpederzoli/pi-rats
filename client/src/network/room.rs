use std::str::from_utf8;

use awc::{ws, Client};
use common::RoomAction;
use futures_util::{SinkExt as _, StreamExt as _};

pub async fn connect_to_server(room_action: RoomAction) -> RoomAction {
    let (_res, mut ws) = Client::new()
        .ws("ws://127.0.0.1:8080/ws/")
        .connect()
        .await
        .unwrap();

    let json_msg = serde_json::to_string(&room_action).unwrap();
    ws.send(ws::Message::Text(json_msg.into())).await.unwrap();

    let mut response = RoomAction::Invalid("".to_string());

    loop {
        if let Ok(ws_msg) = ws.next().await.unwrap() {
            match ws_msg {
                ws::Frame::Text(msg) => match from_utf8(&msg) {
                    Ok(txt) => {
                        response = serde_json::from_str(txt).unwrap();
                    }
                    _ => response = RoomAction::Invalid("Unable to create room".to_string()),
                },
                _ => response = RoomAction::Invalid("Something went wrong".to_string()),
            }
        }
        break;
    }

    response
}
