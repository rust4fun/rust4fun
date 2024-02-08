mod api;
mod app;
mod auth;
pub mod component;
mod pages;
mod router;
pub mod store;
pub mod ws;

use app::App;
use ws::WebsocketService;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
