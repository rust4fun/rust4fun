use rust_study_shared as shared;
use rust_study_client as client;

use client::types;
use client::Client;
use shared::Article;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(PartialEq, Properties, Default)]
pub struct Props {
    pub article: Option<Article>,
}

#[function_component(App)]
fn app() -> Html {
    let art = use_state(|| None::<types::Article>);
    let client = Client::new("http://localhost:8000");

    let art_clone = art.clone();
    spawn_local(async move {
        let body = client
            .comment()
            .await
            .unwrap();

        log::info!("{body:?}");

        art_clone.set(Some(body.into_inner()))
    });

    let title = art.as_ref().map(|x| x.title.clone());
    html! {
        <div class="todomvc-wrapper">
            <section class="todoapp">
                <header class="header">
                    <h1>{ "todos" }</h1>
                </header>
                <section class={classes!("main")}>

                <div>
                    { title }
                </div>
                </section>
                <footer class={classes!("footer")}>

                    // <button class="clear-completed" onclick={onclear_completed}>
                    //     { format!("Clear completed ({completed})") }
                    // </button>
                </footer>
            </section>

        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
