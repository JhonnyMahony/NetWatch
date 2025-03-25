use std::{collections::VecDeque, fs::File, io::Write};

use tauri_plugin_dialog::FilePath;

use super::show_packets::FormatedPacket;

pub fn import_packets() {}

pub fn save_packets(
    packets: &VecDeque<FormatedPacket>,
    file_path: Option<FilePath>,
) -> Result<String, String> {
    match file_path {
        Some(path) => {
            let json_data = serde_json::to_string_pretty(&*packets)
                .map_err(|e| format!("Failed to serialize packets: {}", e))?;
            let mut file = File::create(&path.as_path().unwrap())
                .map_err(|e| format!("Failed to create file: {}", e))?;
            file.write_all(json_data.as_bytes())
                .map_err(|e| format!("Failed to write to file: {}", e))?;
            Ok(format!("File saved successfully to: {:?}", path))
        }
        None => Err("Save operation cancelled".to_string()),
    }
}
