use yew::prelude::*;

pub struct FullScreenLoading;

impl Component for FullScreenLoading {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
                <div class="w-full h-full fixed top-0 left-0 bg-white opacity-75 z-50">
                    <div class="flex justify-center items-center mt-[50vh]">
                        <div class="rounded-full h-20 w-20 bg-blue-800 animate-ping"></div>
                    </div>
                </div>
            </>
        }
    }
}
