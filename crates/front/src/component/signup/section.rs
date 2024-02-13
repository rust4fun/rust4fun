use super::form::SignupFrom;
use crate::auth::{AuthClientError, AuthRequester, AuthResponse};
use crate::provider::AuthStore;
use crate::router::Route;
use gloo::storage::{LocalStorage, Storage};
use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_bootstrap::component::{Column, Container, Row};
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
        <section class="vh-100" style="background-color: #eee;">
        <Container class="h-100">
          <Row class="d-flex justify-content-center align-items-center h-100">
            <Column lg=12 xl=11>
              <div class="card text-black" style="border-radius: 25px;">
                <div class="card-body p-md-5">
                  <Row class="justify-content-center">
                    <Column md=10 lg=6 xl=5 class="order-2 order-lg-1">
                        <p class="text-center h1 fw-bold mb-5 mx-1 mx-md-4 mt-4">{"Sign up"}</p>
                        <SignupFrom
                            onclick={onclick}
                            input_name={handle_name_input.clone()}
                            input_email={handle_email_input.clone()}
                            input_password={handle_password_input.clone()}
                        />
                    </Column>
                    <Column md=10 lg=6 xl=7 class="d-flex align-items-center order-1 order-lg-2">
                      <img src="assets/logo-card.png" class="img-fluid" alt="Logo Card"/>
                    </Column>
                  </Row>
                </div>
              </div>
            </Column>
          </Row>
        </Container>
      </section>
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
