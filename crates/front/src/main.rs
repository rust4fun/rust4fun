mod api;
mod app;
mod auth;
pub mod component;
mod pages;
mod router;
mod store;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
