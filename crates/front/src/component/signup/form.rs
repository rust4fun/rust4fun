// use crate::component::common::InputField;
// use crate::router::Route;
use yew::prelude::*;
// use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_name: Callback<String>,
    pub input_email: Callback<String>,
    pub input_password: Callback<String>,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(SignupFrom)]
pub fn signup_from(_props: &Props) -> Html {
    let _button_class =
        "inline-block rounded-lg bg-blue-500 px-5 py-3 text-sm font-medium text-white";

    html! {
        // <form class="max-w-sm mx-auto my-5">
        //     <InputField input_type="text" label="Name" input={props.input_name.clone()}/>
        //     <InputField input_type="email" label="Email" input={props.input_email.clone()}/>
        //     <InputField input_type="password" label="Password" input={props.input_password.clone()}/>
        //     <div class="flex items-center justify-between">
        //         <p class="text-sm text-gray-500">{"No account? "}
        //             <Link<Route> to={Route::Signup} classes={classes!("underline")}>{ "Signup" }</Link<Route>>
        //         </p>
        //         <button type="submit" class={button_class} onclick={props.onclick.clone()}>{"Sign in"}</button>
        //     </div>
        // </form>
    }
}
