use bevy::prelude::{EventReader, Res};
use common::RoomAction;

use super::websocket::WsChannel;

// use std::str::from_utf8;
//
// use actix_codec::Framed;
// use awc::{
//     ws::{self, Codec},
//     BoxedSocket, Client,
// };
// use common::RoomAction;
// use futures_util::{SinkExt as _, StreamExt as _};
//
pub const URL: &str = "ws://127.0.0.1:8080/ws/";
//
// pub async fn connect_to_server(room_action: RoomAction) -> RoomAction {
//     let (_res, mut ws) = Client::new().ws(URL).connect().await.unwrap();
//
//     let json_msg = serde_json::to_string(&room_action).unwrap();
//     ws.send(ws::Message::Text(json_msg.into())).await.unwrap();
//
//     let mut response = RoomAction::Invalid("".to_string());
//
//     loop {
//         if let Ok(ws_msg) = ws.next().await.unwrap() {
//             match ws_msg {
//                 ws::Frame::Text(msg) => match from_utf8(&msg) {
//                     Ok(txt) => {
//                         response = serde_json::from_str(txt).unwrap();
//                     }
//                     _ => response = RoomAction::Invalid("Unable to create room".to_string()),
//                 },
//                 _ => response = RoomAction::Invalid("Something went wrong".to_string()),
//             }
//         }
//         break;
//     }
//
//     response
// }

pub struct CreateRoomEvent(pub String);
pub struct JoinRoomEvent(pub String);
pub struct RoomCreated;

pub fn create_room(mut create_ev: EventReader<CreateRoomEvent>, ws: Res<WsChannel>) {
    for event in create_ev.iter() {
        println!("create event reader:");
        let create_json = serde_json::to_string(&RoomAction::CreateRoom(event.0.clone()))
            .expect("Unable to create room");
        ws.sender.clone().send(create_json).unwrap();
    }
}

pub fn join_room(mut join_ev: EventReader<JoinRoomEvent>, ws: Res<WsChannel>) {
    for event in join_ev.iter() {
        let join_room_json = serde_json::to_string(&RoomAction::JoinRoom(event.0.clone()))
            .expect("Unable to join room");
        ws.sender.send(join_room_json).unwrap();
    }
}
