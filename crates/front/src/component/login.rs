mod form;
mod section;

use section::LoginSection;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <>
            <LoginSection />
        </>
    }
}
