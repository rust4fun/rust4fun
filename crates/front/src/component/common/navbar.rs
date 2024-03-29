use crate::router::Route;
use crate::store::AuthStore;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

// styles
const HOME_TAB: &str = "block py-2 px-3 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500";
const TAB: &str  = "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent";

pub struct NavigationBar {
    auth_store: Rc<AuthStore>,
}

impl Component for NavigationBar {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let auth_store = Dispatch::<AuthStore>::global().get();

        Self { auth_store }
    }

    /// 参考 UI <https://flowbite.com/docs/components/navbar/#default-navbar>
    fn view(&self, _: &Context<Self>) -> Html {
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
                                <Link<Route> to={Route::Index} classes={classes!(HOME_TAB)}>{ "Home" }</Link<Route>>
                            </li>
                            if self.auth_store.is_authorization {
                                <li>
                                    <Link<Route> to={Route::Dashboard} classes={classes!(TAB)}>{ "Hello" }</Link<Route>>
                                </li>
                                <li>
                                    <Link<Route> to={Route::StudySphere} classes={classes!(TAB)}>{ "Study Sphere" }</Link<Route>>
                                </li>
                                <li>
                                    <Link<Route> to={Route::Logout} classes={classes!(TAB)}>{ "Logout" }</Link<Route>>
                                </li>
                            } else {
                                <li>
                                    <Link<Route> to={Route::Login} classes={classes!(TAB)}>{ "Login" }</Link<Route>>
                                </li>
                                <li>
                                    <Link<Route> to={Route::Signup} classes={classes!(TAB)}>{ "Signup" }</Link<Route>>
                                </li>
                            }
                            </ul>
                        </div>
                    </div>
                </nav>
            </>
        }
    }
}
