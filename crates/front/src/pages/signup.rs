use crate::component::SignupSection;
use yew::prelude::*;

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    html! {
      <>
        <SignupSection />
      </>
    }
}
