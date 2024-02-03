use yew::prelude::*;
use yew_bootstrap::component::{BrandType, NavBar, NavItem};

#[function_component(Index)]
pub fn index() -> Html {
    let brand = BrandType::BrandSimple {
        text: AttrValue::from("Rust4Fun"),
        url: Some(AttrValue::from("https://rust4fun.shuttleapp.rs/")),
    };

    html! {
      <>
      <NavBar nav_id={"navbar"} class="navbar-expand-lg navbar-light bg-white" brand={brand}>
          <NavItem text="Signup" url={AttrValue::from("/signup")} />
          <NavItem text="Login" url={AttrValue::from("/login")} />
      </NavBar>
      <center>
        <img src="assets/landing-page.png"/>
      </center>
      </>
    }
}
