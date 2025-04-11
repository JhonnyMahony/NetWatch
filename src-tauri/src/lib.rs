use std::{
    collections::VecDeque,
    sync::{atomic::AtomicBool, Mutex},
};

use logic::show_packets::FormatedPacket;

mod api;
mod errors;
mod logic;

use api::import_export::{export_packets, import_packets};
use api::packets::{get_interfaces, get_packets, start_watch, stop_watch};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(VecDeque::<FormatedPacket>::new()))
        .manage(AtomicBool::new(true))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            //packets
            start_watch,
            stop_watch,
            get_interfaces,
            get_packets,
            //import_export
            import_packets,
            export_packets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
