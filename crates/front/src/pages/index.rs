use crate::component::{Footer, NavigationBar};
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

// styles
const MAIN_TITLE_STYLE: &str = "text-3xl font-extrabold text-gray-900 sm:text-5xl";
const SUB_TITLE_STYLE: &str = "mx-auto mt-4 max-w-sm text-gray-500";
const LINK_STYLE: &str = "mt-8 inline-block rounded-full border border-indigo-600 px-12 py-3 text-sm font-medium text-indigo-600 hover:bg-indigo-600 hover:text-white focus:outline-none focus:ring active:bg-indigo-500";

pub struct IndexPage;

impl Component for IndexPage {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let main_title = "Rust Study For Fun, For EveryOne!";
        let sub_title = "Study sessions are held from time to time to learn Rust in a fun way! You can check and save the articles you are interested in and deepen your understanding with study fellows through chatting.";
        let link_label = "Let's Study";

        html! {
          <>
            <NavigationBar/ >
            <div class="mx-auto max-w-screen-xl px-4 pb-8 pt-16 sm:px-6 lg:px-8 lg:pt-24">
              <div class="text-center">
                <p class={MAIN_TITLE_STYLE}>{main_title}</p>
                <p class={SUB_TITLE_STYLE}>{sub_title}</p>
                <Link<Route> to={Route::Dashboard} classes={classes!(LINK_STYLE)}>{link_label}</Link<Route>>
              </div>
            </div>
            <Footer />
          </>
        }
    }
}
