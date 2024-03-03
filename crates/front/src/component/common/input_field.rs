use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// styles
const LABEL: &str = "block overflow-hidden rounded-md border border-gray-200 px-3 py-2 shadow-sm focus-within:border-blue-600 focus-within:ring-1 focus-within:ring-blue-600";
const INPUT: &str = "mt-1 w-full border-none p-0 focus:border-transparent focus:outline-none focus:ring-0 sm:text-sm";

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub input_type: String,
    #[prop_or_default]
    pub label: String,
    pub input: Callback<String>,
    #[prop_or_default]
    pub is_valid: bool,
    #[prop_or_default]
    pub invalid_message: String,
}

pub enum Msg {
    ChangeInput(Event),
}

pub struct InputField;

impl Component for InputField {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeInput(e) => {
                let input = ctx.props().input.clone();

                let target = e.target();
                let element = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                let value = element.map(|e| e.value()).unwrap();
                input.emit(value);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = format!("user-{}", ctx.props().input_type);

        html! {
            <>
                <div>
                    <label for={id.clone()} class={LABEL}>
                        <div class="relative">
                            <span class="text-xs font-medium text-gray-700"> {ctx.props().label.clone()} </span>
                            <input
                                type={ctx.props().input_type.clone()}
                                id={id}
                                class={INPUT}
                                onchange={ctx.link().callback(|e: Event| Msg::ChangeInput(e))}
                            />
                            if !ctx.props().is_valid {
                                <span class="flex items-center font-medium tracking-wide text-red-500 text-xs mt-1 ml-1">
                                    {ctx.props().invalid_message.clone()}
                                </span>
                            }
                        </div>
                    </label>
                </div>
            </>
        }
    }
}
