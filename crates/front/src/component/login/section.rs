use crate::component::common::{AuthFrom, FullScreenLoading, InputField, Toast, ToastLevel};
use crate::router::Route;
use crate::server::{Auth, AuthClientError, AuthRequester};
use crate::store::{init_auth, AuthStore};
use crate::structs::user::{UserEmail, UserLogin, ValidationMap};
use garde::Validate;
use gloo::storage::{LocalStorage, Storage};
use gloo::timers::future::sleep;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;
use yewdux::prelude::*;

pub enum Msg {
    InputEmail(String),
    InputPassword(String),
    ToAuthorization,
    ShowToast,
    HideToast,
    OnLoading,
    OffLoading,
}

pub struct LoginSection {
    user: UserLogin,
    pub invalid_map: ValidationMap,
    pub auth_dispatch: Dispatch<AuthStore>,
    pub is_show_toast: bool,
    pub is_loading: bool,
}

impl Component for LoginSection {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let user = UserLogin::default();
        let mut invalid_map = ValidationMap::new();
        invalid_map.insert("email".into(), (true, "".into()));
        let auth_dispatch = Dispatch::<AuthStore>::global();
        let is_show_toast = false;
        let is_loading = false;

        Self {
            user,
            invalid_map,
            auth_dispatch,
            is_show_toast,
            is_loading,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::InputEmail(email) => {
                self.user.email = UserEmail(email);

                true
            }

            Self::Message::InputPassword(pass) => {
                self.user.password = pass;

                true
            }

            Self::Message::ShowToast => {
                self.is_show_toast = true;

                true
            }

            Self::Message::HideToast => {
                self.is_show_toast = false;

                true
            }

            Self::Message::ToAuthorization => {
                let user = &self.user;
                let load_callback = ctx.link().callback(|_: ()| Self::Message::OffLoading);

                if user.email.validate(&()).is_err() {
                    self.invalid_map
                        .insert("email".into(), (false, "invalid email!".into()));
                } else {
                    self.invalid_map.insert("email".into(), (true, "".into()));
                }

                if self.invalid_map.is_valid() {
                    let dispatch = self.auth_dispatch.clone();

                    let user = user.clone();
                    let callback = ctx
                        .link()
                        .callback(|_: ()| Self::Message::ShowToast)
                        .clone();
                    let load_callback = load_callback.clone();
                    let navigator = ctx.link().navigator().unwrap();
                    spawn_local(async move {
                        let res = login(user).await;

                        match res {
                            Ok(auth) => {
                                LocalStorage::set("access_token", auth.token).ok();
                                init_auth(dispatch).await;

                                sleep(Duration::from_millis(500)).await;
                                navigator.push(&Route::Dashboard);
                            }
                            Err(_) => {
                                sleep(Duration::from_millis(500)).await;
                                callback.emit(());
                            }
                        }

                        load_callback.emit(());
                    })
                } else {
                    load_callback.emit(());
                }

                true
            }

            Self::Message::OnLoading => {
                self.is_loading = true;

                true
            }

            Self::Message::OffLoading => {
                self.is_loading = false;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // check_auth
        let store = self.auth_dispatch.get();
        let navigator = ctx.link().navigator().unwrap();

        if store.is_authorization {
            navigator.push(&Route::Dashboard);
        }

        let (is_invalid_email, invalid_email_msg) =
            self.invalid_map.get("email").unwrap_or((false, "".into()));

        html! {
            <>
                <div class="mx-auto max-w-screen-xl px-4 py-16 sm:px-6 lg:px-8">
                    <div class="mx-auto max-w-lg text-center">
                        <h1 class="text-2xl font-bold sm:text-3xl">{"Login"}</h1>
                    </div>
                    <AuthFrom
                        onclick={ctx.link().batch_callback(|_| vec![Self::Message::OnLoading, Self::Message::ToAuthorization])}
                        is_login=true
                        label="Login"
                    >
                        <InputField
                            input_type="email"
                            label="Email"
                            input={ctx.link().callback(Self::Message::InputEmail)}
                            is_valid={is_invalid_email}
                            invalid_message={invalid_email_msg.clone()}
                        />
                        <InputField
                            input_type="password"
                            label="Password"
                            input={ctx.link().callback(Self::Message::InputPassword)}
                            is_valid={true}
                        />
                    </AuthFrom>
                </div>
                if self.is_show_toast {
                    <Toast hide_toast={ctx.link().callback(|_| Self::Message::HideToast)} message={"Invalid email or password!"} level={ToastLevel::Error}/>
                }
                if self.is_loading {
                    <FullScreenLoading />
                }
            </>
        }
    }
}

async fn login(user: UserLogin) -> Result<Auth, AuthClientError> {
    let requester = AuthRequester::new();
    requester.login(user.email.0, user.password).await
}
