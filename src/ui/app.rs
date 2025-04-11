use std::collections::HashMap;

use crate::errors::ApiError;

use super::components::sidenavbar::SideNavBar;
use super::components::AppContext;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{platform::spawn_local, prelude::*};
use yew_hooks::{use_async, use_interval, use_state_ptr_eq};

#[derive(Deserialize, Clone, PartialEq)]
pub struct FormatedPacket {
    pub number: u32,
    pub time: String,
    pub src_ip: String,
    pub dst_ip: String,
    pub protocol: String,
    pub lenght: usize,
    pub info: String,
    pub detailed_info: Option<DetailedInfo>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct DetailedInfo {
    pub interface: String,
    pub src_mac: String,
    pub dst_mac: String,
    pub frame_type: String,
    pub payload_length: usize,
    pub packet_length: usize,
    pub payload_data: String,
}

mod start_args {
    use super::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize)]
    pub struct Args<'a> {
        pub interface: &'a str,
    }
}

mod get_args {
    use super::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize)]
    pub struct Args {
        pub ip: String,
        pub protocol: String,
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(Watch)]
pub fn watch() -> Html {
    let current_packet = use_state(|| None::<FormatedPacket>);
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");
    let packets = use_state_eq(|| Vec::<FormatedPacket>::new());
    let is_running = use_state(|| false);
    let filter_protocol = use_state(|| "all".to_string());
    let filter_ip = use_state(|| String::new());

    let get_packets = use_async({
        let filters = get_args::Args {
            ip: (*filter_ip).clone(),
            protocol: (*filter_protocol).clone(),
        };
        let packets = packets.clone();
        async move {
            let result = invoke("get_packets", to_value(&filters).unwrap()).await;
            if let Ok(pkts) = from_value::<Vec<FormatedPacket>>(result) {
                packets.set(pkts);
            }
            Ok::<(), ()>(())
        }
    });

    let watch = use_async({
        let is_running = is_running.clone();
        let choosed_interface = app_context.interface.clone();
        async move {
            if *is_running {
                is_running.set(false);
                let _ = invoke("stop_watch", JsValue::NULL).await;
            } else {
                let _ = invoke(
                    "start_watch",
                    serde_wasm_bindgen::to_value(&start_args::Args {
                        interface: &choosed_interface,
                    })
                    .unwrap(),
                )
                .await;
                is_running.set(true);
            }
            Ok::<(), ()>(())
        }
    });

    {
        let is_running = is_running.clone();
        let get_packets = get_packets.clone();

        use_interval(
            {
                let is_running = is_running.clone();
                move || {
                    if *is_running {
                        get_packets.run();
                    }
                }
            },
            if *is_running { 500 } else { 0 },
        );
    }

    let toggle_loop = {
        let watch = watch.clone();
        Callback::from(move |_| watch.run())
    };

    let choosed_protocol = use_node_ref();
    let search_input = use_node_ref();

    let set_protocol = {
        let filter_protocol = filter_protocol.clone();
        let is_running = is_running.clone();
        let choosed_protocol = choosed_protocol.clone();
        let get_packets = get_packets.clone();

        Callback::from(move |_| {
            if let Some(protocol) = choosed_protocol.cast::<HtmlSelectElement>() {
                filter_protocol.set(protocol.value());
                if !*is_running {
                    get_packets.run();
                }
            }
        })
    };

    let on_search = {
        let filter_ip = filter_ip.clone();
        let is_running = is_running.clone();
        let search_input = search_input.clone();
        let get_packets = get_packets.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if let Some(search_ip) = search_input.cast::<HtmlInputElement>() {
                filter_ip.set(search_ip.value());
                if !*is_running {
                    get_packets.run();
                }
            }
        })
    };

    //imprt_export
    let import_data = use_async({
        let get_packets = get_packets.clone();
        async move {
            if let Ok(_) = from_value::<()>(invoke("import_packets", JsValue::NULL).await) {
                get_packets.run()
            }
            Ok::<(), ApiError>(())
        }
    });

    let export_data = use_async({
        async move {
            invoke("export_packets", JsValue::NULL).await;
            Ok::<(), ApiError>(())
        }
    });

    let on_click_import = {
        let import_data = import_data.clone();
        Callback::from(move |_| import_data.run())
    };

    let on_click_export = {
        let export_data = export_data.clone();
        Callback::from(move |_| export_data.run())
    };

    // show model
    let is_show_model = use_state(|| false);
    let toggle_model = {
        let is_show_model = is_show_model.clone();

        Callback::from(move |_| is_show_model.set(!*is_show_model))
    };

    html! {
                                     <>
                                     <SideNavBar />

                                                         <div class="p-4 bg-white block sm:flex items-center justify-between border-b border-gray-200  dark:bg-gray-800 dark:border-gray-700">
                                                     <div class="w-full mb-1">
                                                         <div class="mb-4">
                                                             <h1 class="text-xl font-semibold text-gray-900 sm:text-2xl dark:text-white">{"Network traffic"}</h1>
                                                         </div>
                                                             <div class="items-center justify-between block sm:flex md:divide-x md:divide-gray-100 dark:divide-gray-700">
                                                             <div class="flex items-center mb-4 sm:mb-0">
                                                                 <form onsubmit={on_search} class="sm:pr-3">
                                                                     <label for="products-search" class="sr-only">{"Search"}</label>
                                                                     <div class="relative w-48 mt-1 sm:w-64 xl:w-96">
                                                                         <input ref={search_input} type="text" name="email" id="products-search" class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Search by ip" />
                                                                     </div>
                                                                 </form>
                                     <form class="mx-auto px-2">
                                       <label for="protocols" class="text-sm font-medium text-gray-900 dark:text-white">{"protocol"}</label>
                                       <select ref={choosed_protocol} onchange={set_protocol} id="protocols" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
                                         <option selected=true>{"all"}</option>
                                         <option >{"TCP"}</option>
                                         <option >{"UDP"}</option>
                                         <option >{"ICMP"}</option>
                                         <option >{"ICMPv6"}</option>
                                         <option >{"ARP"}</option>
                                       </select>
                                     </form>
                <div class="flex pl-0 mt-3 space-x-1 sm:pl-2 sm:mt-0">
                            <button onclick={on_click_import} class="inline-flex justify-center p-1 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">
                            <svg class="w-6 h-6" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
          <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 15v2a3 3 0 0 0 3 3h10a3 3 0 0 0 3-3v-2m-8 1V4m0 12-4-4m4 4 4-4"/>
        </svg>
        </button>
                            <button onclick={on_click_export} class="inline-flex justify-center p-1 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">
            <svg class="w-6 h-6" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
      <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 15v2a3 3 0 0 0 3 3h10a3 3 0 0 0 3-3v-2M12 4v12m0-12 4 4m-4-4L8 8"/>
    </svg>

                            </button>
                        </div>
                                                                 <div class="flex items-center w-full sm:justify-end">
                                                                     <div class="flex pl-2 space-x-1">
                                                                     </div>
                                                                 </div>
                                                             </div>
                        <div class="flex items-center ml-auto space-x-2 sm:space-x-3">
                    <button onclick={toggle_model.clone()} class="inline-flex items-center justify-center w-1/2 px-3 py-2 text-sm font-medium text-center text-gray-900 bg-white border border-gray-300 rounded-lg hover:bg-gray-100 focus:ring-4 focus:ring-primary-300 sm:w-auto dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700 dark:focus:ring-gray-700">
                        <svg class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                  <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18.5A2.493 2.493 0 0 1 7.51 20H7.5a2.468 2.468 0 0 1-2.4-3.154 2.98 2.98 0 0 1-.85-5.274 2.468 2.468 0 0 1 .92-3.182 2.477 2.477 0 0 1 1.876-3.344 2.5 2.5 0 0 1 3.41-1.856A2.5 2.5 0 0 1 12 5.5m0 13v-13m0 13a2.493 2.493 0 0 0 4.49 1.5h.01a2.468 2.468 0 0 0 2.403-3.154 2.98 2.98 0 0 0 .847-5.274 2.468 2.468 0 0 0-.921-3.182 2.477 2.477 0 0 0-1.875-3.344A2.5 2.5 0 0 0 14.5 3 2.5 2.5 0 0 0 12 5.5m-8 5a2.5 2.5 0 0 1 3.48-2.3m-.28 8.551a3 3 0 0 1-2.953-5.185M20 10.5a2.5 2.5 0 0 0-3.481-2.3m.28 8.551a3 3 0 0 0 2.954-5.185"/>
                </svg>
                {" detect with ai "}
                            </button>

                                                             <button onclick={toggle_loop} id="createProductButton" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800" type="button" data-drawer-target="drawer-create-product-default" data-drawer-show="drawer-create-product-default" aria-controls="drawer-create-product-default" data-drawer-placement="right">
                                                         {format!("{} watch", if *is_running { "Stop" } else { "Start" })}
                                                             </button>
                        </div>
                                                         </div>
                                                     </div>
                                                 </div>
                                                     <div class="flex flex-col">
                                                 <div class="overflow-x-auto">
                                                     <div class="inline-block min-w-full align-middle">
                                                         <div class="overflow-y-auto shadow h-[400px]">
                                                             <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-600">
                                                                 <thead class="bg-gray-100 dark:bg-gray-700 sticky top-0">
                                                                     <tr>
                                                                         <th scope="col" class="py-1 px-2">
                                                     {"#"}
                                                                         </th>
                                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                                     {"Time"}
                                                                         </th>
                                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                                     {"Source"}
                                                                         </th>
                                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                                     {"Destination"}
                                                                         </th>
                                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                                     {"Protocol"}
                                                                         </th>
                                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                                     {"Size"}
                                                                         </th>
                                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                                     {"quick info"}
                                                                         </th>
                                                                     </tr>
                                                                 </thead>
                                                                 <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                                                         {for (*packets).iter().map(|packet| html!{
                                                                 <tr onclick={
                                                         Callback::from({
                                                         let current_packet = current_packet.clone();
                                                         let packet = packet.clone();
                                                         move |_| {
                                                             current_packet.set(Some(packet.clone()))
                                                         }})

                                                         } class="cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700">
                                                                     <td class="w-1 px-4 py-1 text-base font-normal text-gray-800 whitespace-nowrap dark:text-gray-300">
                                                                         {packet.number}
                                                                     </td>
                                                                     <td class="w-1 px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">
                                                                         {&packet.time}
                                                                     </td>
                                                                     <td class="w-1 px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{packet.src_ip.to_string()}</td>
                                                                     <td class="w-1 px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{packet.dst_ip.to_string()}</td>
                                                                     <td class="w-1 px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{&packet.protocol}</td>
                                                                     <td class="w-1 px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{packet.lenght}</td>
                                                                     <td class="px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{&packet.info}</td>
                                                                 </tr>
                                                                 })
                                                             }
                                                                 </tbody>
                                                             </table>
                                                         </div>
                                                     </div>
                                                 </div>
                                             </div>
                                             <div class="sticky bottom-0 right-0 items-center w-full p-4 bg-gray-100 border-t border-gray-200 sm:flex sm:justify-between dark:bg-gray-800 dark:border-gray-700">
                                          <div class="flex items-center mb-4 sm:mb-0">
                                         </div>
                                         </div>
                        <div class="grid grid-cols-1 md:grid-cols-12 gap-2 p-3 h-[450px]">
                        <div class="w-full md:col-span-6 p-2 border-4 dark:border-gray-700 bg-white dark:bg-gray-800 overflow-auto max-h-[450px]">
                            <h3 class="text-gray-700 text-lg sm:text-xl font-normal dark:text-gray-300 text-center mb-4">
                                {"Detailed info"}
                            </h3>

                            { if let Some(pkg) = (*current_packet).clone() {
                                if let Some(detailed_info) = pkg.detailed_info {
                                    html! {
                                        <table class="w-full text-left border-collapse dark:bg-gray-800  text-white text-sm sm:text-base">
                                            <tbody>
                                                <tr class="border-b border-gray-200 dark:border-gray-700">
                                                    <td class="py-2 px-2 sm:px-4 font-medium text-gray-800 dark:text-gray-200">{"Frame number"}</td>
                                                    <td class="py-2 px-2 sm:px-4 text-gray-800 dark:text-gray-200">{format!("{}", pkg.number)}</td>
                                                </tr>
                                                <tr class="border-b border-gray-200 dark:border-gray-700">
                                                    <td class="py-2 px-2 sm:px-4 font-medium text-gray-800 dark:text-gray-200">{"Interface name"}</td>
                                                    <td class="py-2 px-2 sm:px-4 text-gray-800 dark:text-gray-200">{format!("{}", detailed_info.interface)}</td>
                                                </tr>
                                                <tr class="border-b border-gray-200 dark:border-gray-700">
                                                    <td class="py-2 px-2 sm:px-4 font-medium text-gray-800 dark:text-gray-200">{"Arrival time"}</td>
                                                    <td class="py-2 px-2 sm:px-4 text-gray-800 dark:text-gray-200">{format!("{}", pkg.time)}</td>
                                                </tr>
                                                <tr class="border-b border-gray-200 dark:border-gray-700">
                                                    <td class="py-2 px-2 sm:px-4 font-medium text-gray-800 dark:text-gray-200">{"Source Mac Address"}</td>
                                                    <td class="py-2 px-2 sm:px-4 text-gray-800 dark:text-gray-200">{format!("{}", detailed_info.src_mac)}</td>
                                                </tr>
                                                <tr class="border-b border-gray-200 dark:border-gray-700">
                                                    <td class="py-2 px-2 sm:px-4 font-medium text-gray-800 dark:text-gray-200">{"Destination Mac Address"}</td>
                                                    <td class="py-2 px-2 sm:px-4 text-gray-800 dark:text-gray-200">{format!("{}", detailed_info.dst_mac)}</td>
                                                </tr>
                                                <tr class="border-b border-gray-200 dark:border-gray-700">
                                                    <td class="py-2 px-2 sm:px-4 font-medium text-gray-800 dark:text-gray-200">{"Frame protocol name"}</td>
                                                    <td class="py-2 px-2 sm:px-4 text-gray-800 dark:text-gray-200">{format!("{}", detailed_info.frame_type)}</td>
                                                </tr>
                                                <tr class="border-b border-gray-200 dark:border-gray-700">
                                                    <td class="py-2 px-2 sm:px-4 font-medium text-gray-800 dark:text-gray-200">{"Payload length"}</td>
                                                    <td class="py-2 px-2 sm:px-4 text-gray-800 dark:text-gray-200">{format!("{} bytes, ({}) bits", detailed_info.payload_length, detailed_info.payload_length * 8)}</td>
                                                </tr>
                                                <tr class="border-b border-gray-200 dark:border-gray-700">
                                                    <td class="py-2 px-2 sm:px-4 font-medium text-gray-800 dark:text-gray-200">{"Packet length"}</td>
                                                    <td class="py-2 px-2 sm:px-4 text-gray-800 dark:text-gray-200">{format!("{} bytes, ({}) bits", detailed_info.packet_length, detailed_info.packet_length * 8)}</td>
                                                </tr>
                                            </tbody>
                                        </table>
                                    }
                                } else {
                                    html! {
                                        <a class="text-sm sm:text-base font-medium text-gray-400 text-center block py-4">{"No additional info"}</a>
                                    }
                                }
                            } else {
                                html! {
                                    <a class="text-sm sm:text-base font-medium text-gray-400 text-center block py-4">{"Click on table element to show more info about"}</a>
                                }
                            }}
                        </div>
                    <div class="w-full md:col-span-6 p-2 border-4 dark:border-gray-700 bg-white dark:bg-gray-800 overflow-auto max-h-[450px]">
                            { if let Some(current_packet) = (*current_packet).clone() {
                                if let Some(detailed_info) = current_packet.detailed_info {
                            html!{
                            {(*detailed_info.payload_data).lines().map(|line| {
                                if line.trim().is_empty() {
                                    return html! { <div></div> };
                                }

                                let offset = &line[0..4];
                                let hex = &line[4..55].trim_end();
                                let ascii = &line[55..].trim_end();

                                html! {
                                    <div class="flex items-center text-sm font-mono">
                                        <span class="w-12 text-blue-700">{offset}</span>
                                        <span class="w-[400px] text-gray-900">{hex}</span>
                                        <span class="text-green-700">{ascii}</span>
                                    </div>
                                }
                            }).collect::<Html>()}
                            }
                            }else{
                            html!{}

                        }
                    }else{
                    html!{}

                    }
                            }
                        </div>
                    </div>

                <div id="popup-modal" tabindex="-1" class={format!("{} fixed flex bg-black bg-opacity-30 shadow left-0 right-0 z-50 items-center justify-center overflow-x-hidden overflow-y-auto inset-0 h-full", if *is_show_model{ ""} else{"hidden"})}>
                <div class="relative p-4 w-full max-w-md max-h-full">
                    <div class="relative bg-white rounded-lg shadow-sm dark:bg-gray-700">
                        <button onclick={toggle_model.clone()} type="button" class="absolute top-3 end-2.5 text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white" data-modal-hide="popup-modal">
                            <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                            </svg>
                            <span class="sr-only">{"Close modal"}</span>
                        </button>
                        <div class="p-4 md:p-5 text-center">
                            <svg class="mx-auto mb-4 text-gray-400 w-12 h-12 dark:text-gray-200" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 11V6m0 8h.01M19 10a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"/>
                            </svg>
                            <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">{"Dos Detected"}</h3>
                            <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">{"Ip: 192.168.33.2"}</h3>

                            <button data-modal-hide="popup-modal" type="button" class="text-white bg-red-600 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 dark:focus:ring-red-800 font-medium rounded-lg text-sm inline-flex items-center px-5 py-2.5 text-center">
            {"Block ip"}
                            </button>
                            <button onclick={toggle_model.clone()} data-modal-hide="popup-modal" type="button" class="py-2.5 px-5 ms-3 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700">{" No, cancel "}</button>
                        </div>
                    </div>
                </div>
            </div>
                                     </>
                                                     }
}
