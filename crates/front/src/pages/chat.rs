use crate::provider::ChatStore;
use crate::ws::ChatMessage;
use crate::WebsocketService;
use chrono::Utc;
use futures::{channel::mpsc::Sender, SinkExt};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use yewdux::prelude::*;

#[function_component(ChatPage)]
pub fn chat_page() -> Html {
    let send_message = use_state(|| None::<ChatMessage>);
    let (store, dispatch) = use_store::<ChatStore>();

    let ws = use_mut_ref(|| WebsocketService::new(dispatch));
    let onclick = onclick_callback(send_message.clone(), ws.borrow().tx.clone());
    let onchnage = Callback::from({
        let msg = send_message.clone();
        move |e: Event| {
            let target = e.target();
            let element = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let content = element.map(|e| e.value()).unwrap();

            let value = ChatMessage {
                content,
                created_at: Utc::now().naive_utc(),
                meta: None,
            };
            msg.set(Some(value));
        }
    });

    let messages = &store.messages;
    html! {
        html! {
            <>
                <div id="introductions">
                    {
                        messages.iter().map(|chat| {
                            html!{<div>{ format!("{}",chat.content) }</div>}
                        }).collect::<Html>()
                    }
                </div>
                <div class="w-full h-14 flex px-3 items-center">
                    <input onchange={onchnage} value={(*send_message).clone().map_or("".into(), |x| x.content)} type="text" placeholder="message" class="block w-full py-2 pl-4 mx-3 bg-gray-100 rounded-full outline-none focus:text-gray-700" name="message" />
                    <button onclick={onclick} class="p-3 shadow-sm bg-blue-600 w-10 h-10 rounded-full flex justify-center items-center color-white">
                </button>
            </div>
            </>
        }
    }
}

fn onclick_callback(
    send_message: UseStateHandle<Option<ChatMessage>>,
    tx: Sender<String>,
) -> Callback<MouseEvent> {
    Callback::from({
        move |_: MouseEvent| {
            let cloned_state = send_message.clone();
            let mut tx2 = tx.clone();
            spawn_local(async move {
                let chat = (*cloned_state).clone();
                let message = serde_json::to_string(&chat).unwrap();
                tx2.send(message).await.unwrap();

                cloned_state.set(None);
            });
        }
    })
}
