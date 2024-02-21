use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div class="grid h-screen place-content-center bg-white px-4">
            <h1 class="uppercase tracking-widest text-gray-500">{"404 | Not Found"}</h1>
            <Link<Route> to={Route::Index} classes={classes!("underline")}>{ "Go To Home" }</Link<Route>>
        </div>
    }
}
