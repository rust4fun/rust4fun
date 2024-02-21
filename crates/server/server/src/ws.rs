use rust_study_shared as shared;

use crate::State;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::Path,
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use chrono::NaiveDateTime;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use shared::PlanetId;
use std::collections::HashMap;
use std::ops::ControlFlow;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Default)]
pub struct ChatRooms(HashMap<PlanetId, Channel>);

impl ChatRooms {
    pub fn inner(&self) -> &HashMap<PlanetId, Channel> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut HashMap<PlanetId, Channel> {
        &mut self.0
    }
}

#[derive(Clone)]
pub struct Channel(broadcast::Sender<String>);

impl Channel {
    pub fn new(broadcaster: broadcast::Sender<String>) -> Self {
        Self(broadcaster)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.0.subscribe()
    }

    pub fn send(&self, msg: String) {
        self.0
            .send(msg)
            .map_err(|e| tracing::error!("{e:?}"))
            .unwrap();
    }
}

pub fn router(state: Arc<State>) -> Router {
    Router::new()
        .route("/:chat_group_id", get(ws_handler))
        .layer(Extension(state))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    content: String,
    created_at: NaiveDateTime,
    meta: Option<ChatMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChatMeta {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Messages {
    Connected,
    Chat(ChatMessage),
    Disconnected,
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    Extension(state): Extension<Arc<State>>,
    Path(planet_id): Path<PlanetId>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    tracing::info!("`{user_agent}` connected.");

    ws.on_upgrade(move |socket| handle_socket(socket, state, planet_id))
}

async fn handle_socket(socket: WebSocket, state: Arc<State>, planet_id: PlanetId) {
    let (mut sender, mut receiver) = socket.split();

    // connected message を送る
    tracing::info!("connected!");
    // let connect_msg = serde_json::to_string(&Messages::Connected).unwrap();
    // sender.send(Message::Text(connect_msg)).await.unwrap();

    // chat_room を取得
    let rooms = state.chat_rooms();
    let chat_room = {
        let mut rooms = rooms.lock().await;
        tracing::debug!("chat room length {}", rooms.0.len());
        match rooms.0.get(&planet_id) {
            Some(v) => v.clone(),
            None => {
                let (tx, _) = broadcast::channel(10);
                let chat_room = Channel::new(tx);
                rooms.0.insert(planet_id, chat_room.clone());
                chat_room
            }
        }
    };
    let mut chat_recv = chat_room.subscribe();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if process_message(msg.clone()).is_break() {
                break;
            }

            if let Message::Text(t) = msg {
                chat_room.send(t);
            };
        }
    });

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = chat_recv.recv().await {
            sender
                .send(Message::Text(msg))
                .await
                .map_err(|e| tracing::error!("{e:?}"))
                .unwrap();
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
    tracing::info!("disconnected!");
}

fn process_message(msg: Message) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            tracing::info!(">>> sent str: {t:?}");
        }
        Message::Binary(d) => {
            tracing::info!(">>> sent {} bytes: {:?}", d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                tracing::info!(
                    ">>> sent close with code {} and reason `{}`",
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::info!(">>> somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            tracing::info!(">>> sent pong with {v:?}");
        }
        Message::Ping(v) => {
            tracing::info!(">>> sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
