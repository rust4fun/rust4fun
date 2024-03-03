use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

// styles
const SUBMIT_BUTTON: &str =
    "inline-block rounded-lg bg-blue-500 px-5 py-3 text-sm font-medium text-white";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub onclick: Callback<MouseEvent>,
    pub is_login: bool,
    pub label: String,
}

pub enum Msg {
    Submit(MouseEvent),
}

pub struct AuthFrom;

impl Component for AuthFrom {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Submit(e) => {
                let onclick = ctx.props().onclick.clone();
                onclick.emit(e);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <form class="max-w-sm mx-auto my-5">
                { for ctx.props().children.iter() }
                <div class="flex items-center justify-between">
                    if ctx.props().is_login {
                        <p class="text-sm text-gray-500">{"No account? "}
                            <Link<Route> to={Route::Signup} classes={classes!("underline")}>{ "Signup" }</Link<Route>>
                        </p>
                    }
                    <button
                        type="button"
                        class={SUBMIT_BUTTON}
                        onclick={ctx.link().callback(Self::Message::Submit)}
                    >{ctx.props().label.clone()}
                    </button>
                </div>
            </form>
        }
    }
}
