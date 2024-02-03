use crate::component::Login;
use yew::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    html! {
      <>
        <section>
            <Login />
        </section>
      </>
    }
}
