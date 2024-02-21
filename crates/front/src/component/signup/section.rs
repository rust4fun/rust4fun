use super::form::SignupFrom;
use crate::auth::{AuthClientError, AuthRequester, AuthResponse};
use crate::provider::AuthStore;
use crate::router::Route;
use gloo::storage::{LocalStorage, Storage};
use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::navigator::Navigator;
use yewdux::{use_store, Dispatch};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct User {
    name: String,
    email: String,
    password: String,
}

/// 参照元
/// <https://mdbootstrap.com/docs/standard/extended/registration/>
#[function_component(SignupSection)]
pub fn signup_section() -> Html {
    // definition
    let user = use_state(User::default);
    let navigator = use_navigator().unwrap();
    let (_, dispatch) = use_store::<AuthStore>();

    // handler
    let handle_name_input = input_user_callback("name", user.clone());
    let handle_email_input = input_user_callback("email", user.clone());
    let handle_password_input = input_user_callback("password", user.clone());
    let onclick = onclick_callback(user.clone(), navigator.clone(), dispatch.clone());

    // view
    html! {
        <div class="mx-auto max-w-screen-xl px-4 py-16 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-lg text-center">
                <h1 class="text-2xl font-bold sm:text-3xl">{"Login"}</h1>
            </div>
            <SignupFrom
                onclick={onclick}
                input_name={handle_name_input.clone()}
                input_email={handle_email_input.clone()}
                input_password={handle_password_input.clone()}
            />
        </div>
    }
}

fn input_user_callback(name: &'static str, user: UseStateHandle<User>) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = user.deref().clone();
        match name {
            "name" => data.name = value,
            "email" => data.email = value,
            "password" => data.password = value,
            _ => (),
        }
        user.set(data);
    })
}

fn onclick_callback(
    user: UseStateHandle<User>,
    navigator: Navigator,
    dispatch: Dispatch<AuthStore>,
) -> Callback<MouseEvent> {
    Callback::from({
        move |_: MouseEvent| {
            let cloned_state = user.clone();
            let cloned_navigator = navigator.clone();
            let cloned_dispatch = dispatch.clone();

            spawn_local(async move {
                let res = signup((*cloned_state).clone()).await;

                match res {
                    Ok(auth) => {
                        LocalStorage::set("access_token", auth.token).ok();
                        cloned_dispatch.reduce_mut(|state| state.is_authorization = true);
                        cloned_navigator.push(&Route::Dashboard)
                    }
                    Err(e) => {
                        log::error!("{e:?}")
                    }
                }
            })
        }
    })
}

async fn signup(user: User) -> Result<AuthResponse, AuthClientError> {
    let requester = AuthRequester::new();
    requester.signup(user.name, user.email, user.password).await
}
