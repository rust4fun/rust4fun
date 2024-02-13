use yew::prelude::*;

#[function_component(IndexPage)]
pub fn index_page() -> Html {
    let main_title = "Rust Study For Fun, For EveryOne!";
    let sub_title = "Study sessions are held from time to time to learn Rust in a fun way! You can check and save the articles you are interested in and deepen your understanding with study fellows through chatting.";

    html! {
      <>
        <div class="mx-auto max-w-screen-xl px-4 pb-8 pt-16 sm:px-6 lg:px-8 lg:pt-24">
          <div class="text-center">
            <p class="text-3xl font-extrabold text-gray-900 sm:text-5xl">{main_title}</p>
            <p class="mx-auto mt-4 max-w-sm text-gray-500">{sub_title}</p>
            <a href="/" class="mt-8 inline-block rounded-full border border-indigo-600 px-12 py-3 text-sm font-medium text-indigo-600 hover:bg-indigo-600 hover:text-white focus:outline-none focus:ring active:bg-indigo-500">
              {"Let's Study"}
            </a>
          </div>
        </div>
      </>
    }
}
