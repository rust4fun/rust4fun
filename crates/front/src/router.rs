use crate::pages::{Index, LoginPage, SignupPage};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/signup")]
    Signup,
    #[at("/login")]
    LoginPage,
    #[at("/dashboard")]
    Dashboard,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! {<Index /> },
        Route::Signup => html! {<SignupPage /> },
        Route::LoginPage => html! {<LoginPage /> },
        Route::Dashboard => html! {<Index />},
    }
}
