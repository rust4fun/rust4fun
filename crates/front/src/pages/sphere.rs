use crate::component::{Planet, SideBar};
use uuid::Uuid;
use yew::prelude::*;

#[function_component(SpherePage)]
pub fn sphere_page() -> Html {
    let planet_id = Uuid::parse_str("db28639e-9045-4a77-99a7-62e08bc615e0").unwrap();

    html! {
        <>
            <div class="flex absolute inset-0">
                <div class="flex flex-row relative overflow-hidden w-full h-full">
                    <SideBar/>
                    <Planet {planet_id} />
                </div>
            </div>
        </>
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
