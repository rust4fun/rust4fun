use crate::component::common::InputField;
use yew::prelude::*;
use yew_bootstrap::component::Button;
use yew_icons::IconId;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_email: Callback<String>,
    pub input_password: Callback<String>,
    pub onclick: Callback<MouseEvent>,
}

/// 参照元
/// <https://mdbootstrap.com/docs/standard/extended/registration/>
#[function_component(LoginFrom)]
pub fn login_from(props: &Props) -> Html {
    html! {
        <form class="mx-1 mx-md-4">
            <InputField input_type="email" icon_id={IconId::BootstrapEnvelopeAtFill} label="Email" input={props.input_email.clone()}/>
            <InputField input_type="password" icon_id={IconId::FontAwesomeSolidKey} label="Email" input={props.input_password.clone()}/>
            <div class="d-flex justify-content-center mx-4 mb-3 mb-lg-4">
                <Button onclick={props.onclick.clone()} text={ "Login" }/>
            </div>
        </form>
    }
}
