use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub hide_toast: Callback<()>,
    #[prop_or_default]
    pub message: String,
    #[prop_or_default]
    pub level: ToastLevel,
}

pub enum Msg {
    Close,
}

pub struct Toast;

impl Component for Toast {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Close => {
                ctx.props().hide_toast.emit(());

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let main_class = format!(
            "max-w-xs {} text-sm text-white rounded-xl shadow-lg",
            ctx.props().level.as_style()
        );

        html! {
            <div class="absolute bottom-10 end-10">
                <div class={main_class} role="alert">
                    <div class="flex p-4">
                        {ctx.props().message.clone()}
                        <div class="ms-auto">
                            <button
                                type="button"
                                class="inline-flex flex-shrink-0 justify-center items-center size-5 rounded-lg text-white hover:text-white opacity-50 hover:opacity-100 focus:outline-none focus:opacity-100"
                                onclick={ctx.link().callback(|_| Self::Message::Close)}
                            >
                            <svg class="flex-shrink-0 size-4" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

#[derive(PartialEq, Default)]
pub enum ToastLevel {
    #[default]
    Flat,
    Info,
    Warn,
    Error,
}

impl ToastLevel {
    fn as_style(&self) -> &str {
        match self {
            ToastLevel::Flat => "bg-teal-500",
            ToastLevel::Info => "bg-blue-500",
            ToastLevel::Warn => "bg-yellow-500",
            ToastLevel::Error => "bg-red-500",
        }
    }
}
