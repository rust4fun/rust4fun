use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
                <Switch<Route> render={switch} />
                // if is_page_loading {
                //     <div class="pt-4 pl-2 top-[5.5rem] fixed">
                //         <Spinner width={Some("1.5rem")} height={Some("1.5rem")} color="text-ct-yellow-600" />
                //     </div>
                // }
        </BrowserRouter>
    }
}