use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_type: String,
    pub icon_id: IconId,
    pub label: String,
    pub input: Callback<String>,
}

#[function_component(InputField)]
pub fn input_field(props: &Props) -> Html {
    let onchnage = Callback::from({
        let cloned_input = props.input.clone();
        move |e: Event| {
            let target = e.target();
            let element = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let value = element.map(|e| e.value()).unwrap();
            cloned_input.emit(value);
        }
    });

    html! {
        <div class="d-flex flex-row align-items-center mb-4">
            <Icon icon_id={props.icon_id} class="me-3"/>
            <div class="form-outline flex-fill mb-0">
                <input type={props.input_type.clone()}
                       class="form-control"
                       onchange={onchnage} />
                <label class="form-label">{props.label.clone()}</label>
            </div>
        </div>
    }
}
