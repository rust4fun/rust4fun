use std::ops::Deref;

use crate::api::{ApiRequester, ClientRootExt};
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    // definition
    let name = use_state(String::default);

    // handler
    let cloned_name = name.clone();
    use_effect_with((), move |_| {
        spawn_local(async move {
            let token: String = LocalStorage::get("access_token").ok().unwrap();

            let requester = ApiRequester::new(&token);
            let res = requester.client().me().send().await.unwrap();

            cloned_name.set(res.name.clone().unwrap())
        });
    });

    html! {
      <>
      <center>
        <p>{format!("Hello {}", name.deref())}</p>
      </center>
      </>
    }
}
