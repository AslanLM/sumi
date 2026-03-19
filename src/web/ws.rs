use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;

use super::server::Log;

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<Log>,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    // spawn a task to send messages to the client
    let mut send_task = tokio::spawn(async move {
        while let Ok(log) = rx.recv().await {
            let json_log = serde_json::to_string(&log).unwrap();
            if sender
                .send(Message::Text(json_log))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    // spawn a task to receive messages from the client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Close(_) = msg {
                break;
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
