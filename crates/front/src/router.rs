use crate::pages::{ChatPage, DashboardPage, IndexPage, LoginPage, SignupPage};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/signup")]
    Signup,
    #[at("/login")]
    Login,
    #[at("/dashboard")]
    Dashboard,
    #[at("/chat")]
    Chat,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! {<IndexPage /> },
        Route::Signup => html! {<SignupPage /> },
        Route::Login => html! {<LoginPage /> },
        Route::Dashboard => html! {<DashboardPage />},
        Route::Chat => html! {<ChatPage />},
    }
}
