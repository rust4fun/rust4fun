use crate::router::Route;
use crate::store::AuthStore;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(LooutPage)]
pub fn logout_page() -> Html {
    let (_, dispatch) = use_store::<AuthStore>();
    dispatch.reduce_mut(|store| {
        store.is_authorization = false;
        store.user = None;
    });

    LocalStorage::clear();

    let navigator = use_navigator().unwrap();
    navigator.push(&Route::Index);

    html! {}
}
