use std::rc::Rc;

use crate::server::WebsocketService;
use crate::store::{fetch_message, ChatStore};
use uuid::Uuid;
use yew::prelude::*;
use yewdux::Dispatch;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub planet_id: Uuid,
}

pub enum Msg {}

pub struct Planet {
    _wss: Option<WebsocketService>,
    chat_store: Rc<ChatStore>,
}

impl Component for Planet {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let chat_dispatch = Dispatch::<ChatStore>::global();
        let _wss = WebsocketService::new_open(ctx.props().planet_id, chat_dispatch.clone());

        fetch_message(ctx.props().planet_id);
        let chat_store = chat_dispatch.get();

        Self {
            _wss: Some(_wss),
            chat_store,
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let _messages = &self.chat_store.messages;

        use crate::server::ChatMessage;
        let messages = vec![
            ChatMessage {
                content: "hello!".into(),
                created_at: chrono::Utc::now().naive_utc(),
                meta: None,
            },
            ChatMessage {
                content: "hello!".into(),
                created_at: chrono::Utc::now().naive_utc(),
                meta: None,
            },
            ChatMessage {
                content: "hello!".into(),
                created_at: chrono::Utc::now().naive_utc(),
                meta: None,
            },
            ChatMessage {
                content: "hello!".into(),
                created_at: chrono::Utc::now().naive_utc(),
                meta: None,
            },
            ChatMessage {
                content: "hello!".into(),
                created_at: chrono::Utc::now().naive_utc(),
                meta: None,
            },
            ChatMessage {
                content: "hello!".into(),
                created_at: chrono::Utc::now().naive_utc(),
                meta: None,
            },
            ChatMessage {
                content: "hello!".into(),
                created_at: chrono::Utc::now().naive_utc(),
                meta: None,
            },
            ChatMessage {
                content: "hello!".into(),
                created_at: chrono::Utc::now().naive_utc(),
                meta: None,
            },
            ChatMessage {
                content: "hello!".into(),
                created_at: chrono::Utc::now().naive_utc(),
                meta: None,
            },
            ChatMessage {
                content: "hello!".into(),
                created_at: chrono::Utc::now().naive_utc(),
                meta: None,
            },
            ChatMessage {
                content: "hello!".into(),
                created_at: chrono::Utc::now().naive_utc(),
                meta: None,
            },
            ChatMessage {
                content: "hello!".into(),
                created_at: chrono::Utc::now().naive_utc(),
                meta: None,
            },
        ];

        html! {
            <>
            <div class="w-4/5">
                <div class="flex flex-col">
                    <div class="relative">
                        <div class="flex flex-col overflow-auto">
                            <div class="px-6 py-4">
                                    {
                                        messages.iter().map(|chat| {
                                            tmp(chat.content.clone())
                                        }).collect::<Html>()
                                    }
                            </div>
                        </div>
                    </div>
                    // <form class="h-auto w-full sticky shadow-2xl bg-gradient-to-br from-gray-100 to-gray-300 border-l dark:border-gray-800 border-gray-200 bottom-0 flex-1 flex flex-col lg:order-last">

                    //     <div class="h-14 px-3 items-center relative">
                    //     <textarea name=”テキストエリア” rows=”3″ cols=”50″ wrap=”hard”>あらかじめ文字を表示
                    //     </textarea>
                    //         <input type="text" placeholder="message" class="block w-full py-2 pl-4 mx-3 bg-gray-100 rounded-none outline-none focus:text-gray-700" name="message" />
                    //         <button class="p-3 shadow-sm bg-blue-600 w-10 h-10 rounded-full flex justify-center items-center color-white" />
                    //     </div>
                    // </form>
                </div>

            </div>
            </>
        }
    }

    fn destroy(&mut self, _: &Context<Self>) {
        self._wss = None
    }
}

fn tmp(message: String) -> Html {
    html! {
        <div class="border-gray-600 py-3 flex items-start mb-4 text-sm">
        <img src="https://cdn.discordapp.com/embed/avatars/2.png" class="h-6 w-6 rounded-full mr-3"/>
            <div class="flex-1 overflow-hidden">
                <div>
                    <span class="font-bold text-red-300 cursor-pointer hover:underline">{"name"}</span>
                    <span class="font-bold text-gray-400 text-xs">{"10:00"}</span>
                </div>
                <p class="text-black leading-normal">{message}</p>
            </div>
        </div>
    }
}
