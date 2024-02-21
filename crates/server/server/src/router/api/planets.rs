pub mod create_message;
pub mod create_planet;
pub mod delete_message;
pub mod delete_planets;
pub mod edit_message;
pub mod edit_planets;
pub mod get_planet_detail;
pub mod list_messages;
pub mod list_planets;

use axum::{
    routing::{post, put},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_planet::handler).get(list_planets::handler))
        .route(
            "/:chat_room_id",
            put(edit_planets::handler)
                .get(get_planet_detail::handler)
                .delete(delete_planets::handler),
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
