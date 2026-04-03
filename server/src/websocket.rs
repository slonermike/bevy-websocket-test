use crate::protocol::{SpawnBallMessage, SpawnRequest};
use axum::{
    Router,
    extract::State,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
};
use bevy::prelude::*;
use rand::RngExt;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Resource)]
pub struct SpawnReceiver(pub Mutex<Receiver<SpawnRequest>>);

pub async fn run_server(tx: Sender<SpawnRequest>) {
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(Arc::new(tx));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Websocket server listening on ws://localhost:3000/ws");
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, State(tx): State<Arc<Sender<SpawnRequest>>>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(mut socket: WebSocket, tx: Arc<Sender<SpawnRequest>>) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            match serde_json::from_str::<SpawnRequest>(&text) {
                Ok(request) => {
                    let _ = tx.send(request);
                }
                Err(e) => {
                    eprintln!("Failed to parse message: {}", e);
                }
            }
        }
    }
}

pub fn handle_websocket_spawns(
    receiver: Res<SpawnReceiver>,
    mut writer: MessageWriter<SpawnBallMessage>,
) {
    let rx = receiver.0.lock().unwrap();
    while let Ok(request) = rx.try_recv() {
        let mut rng = rand::rng();
        writer.write(SpawnBallMessage {
            position: Vec2::new(request.x, request.y),
            velocity: Vec2::new(
                rng.random_range(-200.0..200.0),
                rng.random_range(-200.0..200.0),
            ),
        });
    }
}
