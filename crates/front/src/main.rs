mod api;
mod app;
mod auth;
pub mod component;
mod pages;
pub mod provider;
mod router;
pub mod ws;

use app::App;
use ws::WebsocketService;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
