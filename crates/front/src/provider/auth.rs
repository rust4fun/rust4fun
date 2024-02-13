use yew::ContextProvider;
use yewdux::Store;

pub type AuthProvider = ContextProvider<AuthStore>;

#[derive(Clone, Debug, PartialEq, Default, Store)]
pub struct AuthStore {
    pub is_authorization: bool,
}
