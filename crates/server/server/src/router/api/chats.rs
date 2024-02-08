pub mod create_chat_room;
pub mod create_message;
pub mod delete_chat_rooms;
pub mod delete_message;
pub mod edit_chat_room;
pub mod edit_message;
pub mod get_chat_room_detail;
pub mod list_chat_rooms;
pub mod list_messages;

use axum::{
    routing::{post, put},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            post(create_chat_room::handler).get(list_chat_rooms::handler),
        )
        .route(
            "/:chat_room_id",
            put(edit_chat_room::handler)
                .get(get_chat_room_detail::handler)
                .delete(delete_chat_rooms::handler),
        )
        .route(
            "/:chat_room_id/messages",
            post(create_message::handler).get(list_messages::handler),
        )
        .route(
            "/:chat_room_id/messages",
            put(edit_message::handler).delete(delete_message::handler),
        )
}
