use crate::component::Signup;
use yew::prelude::*;

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    html! {
      <>
        <section>
            <Signup />
        </section>
      </>
    }
}
