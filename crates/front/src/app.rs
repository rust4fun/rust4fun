use crate::component::{Footer, NavigationBar};
use crate::provider::AuthStore;
use crate::router::{switch, Route};
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;

#[function_component(App)]
pub fn app() -> Html {
    let (_, dispatch) = use_store::<AuthStore>();

    let cloned_dispatch = dispatch.clone();
    LocalStorage::get("access_token")
        .map(move |_: String| cloned_dispatch.reduce_mut(|s| s.is_authorization = true))
        .map_err(|e| log::warn!("{e:?}"))
        .ok();

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
