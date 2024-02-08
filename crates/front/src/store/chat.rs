use crate::ws::ChatMessage;
use yewdux::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct ChatStore {
    pub messages: Vec<ChatMessage>,
}
