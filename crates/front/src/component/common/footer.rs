use yew::prelude::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
                <footer class="absolute bottom-0 w-full py-10 px-4 sm:px-6 lg:px-8 mx-auto">
                    <div class="text-center">
                        <div>
                            <a class="flex-none text-xl font-semibold text-black dark:text-white dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600" href="#" aria-label="Brand">
                                {"Rust4Fun"}
                            </a>
                        </div>

                        <div class="mt-3">
                            <p class="text-gray-500">{"© 2024 rust4fun."}</p>
                        </div>

                        <div class="mt-3 space-x-2">
                            <a class="inline-flex justify-center items-center w-10 h-10 text-center text-gray-500 hover:bg-gray-100 rounded-full focus:outline-none focus:ring-2 focus:ring-blue-600 focus:ring-offset-2 focus:ring-offset-white transition dark:text-gray-500 dark:hover:text-gray-200 dark:hover:bg-gray-800" href="https://github.com/rust4fun/rust4fun">
                                <i class="lab la-github"></i>
                            </a>
                        </div>
                    </div>
                </footer>
            </>
        }
    }
}
