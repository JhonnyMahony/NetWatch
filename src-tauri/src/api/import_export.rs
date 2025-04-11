use std::{collections::VecDeque, fs, sync::Mutex};

use tauri::Manager;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use crate::{errors::AppResult, logic::show_packets::FormatedPacket};

#[tauri::command]
pub async fn export_packets(app_handle: tauri::AppHandle) -> AppResult<()> {
    let file_path = app_handle
        .dialog()
        .file()
        .set_file_name("packets.json")
        .add_filter("My Filter", &["json"])
        .blocking_save_file();
    match file_path {
        Some(path) => {
            let packets = app_handle
                .try_state::<Mutex<VecDeque<FormatedPacket>>>()
                .ok_or_else(|| anyhow::anyhow!("Packets state not found"))?;

            let packets = packets
                .lock()
                .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?
                .clone();

            let serialized_data =
                serde_json::to_string_pretty(&packets).map_err(anyhow::Error::from)?;
            fs::write(path.as_path().unwrap(), serialized_data)
                .map_err(|_| anyhow::anyhow!("Cant write data to file"))?;
            Ok(())
        }
        None => {
            println!("No file selected");
            Err(anyhow::anyhow!("No file selected").into())
        }
    }
}

#[tauri::command]
pub async fn import_packets(app_handle: tauri::AppHandle) -> AppResult<()> {
    let file_path = app_handle
        .dialog()
        .file()
        .add_filter("My Filter", &["json"])
        .blocking_pick_file();
    match file_path {
        Some(path) => {
            let packets = app_handle
                .try_state::<Mutex<VecDeque<FormatedPacket>>>()
                .ok_or_else(|| anyhow::anyhow!("Packets state not found"))?;

            let imported_data =
                fs::read_to_string(path.as_path().unwrap()).map_err(anyhow::Error::from)?;

            let data: VecDeque<FormatedPacket> =
                serde_json::from_str(&imported_data).map_err(anyhow::Error::from)?;

            let mut packets = packets
                .lock()
                .map_err(|e| anyhow::anyhow!("Mutex lock failed: {}", e))?;

            *packets = data;
            Ok(())
        }
        None => Err(anyhow::anyhow!("No file selected").into()),
    }
}
