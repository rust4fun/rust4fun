use yew::ContextProvider;
use yewdux::Store;

pub type AuthProvider = ContextProvider<AuthStore>;

#[derive(Clone, Debug, PartialEq, Default, Store)]
pub struct AuthStore {
    pub is_authorization: bool,
    pub user: Option<UserInfo>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct UserInfo {
    pub name: String,
}

impl UserInfo {
    pub fn new(name: String) -> Self {
        UserInfo { name }
    }
}
