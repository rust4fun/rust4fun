use crate::api::ApiRequester;
use crate::component::{Footer, NavigationBar};
use crate::provider::{AuthStore, UserInfo};
use crate::router::{switch, Route};
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::{use_store, Dispatch};

#[function_component(App)]
pub fn app() -> Html {
    let (_, dispatch) = use_store::<AuthStore>();

    spawn_local(init_auth(dispatch.clone()));

    html! {
        <>
            <BrowserRouter>
                <NavigationBar/ >
                <Switch<Route> render={switch} />
            </BrowserRouter>
            <Footer />
        </>
    }
}

async fn init_auth(dispatch: Dispatch<AuthStore>) {
    if let Ok(token) = LocalStorage::get::<String>("access_token") {
        let client = ApiRequester::new(&token);
        match client.me().await {
            Ok(user) => {
                dispatch.reduce_mut(|s| {
                    s.is_authorization = true;
                    s.user = Some(UserInfo::new(user.name.unwrap()));
                });
            }
            Err(_) => {
                dispatch.reduce_mut(|s| {
                    s.is_authorization = false;
                    s.user = None;
                });
            }
        }
    }
}
