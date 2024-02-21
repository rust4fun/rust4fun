use crate::provider::AuthStore;
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(AuthGuard)]
pub fn auth_guard(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let (store, _) = use_store::<AuthStore>();

    if !store.is_authorization {
        navigator.push(&Route::Unauthorized)
    }

    html! {
        <>
            { props.children.clone() }
        </>
    }
}
