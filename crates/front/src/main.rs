pub mod component;
mod pages;
mod router;
pub mod server;
pub mod store;
pub mod structs;
pub mod utils;

use component::FullScreenLoading;
use gloo::timers::future::sleep;
use router::{switch, Route};
use std::time::Duration;
use store::{init_auth, AuthStore, LoadingStore, LocalStore};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;
use yewdux::Dispatch;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    Dispatch::<LocalStore>::global().set(LocalStore { count: 1 });
    Dispatch::<AuthStore>::global().set(AuthStore {
        is_authorization: false,
        user: None,
    });
    Dispatch::<LoadingStore>::global().set(LoadingStore { is_loading: true });

    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    let (store, load_dispatch) = use_store::<LoadingStore>();
    let (_, auth_dispatch) = use_store::<AuthStore>();

    use_effect_with((), {
        move |()| {
            let auth_dispatch = auth_dispatch.clone();
            let load_dispatch = load_dispatch.clone();
            spawn_local(async move {
                init_auth(auth_dispatch).await;

                sleep(Duration::from_secs(1)).await;
                load_dispatch.reduce_mut(|s| {
                    s.is_loading = false;
                });
            })
        }
    });

    html! {
        <>
            if store.is_loading {
                <FullScreenLoading />
            } else {
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            }
        </>
    }
}
