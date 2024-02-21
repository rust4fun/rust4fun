use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(IndexPage)]
pub fn index_page() -> Html {
    let main_title = "Rust Study For Fun, For EveryOne!";
    let main_title_class = "text-3xl font-extrabold text-gray-900 sm:text-5xl";

    let sub_title = "Study sessions are held from time to time to learn Rust in a fun way! You can check and save the articles you are interested in and deepen your understanding with study fellows through chatting.";
    let sub_title_class = "mx-auto mt-4 max-w-sm text-gray-500";

    let link_label = "Let's Study";
    let link_class = "mt-8 inline-block rounded-full border border-indigo-600 px-12 py-3 text-sm font-medium text-indigo-600 hover:bg-indigo-600 hover:text-white focus:outline-none focus:ring active:bg-indigo-500";

    html! {
      <>
        <div class="mx-auto max-w-screen-xl px-4 pb-8 pt-16 sm:px-6 lg:px-8 lg:pt-24">
          <div class="text-center">
            <p class={main_title_class}>{main_title}</p>
            <p class={sub_title_class}>{sub_title}</p>
            <Link<Route> to={Route::Dashboard} classes={classes!(link_class)}>{link_label}</Link<Route>>
          </div>
        </div>
      </>
    }
}
