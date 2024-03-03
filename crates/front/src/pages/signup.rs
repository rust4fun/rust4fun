use crate::component::SignupSection;
use yew::prelude::*;

#[function_component(SignupPage)]
pub fn login_page() -> Html {
    html! {
      <>
          <SignupSection />
      </>
    }
}
