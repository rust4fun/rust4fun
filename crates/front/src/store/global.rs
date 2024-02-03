use yewdux::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct GlobalStore {
    pub page_loading: bool,
}

// fn set_page_loading(loading: bool, dispatch: Dispatch<GlobalStore>) {
//     dispatch.reduce_mut(move |store| {
//         store.page_loading = loading;
//     })
// }
