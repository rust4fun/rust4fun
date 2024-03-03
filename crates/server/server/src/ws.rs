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

    pub fn manage_channel(&mut self, planet_id: PlanetId) -> Channel {
        let channel = self.0.entry(planet_id).or_default().clone();

        tracing::info!("Active channel {}", self.0.len());

        channel
    }

    pub fn send_or_pop(&mut self, planet_id: PlanetId, msg: String) {
        let channel = self.manage_channel(planet_id.clone());

        if let Err(e) = channel.send(msg) {
            tracing::warn!("No active reciever ({e:?}), and remove channel");
            self.0.remove(&planet_id);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Channel(broadcast::Sender<String>);

impl Channel {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(10);
        Self(tx)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.0.subscribe()
    }

    fn send(&self, msg: String) -> Result<usize, tokio::sync::broadcast::error::SendError<String>> {
        self.0.send(msg)
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self::new()
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

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if process_message(msg.clone()).is_break() {
                break;
            }
        }
    });

    // chat_room を取得
    let rooms = state.chat_rooms();
    let chat_room = {
        let mut rooms = rooms.lock().await;
        rooms.manage_channel(planet_id)
    };
    let mut chat_recv = chat_room.subscribe();

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
