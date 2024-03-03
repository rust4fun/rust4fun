use crate::server::ApiRequester;
use crate::structs::user::UserInfo;
use gloo::storage::{LocalStorage, Storage};
use yewdux::{Dispatch, Store};

#[derive(Clone, Debug, PartialEq, Default, Store)]
pub struct AuthStore {
    pub is_authorization: bool,
    pub user: Option<UserInfo>,
}

pub async fn init_auth(dispatch: Dispatch<AuthStore>) {
    if let Ok(token) = LocalStorage::get::<String>("access_token") {
        let client = ApiRequester::new(&token);
        match client.me().await {
            Ok(user) => {
                dispatch.reduce_mut(|s| {
                    s.is_authorization = true;
                    s.user = Some(UserInfo::new(user.name.unwrap()));
                });
            }
            Err(_) => {
                dispatch.reduce_mut(|s| {
                    s.is_authorization = false;
                    s.user = None;
                });
            }
        }
    }
}
