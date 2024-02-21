use crate::auth::{AuthClientError, AuthRequester, AuthResponse};
use crate::component::common::{AuthFrom, InputField, Toast, ToastLevel};
use crate::provider::AuthStore;
use crate::router::Route;
use garde::Validate;
use gloo::storage::{LocalStorage, Storage};
use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::navigator::Navigator;
use yewdux::{use_store, Dispatch};

#[derive(Debug, Default, PartialEq, Clone, Validate)]
#[garde(transparent)]
pub struct UserEmail(#[garde(email)] pub String);

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UserLogin {
    pub email: UserEmail,
    pub password: String,
}

/// 参照元
/// <https://mdbootstrap.com/docs/standard/extended/registration/>
#[function_component(LoginSection)]
pub fn login_section() -> Html {
    // definition
    let user = use_state(UserLogin::default);
    let is_valid_email = use_state(|| true);
    let invalid_email_message = use_state(String::default);
    let is_show_toast = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let (_, dispatch) = use_store::<AuthStore>();

    // handler
    let handle_email_input = input_user_callback("email", user.clone());
    let handle_password_input = input_user_callback("password", user.clone());
    let onclick = onclick_callback(
        user.clone(),
        navigator.clone(),
        dispatch.clone(),
        is_valid_email.clone(),
        invalid_email_message.clone(),
        is_show_toast.clone(),
    );

    // view
    html! {
        <>
            <div class="mx-auto max-w-screen-xl px-4 py-16 sm:px-6 lg:px-8">
                <div class="mx-auto max-w-lg text-center">
                    <h1 class="text-2xl font-bold sm:text-3xl">{"Login"}</h1>
                </div>
                <AuthFrom onclick={onclick} is_login=true label="Login">
                    <InputField
                        input_type="email"
                        label="Email"
                        input={handle_email_input.clone()}
                        is_valid={is_valid_email.clone()}
                        invalid_message={invalid_email_message}
                    />
                    <InputField
                        input_type="password"
                        label="Password"
                        input={handle_password_input.clone()}
                    />
                </AuthFrom>
            </div>
            <Toast is_show={is_show_toast} message={"Invalid email or password!"} level={ToastLevel::Error}/>
        </>
    }
}

fn input_user_callback(name: &'static str, user: UseStateHandle<UserLogin>) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = user.deref().clone();
        match name {
            "email" => data.email = UserEmail(value),
            "password" => data.password = value,
            _ => (),
        }
        user.set(data);
    })
}

fn onclick_callback(
    user: UseStateHandle<UserLogin>,
    navigator: Navigator,
    dispatch: Dispatch<AuthStore>,
    is_valid_email: UseStateHandle<bool>,
    invalid_em_msg: UseStateHandle<String>,
    is_show_toast: UseStateHandle<bool>,
) -> Callback<MouseEvent> {
    Callback::from({
        move |_: MouseEvent| {
            if user.email.validate(&()).is_err() {
                is_valid_email.set(false);
                invalid_em_msg.set("invalid email!".into())
            } else {
                is_valid_email.set(true);
            }

            if *is_valid_email {
                let cloned_state = user.clone();
                let cloned_navigator = navigator.clone();
                let cloned_dispatch = dispatch.clone();
                let is_show_toast = is_show_toast.clone();
                spawn_local(async move {
                    let res = login((*cloned_state).clone()).await;

                    match res {
                        Ok(auth) => {
                            LocalStorage::set("access_token", auth.token).ok();
                            cloned_dispatch.reduce_mut(|state| state.is_authorization = true);
                            cloned_navigator.push(&Route::Dashboard)
                        }
                        Err(_) => {
                            is_show_toast.set(true);
                        }
                    }
                })
            }
        }
    })
}

async fn login(user: UserLogin) -> Result<AuthResponse, AuthClientError> {
    let requester = AuthRequester::new();
    requester.login(user.email.0, user.password).await
}
