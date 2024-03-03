use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub planet_name: String,
}

const STYLE: &str = "block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700";

#[function_component(PlanetItem)]
pub fn planet_item(props: &Props) -> Html {
    let Props { planet_name } = props;

    html! {
        <>
            <li>
                <button type="button" class={STYLE}>
                    {planet_name}
                </button>
            </li>
        </>
    }
}
