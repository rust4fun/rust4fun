use crate::component::ChatComp;
use yew::prelude::*;

#[function_component(SpherePage)]
pub fn sphere_page() -> Html {
    html! {
        <ChatComp/>
    }
}

// fn onclick_callback(
//     send_message: UseStateHandle<Option<ChatMessage>>,
//     tx: Sender<String>,
// ) -> Callback<MouseEvent> {
//     Callback::from({
//         move |_: MouseEvent| {
//             let cloned_state = send_message.clone();
//             let mut tx2 = tx.clone();
//             spawn_local(async move {
//                 let chat = (*cloned_state).clone();
//                 let message = serde_json::to_string(&chat).unwrap();
//                 tx2.send(message).await.unwrap();

//                 cloned_state.set(None);
//             });
//         }
//     })
// }
