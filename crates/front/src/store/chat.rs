use crate::server::ChatMessage;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yewdux::{Dispatch, Store};

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct ChatStore {
    pub messages: Vec<ChatMessage>,
}

pub fn fetch_message(_planet_id: Uuid) {
    let _dispatch = Dispatch::<ChatStore>::global();

    spawn_local(async move {})
}
