use yew::prelude::*;

#[derive(PartialEq, Default)]
pub enum ToastLevel {
    #[default]
    Flat,
    Info,
    Warn,
    Error,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_show: UseStateHandle<bool>,
    #[prop_or_default]
    pub message: String,
    #[prop_or_default]
    pub level: ToastLevel,
}

#[function_component(Toast)]
pub fn toast(props: &Props) -> Html {
    let level_color = match props.level {
        ToastLevel::Flat => "bg-teal-500",
        ToastLevel::Info => "bg-blue-500",
        ToastLevel::Warn => "bg-yellow-500",
        ToastLevel::Error => "bg-red-500",
    };
    let main_class = format!("max-w-xs {level_color} text-sm text-white rounded-xl shadow-lg");

    let onclick = Callback::from({
        let is_show = props.is_show.clone();
        move |_: MouseEvent| {
            is_show.set(false);
        }
    });

    html! {
        if *props.is_show {
            <div class="absolute bottom-10 end-10">
                <div class={main_class} role="alert">
                    <div class="flex p-4">
                        {props.message.clone()}
                        <div class="ms-auto">
                            <button
                                type="button"
                                class="inline-flex flex-shrink-0 justify-center items-center size-5 rounded-lg text-white hover:text-white opacity-50 hover:opacity-100 focus:outline-none focus:opacity-100"
                                onclick={onclick}
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
