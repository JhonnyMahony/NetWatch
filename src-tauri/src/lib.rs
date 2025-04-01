use std::{
    collections::VecDeque,
    net::IpAddr,
    sync::Mutex,
    thread::{self, sleep},
    time::Duration,
};

use logic::{
    create_chanel, get_interface,
    show_packets::{
        get_payload_data, process_arp_packet, process_ipv4_packet, process_ipv6_packet,
        DetailedInfo, FormatedPacket,
    },
};
use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    Packet,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Manager;

mod logic;

#[derive(Serialize, Deserialize)]
struct FilterState {
    ip: IpAddr,
    protocol: String,
}

#[tauri::command]
fn get_interfaces() -> Vec<String> {
    pnet::datalink::interfaces()
        .iter()
        .map(|int| int.name.clone())
        .collect()
}

#[tauri::command]
fn start_watch(app_handle: tauri::AppHandle, interface: String) {
    log::info!("start watch");
    if let Some(running) = app_handle.try_state::<AtomicBool>() {
        running.store(true, Ordering::SeqCst);
    }
    if let Some(packets) = app_handle.try_state::<Mutex<VecDeque<FormatedPacket>>>() {
        packets.lock().unwrap().clear();
    }

    let interface = get_interface(interface);
    log::info!("listen on: {}", interface.name);

    thread::spawn(move || {
        let mut count_fp = 0;
        let (_, mut rx) = create_chanel(interface.clone());

        let handle = app_handle.state::<AtomicBool>();
        while handle.load(Ordering::SeqCst) {
            if let Ok(packet) = rx.next() {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    let formated_packet = match ethernet_packet.get_ethertype() {
                        EtherTypes::Arp => {
                            process_arp_packet(&ethernet_packet, &mut count_fp, "ARP".to_string())
                        }
                        EtherTypes::Rarp => {
                            process_arp_packet(&ethernet_packet, &mut count_fp, "RARP".to_string())
                        }
                        EtherTypes::Ipv4 => process_ipv4_packet(&ethernet_packet, &mut count_fp),
                        EtherTypes::Ipv6 => process_ipv6_packet(&ethernet_packet, &mut count_fp),
                        _ => None,
                    };
                    if let Some(mut fp) = formated_packet {
                        let src_mac = ethernet_packet.get_source().to_string();
                        let dst_mac = ethernet_packet.get_destination().to_string();
                        let frame_type = ethernet_packet.get_ethertype().to_string();
                        let payload_length = ethernet_packet.payload().len();
                        let packet_length = ethernet_packet.packet().len();
                        let payload_data = get_payload_data(ethernet_packet.payload());

                        let det_inf = DetailedInfo {
                            src_mac,
                            dst_mac,
                            frame_type,
                            packet_length,
                            payload_length,
                            interface: interface.name.clone(),
                            payload_data,
                        };
                        fp.detailed_info = Some(det_inf);

                        if let Some(packets) =
                            app_handle.try_state::<Mutex<VecDeque<FormatedPacket>>>()
                        {
                            packets.lock().unwrap().push_back(fp);
                        }
                    }
                }
            }
            sleep(Duration::from_millis(100));
        }
        println!("loop stopped");
    });
}

#[tauri::command]
fn stop_watch(app_handle: tauri::AppHandle) {
    log::info!("stoped watch");
    if let Some(running) = app_handle.try_state::<AtomicBool>() {
        running.store(false, Ordering::SeqCst);
    }
}

#[tauri::command]
fn get_packets(
    app_handle: tauri::AppHandle,
    protocol: String,
    ip: String,
) -> VecDeque<FormatedPacket> {
    log::info!("filtered protocol: {}, ip: {}", protocol, ip);
    if let Some(packets) = app_handle.try_state::<Mutex<VecDeque<FormatedPacket>>>() {
        let locked_packets = match packets.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let packets = locked_packets
            .iter() // Use iter() instead of cloning
            .filter(|packet| {
                (protocol == "all" || protocol == packet.protocol)
                    && (ip.is_empty()
                        || packet.src_ip.to_string().starts_with(&ip)
                        || packet.dst_ip.to_string().starts_with(&ip))
            })
            .cloned()
            .collect::<VecDeque<FormatedPacket>>();
        packets
    } else {
        VecDeque::new()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(VecDeque::<FormatedPacket>::new()))
        .manage(AtomicBool::new(true))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_watch,
            stop_watch,
            get_interfaces,
            get_packets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
