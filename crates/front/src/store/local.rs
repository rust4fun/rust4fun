use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Store)]
#[store(storage = "local")]
pub struct LocalStore {
    pub count: u32,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Store)]
pub struct LoadingStore {
    pub is_loading: bool,
}
