use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub input_type: String,
    #[prop_or_default]
    pub label: String,
    pub input: Callback<String>,
    #[prop_or_default]
    pub is_valid: Option<UseStateHandle<bool>>,
    #[prop_or_default]
    pub invalid_message: Option<UseStateHandle<String>>,
}

#[function_component(InputField)]
pub fn input_field(props: &Props) -> Html {
    let onchange = Callback::from({
        let input = props.input.clone();
        move |e: Event| {
            let target = e.target();
            let element = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let value = element.map(|e| e.value()).unwrap();
            input.emit(value);
        }
    });

    let id = format!("user-{}", props.input_type);

    let label_class = "block overflow-hidden rounded-md border border-gray-200 px-3 py-2 shadow-sm focus-within:border-blue-600 focus-within:ring-1 focus-within:ring-blue-600";
    let input_class = "mt-1 w-full border-none p-0 focus:border-transparent focus:outline-none focus:ring-0 sm:text-sm";

    html! {
        <>
            <div>
                <label for={id.clone()} class={label_class}>
                    <div class="relative">
                        <span class="text-xs font-medium text-gray-700"> {props.label.clone()} </span>
                        <input
                            type={props.input_type.clone()}
                            id={id}
                            class={input_class}
                            onchange={onchange}
                        />
                        if !props.is_valid.as_ref().is_some_and(|x| *x.deref()) {
                            <span class="flex items-center font-medium tracking-wide text-red-500 text-xs mt-1 ml-1">
                                {props.invalid_message.as_deref().cloned().unwrap_or_default()}
                            </span>
                        }
                    </div>
                </label>
            </div>
        </>
    }
}
