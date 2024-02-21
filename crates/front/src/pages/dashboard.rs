use crate::provider::AuthStore;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    let (store, _) = use_store::<AuthStore>();

    html! {
      <>
        <div class="grid place-items-center h-screen p-4 bg-gradient-to-bl from-gray-100 to-gray-300">
        {store.user.as_ref().map(|x| x.name.clone()).unwrap_or_default()}
        </div>
      </>
    }
}
