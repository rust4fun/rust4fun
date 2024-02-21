use crate::component::AuthGuard;
use crate::pages::{
    DashboardPage, IndexPage, LoginPage, NotFoundPage, SignupPage, SpherePage, UnauthorizedPage,
};

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
    #[at("/logout")]
    Logout,

    // required auth
    #[at("/dashboard")]
    Dashboard,
    #[at("/sphere")]
    StudySphere,

    #[at("/unauthorized")]
    Unauthorized,

    #[not_found]
    #[at("/notfound")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! { <IndexPage /> },
        Route::Signup => html! { <SignupPage /> },
        Route::Login => html! { <LoginPage /> },
        Route::Logout => html! { <LoginPage /> },

        Route::Unauthorized => html! { <UnauthorizedPage /> },
        Route::NotFound => html! { <NotFoundPage /> },

        // required auth
        Route::Dashboard => html! {
            <AuthGuard>
                <DashboardPage />
            </AuthGuard>
        },
        Route::StudySphere => html! {
            <AuthGuard>
                <SpherePage />
            </AuthGuard>
        },
    }
}
