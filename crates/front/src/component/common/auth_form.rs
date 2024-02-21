use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub onclick: Callback<MouseEvent>,
    pub is_login: bool,
    pub label: String,
}

/// 参照元
/// <https://mdbootstrap.com/docs/standard/extended/registration/>
#[function_component(AuthFrom)]
pub fn auth_from(props: &Props) -> Html {
    let button_class =
        "inline-block rounded-lg bg-blue-500 px-5 py-3 text-sm font-medium text-white";

    html! {
        <form class="max-w-sm mx-auto my-5">
            { for props.children.iter() }
            <div class="flex items-center justify-between">
                if props.is_login {
                    <p class="text-sm text-gray-500">{"No account? "}
                        <Link<Route> to={Route::Signup} classes={classes!("underline")}>{ "Signup" }</Link<Route>>
                    </p>
                }
                <button type="button" class={button_class} onclick={props.onclick.clone()}>{props.label.clone()}</button>
            </div>
        </form>
    }
}
