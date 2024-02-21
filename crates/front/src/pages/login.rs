use crate::component::LoginSection;
use yew::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    html! {
      <>
          <LoginSection />
      </>
    }
}
