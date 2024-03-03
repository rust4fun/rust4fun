use super::platnet_item::PlanetItem;
use super::sphere_title_item::SphereTitleItem;
use yew::prelude::*;

use crate::router::Route;
use yew_router::prelude::*;

#[function_component(SideBar)]
pub fn side_bar() -> Html {
    let icon = html! {
      <div class="inline-flex size-16 items-center justify-center">
        <span class="grid size-10 place-content-center rounded-lg bg-gray-100 text-xs text-gray-600">
          {"N"}
        </span>
      </div>
    };

    let planets = html! {
      <div class="flex h-screen flex-1 flex-col justify-between border-e bg-white">
      <div class="px-4 py-3">
        <ul class="space-y-1">
          <SphereTitleItem title={"title"}/>

          <li>
            <details class="group [&_summary::-webkit-details-marker]:hidden">
              <summary
                class="flex cursor-pointer items-center justify-between rounded-lg px-4 py-2 text-gray-500 hover:bg-gray-100 hover:text-gray-700"
              >
                <span class="text-sm font-medium"> {"Teams"} </span>
              </summary>

              <ul class="mt-2 space-y-1 px-4">
                <PlanetItem planet_name={"Banned Users"}/>
                <PlanetItem planet_name={"Calendar"}/>
              </ul>
            </details>
          </li>
        </ul>
      </div>
    </div>
    };

    let a = "group relative flex justify-center rounded px-2 py-1.5 text-gray-500 hover:bg-gray-50 hover:text-gray-700";

    let sphere = html! {
      <div class="flex h-screen w-16 flex-col justify-between border-e bg-white">
      <div>
        {icon}

        <div class="border-t border-gray-100">
          <div class="px-2">
            <ul class="space-y-1 border-t border-gray-100 pt-4">
              <li>
                <Link<Route> to={Route::Index} classes={classes!(a)}>
                  <i class="las la-home"></i>
                  <span
                    class="invisible absolute start-full top-1/2 ms-4 -translate-y-1/2 rounded bg-gray-900 px-2 py-1.5 text-xs font-medium text-white group-hover:visible"
                  >
                    {"Home"}
                  </span>
                </Link<Route>>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
    };

    html! {
        <>
        <div class="py-5 w-1/5 h-full">
          <div class="flex flex-row">
          {sphere}
          {planets}
          </div>
        </div>
        </>
    }
}
