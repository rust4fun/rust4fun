use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

const STYLE: &str =
    "border-b block rounded-lg bg-gray-100 px-4 py-2 text-sm font-medium text-gray-700";

#[function_component(SphereTitleItem)]
pub fn sphere_title_item(props: &Props) -> Html {
    let Props { title } = props;

    html! {
        <>
            <li>
                <div class={STYLE}>
                    {title}
                </div>
            </li>
        </>
    }
}
