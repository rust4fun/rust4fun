use crate::component::NavigationBar;
use crate::store::AuthStore;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

pub struct DashboardPage {
    auth_store: Rc<AuthStore>,
}

impl Component for DashboardPage {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let auth_store = Dispatch::<AuthStore>::global().get();

        Self { auth_store }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let user_name = self
            .auth_store
            .user
            .as_ref()
            .map(|x| x.name.clone())
            .unwrap_or_default();
        let display = format!("Hello {user_name}");

        html! {
          <>
            <NavigationBar/ >
            <div class="text-center py-10 px-4 sm:px-6 lg:px-8">
                <h1 class="block text-7xl font-bold text-gray-800 sm:text-9xl dark:text-white">{display}</h1>
            </div>
          </>
        }
    }
}
