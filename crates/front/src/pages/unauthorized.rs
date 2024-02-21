use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(UnauthorizedPage)]
pub fn unauthorized_page() -> Html {
    html! {
        <div class="grid h-screen place-content-center bg-white px-4">
            <h1 class="uppercase tracking-widest text-gray-500">{"401 | Unauthorized"}</h1>
            <Link<Route> to={Route::Login} classes={classes!("underline")}>{ "Login" }</Link<Route>>
        </div>
    }
}
