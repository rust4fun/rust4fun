use crate::store::ChatStore;
use chrono::NaiveDateTime;
use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use gloo::net::websocket::{futures::WebSocket, Message, State};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yewdux::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatMessage {
    pub content: String,
    pub created_at: NaiveDateTime,
    pub meta: Option<ChatMeta>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChatMeta {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind")]
pub enum Messages {
    Connected,
    Chat(ChatMessage),
    Disconnected,
}

pub struct WebsocketService {
    pub tx: Sender<String>,
}

impl WebsocketService {
    pub fn new(dispatch: Dispatch<ChatStore>) -> Self {
        let ws = WebSocket::open("ws://127.0.0.1:8080/chat/db28639e-9045-4a77-99a7-62e08bc615e0")
            .unwrap();

        let (mut write, mut read) = ws.split();

        let (tx, mut rx) = futures::channel::mpsc::channel::<String>(10);

        // tx -> rx -> write
        spawn_local(async move {
            while let Some(s) = rx.next().await {
                write.send(Message::Text(s)).await.unwrap();
            }
        });

        // read -> tx2 -> rx2
        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(message)) => {
                        dispatch.reduce_mut(move |store| {
                            let chat = serde_json::from_str(&message).unwrap();
                            store.messages.push(chat);
                        });
                    }
                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(message) = decoded {
                            dispatch.reduce_mut(move |store| {
                                let chat = serde_json::from_str(message).unwrap();
                                store.messages.push(chat);
                            });
                        }
                    }
                    Err(e) => {
                        log::error!("ws: {:?}", e)
                    }
                }
            }
            log::debug!("WebSocket Closed");
        });

        Self { tx }
    }
}

use yew::prelude::*;

#[derive(Clone)]
pub struct UseWebSocketHandle {
    pub state: UseStateHandle<State>,
    pub recv_message: UseStateHandle<Option<ChatMessage>>,
    pub sender: Option<Sender<String>>,
}

impl UseWebSocketHandle {
    pub async fn send(&self, meg: Messages) {
        let s = serde_json::to_string(&meg).unwrap();
        let sender_clone = self.sender.clone();
        if let Some(mut tx) = sender_clone {
            let _ = tx.send(s).await;
        }
    }
}
