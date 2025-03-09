use super::theme::{Dark, Light, ThemeAction};
use super::AppContext;
use wasm_bindgen::prelude::*;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(SideNavBar)]
pub fn sidenavbar() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    use_async_with_options(
        {
            let app_context = app_context.clone();
            async move {
                if let Ok(int) = serde_wasm_bindgen::from_value::<Vec<String>>(
                    invoke("get_interfaces", JsValue::NULL).await,
                ) {
                    app_context.interfaces.set(int);
                }
                Ok::<(), ()>(())
            }
        },
        UseAsyncOptions::enable_auto(),
    );

    let cycle_theme = {
        let app_context = app_context.clone();
        let current_theme: &str = app_context.theme.current;
        let current_theme_index: usize = match app_context
            .theme_cycle
            .iter()
            .position(|x: &&str| x == &current_theme)
        {
            Some(i) => i,
            None => 0,
        };
        let next_theme: &str = match app_context.theme_cycle.iter().nth(current_theme_index + 1) {
            Some(nt) => nt,
            None => "light",
        };
        Callback::from(move |_| match next_theme {
            "dark" => app_context.theme.dispatch(ThemeAction::Dark),
            "light" | _ => app_context.theme.dispatch(ThemeAction::Light),
        })
    };

    fn handle_theme_icon(app_context: AppContext) -> Html {
        match app_context.theme.current {
            "dark" => {
                html! {<Dark class={Some("cursor-pointer h-[1.5rem] w-[1.5rem] fill-slate-300")} />}
            }
            "light" | _ => {
                html! {<Light class={Some("cursor-pointer h-[1.5rem] w-[1.5rem] fill-orange-400")} />}
            }
        }
    }

    let show_mobile_nav: UseStateHandle<bool> = use_state(|| false);

    let toggle_mobile_nav = {
        let show_mobile_nav: UseStateHandle<bool> = show_mobile_nav.clone();
        Callback::from(move |_| show_mobile_nav.set(!*show_mobile_nav))
    };

    let choosed_interface = use_node_ref();
    let on_change_interface = {
        let choosed_interface = choosed_interface.clone();
        let app_context = app_context.clone();
        Callback::from(move |_| {
            if let Some(interface) = choosed_interface.cast::<HtmlSelectElement>() {
                app_context.interface.set(interface.value())
            }
        })
    };

    html! {
        <>
                    <nav class=" z-40 w-full bg-white border-b border-gray-200 dark:bg-gray-800 dark:border-gray-700">
              <div class="px-3 py-1 lg:px-5 lg:pl-3">
                <div class="flex items-center justify-between">
                  <div class="flex items-center justify-start rtl:justify-end">
                    <button
                        onclick={ toggle_mobile_nav }
                        data-drawer-target="logo-sidebar" data-drawer-toggle="logo-sidebar" aria-controls="logo-sidebar" type="button" class="inline-flex items-center p-2 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600">
                        <span class="sr-only">{"Open sidebar"}</span>
                        <svg class="w-6 h-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                           <path clip-rule="evenodd" fill-rule="evenodd" d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z"></path>
                        </svg>
                     </button>
         <img src="../static/logo.png" class="h-12" alt="Flowbite Logo" />
            <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">{"NetWatch"}</span>

                  </div>
                  <div class="flex items-center">
                      <div class="flex items-center ms-3">
    <form class="mx-auto px-2">
      <label for="interfaces" class="text-sm font-medium text-gray-900 dark:text-white">{"Interface"}</label>
      <select onchange={on_change_interface} ref={choosed_interface.clone()} id="interfaces" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
        <option selected=true>{"any"}</option>
     { for (*(app_context.interfaces)).iter().map(|int_name| html!{
        <option >{int_name}</option>
     })}
      </select>
    </form>
                            <a onclick={ cycle_theme }>
                                { handle_theme_icon(app_context.clone()) }
                            </a>
                        <div>
                        </div>
                    </div>
                   </div>
                </div>
              </div>
            </nav>
    //            <aside id="logo-sidebar" class={format!("fixed top-0 left-0 z-30 w-64 h-screen pt-20 transition-transform {}  bg-white border-r border-gray-200 sm:translate-x-0 dark:bg-gray-800 dark:border-gray-700", if *show_mobile_nav { "translate-x-0" } else { "-translate-x-full" }) } aria-label="Sidebar">
    //           <div class="h-full px-3 pb-4 overflow-y-auto bg-white dark:bg-gray-800">
    //              <ul class="space-y-2 font-medium">
    //                 <li>
    //                    <a class="cursor-pointer flex items-center p-2 text-gray-900 rounded-lg dark:text-white   group bg-gray-100 dark:bg-gray-700">
    //                       <svg class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 22 21">
    //                          <path d="M16.975 11H10V4.025a1 1 0 0 0-1.066-.998 8.5 8.5 0 1 0 9.039 9.039.999.999 0 0 0-1-1.066h.002Z"/>
    //                          <path d="M12.5 0c-.157 0-.311.01-.565.027A1 1 0 0 0 11 1.02V10h8.975a1 1 0 0 0 1-.935c.013-.188.028-.374.028-.565A8.51 8.51 0 0 0 12.5 0Z"/>
    //                       </svg>
    //                       <span class="ms-3">{"Головна"}</span>
    //                    </a>
    //                 </li>
    //                 <li>
    //                 </li>
    //              </ul>
    //           </div>
    //        </aside>

        </>


        }
}
