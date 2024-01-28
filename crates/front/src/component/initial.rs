use crate::api::types;
use crate::api::ApiRequester;
use crate::api::ClientArticlesExt;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_bootstrap::component::card::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::Color;

#[function_component(Initial)]
pub fn initial() -> Html {
    let art = use_state(Vec::new);

    let art_clone = art.clone();
    let on_click = Callback::from(move |_| spawn_local(on_clicks(art_clone.clone())));

    let title = art.first().and_then(|x| x.title.clone());
    let description = art.first().and_then(|x| x.description.clone());
    html! {
            <section>
                <header class="header">
                    <h1>{ "todos" }</h1>
                </header>
                <section class={classes!("main")}>
                    <div>
                        <Button style={Color::Primary} onclick={on_click} modal_target={ "ExampleModal" }>
                            {{ "Launch demo modal" }}
                        </Button>
                        <Modal id="ExampleModal">
                            <ModalHeader title="Modal title" id="ExampleModal"/>
                            <ModalBody>
                                if title.is_some() {
                                    <Card class="width: 18rem;">
                                        <CardHeader>{"Card です"}</CardHeader>
                                        <CardBody>{{ title.unwrap() }}</CardBody>
                                        <CardBody>{{ description.unwrap() }}</CardBody>
                                        <CardFooter>{"最後"}</CardFooter>
                                    </Card>
                                }
                            </ModalBody>
                            <ModalFooter>
                                <Button style={ Color::Secondary } modal_dismiss={ true }>{ "Close" }</Button>
                                <Button style={ Color::Primary }>{ "Save changes" }</Button>
                            </ModalFooter>
                        </Modal>
                    </div>
                </section>
                <footer class={classes!("footer")}>
                </footer>
            </section>
    }
}

async fn on_clicks(state: UseStateHandle<Vec<types::Article>>) {
    // TODO: local storage を扱う glbal store を作る
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    // token を fetch する仕組みを作る
    storage.set("access_token", "").unwrap();
    let token = storage.get_item("access_token").unwrap().unwrap();

    let req = ApiRequester::new(&token);

    // let body = types::RequestBody::builder()
    //     .url("https://qiita.com/noshishi/items/2821c01d590bf9c96038".to_string());
    let body = req.client().list().send().await.unwrap();

    log::info!("{body:?}");

    state.set(body.into_inner());
}
