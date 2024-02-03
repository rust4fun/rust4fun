mod form;
mod section;

use section::SignupSection;
use yew::prelude::*;

#[function_component(Signup)]
pub fn signup() -> Html {
    html! {
        <>
            <SignupSection />
        </>
    }
}
