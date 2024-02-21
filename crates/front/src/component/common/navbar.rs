use crate::provider::AuthStore;
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;

/// 参考 UI <https://flowbite.com/docs/components/navbar/#default-navbar>
#[function_component(NavigationBar)]
pub fn navigation_bar() -> Html {
    let (store, _) = use_store::<AuthStore>();

    let home_tab_class = "block py-2 px-3 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500";
    let tab_class = "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent";

    html! {
        <>
            <nav class="bg-white border-gray-200 dark:bg-gray-900">
                <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                    <a href="rust4fun.shuttleapp.rs/" class="flex items-center space-x-3 rtl:space-x-reverse">
                        <img src="assets/logo-16.svg" class="h-8 rounded-full" alt="rust4fun Logo" />
                        <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">{"Rust4Fun"}</span>
                    </a>
                    <div class="hidden w-full md:block md:w-auto" id="navbar-default">
                        <ul class="font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
                        <li>
                            <Link<Route> to={Route::Index} classes={classes!(home_tab_class)}>{ "Home" }</Link<Route>>
                        </li>
                        if store.is_authorization {
                            <li>
                                <Link<Route> to={Route::Dashboard} classes={classes!(tab_class)}>{ "Articles" }</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::StudySphere} classes={classes!(tab_class)}>{ "Study Sphere" }</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::Logout} classes={classes!(tab_class)}>{ "Logout" }</Link<Route>>
                            </li>
                        } else {
                            <li>
                                <Link<Route> to={Route::Login} classes={classes!(tab_class)}>{ "Login" }</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::Signup} classes={classes!(tab_class)}>{ "Signup" }</Link<Route>>
                            </li>
                        }
                        </ul>
                    </div>
                </div>
            </nav>
        </>
    }
}
