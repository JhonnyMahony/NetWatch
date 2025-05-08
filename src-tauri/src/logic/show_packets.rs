use std::net::IpAddr;

use chrono::Local;
use pnet::packet::{
    arp::{ArpOperation, ArpOperations, ArpPacket},
    ethernet::{EtherTypes, EthernetPacket},
    icmp::{IcmpPacket, IcmpType},
    icmpv6::{Icmpv6Packet, Icmpv6Type, Icmpv6Types},
    ip::{
        IpNextHeaderProtocol,
        IpNextHeaderProtocols::{self, Tlsp},
    },
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    tcp::{TcpFlags, TcpPacket},
    udp::UdpPacket,
    Packet,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FormatedPacket {
    pub number: u32,
    pub time: String,
    pub src_ip: IpAddr,
    pub dst_ip: IpAddr,
    pub protocol: String,
    pub lenght: usize,
    pub info: String,
    pub detailed_info: Option<DetailedInfo>,
    #[serde(skip)]
    pub timestamp: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DetailedInfo {
    pub interface: String,
    pub src_mac: String,
    pub dst_mac: String,
    pub frame_type: String,
    pub payload_length: usize,
    pub packet_length: usize,
    pub payload_data: String,
}

impl FormatedPacket {
    fn new(
        number: u32,
        src_ip: IpAddr,
        dst_ip: IpAddr,
        protocol: String,
        lenght: usize,
        info: String,
    ) -> Self {
        let now = Local::now();
        let time = now.time().format("%H:%M:%S").to_string();
        let timestamp = now.timestamp() as f64 + now.timestamp_subsec_nanos() as f64 / 1e9;

        Self {
            number,
            time,
            src_ip,
            dst_ip,
            lenght,
            protocol,
            info,
            detailed_info: None,
            timestamp,
        }
    }
}

pub fn get_payload_data(payload: &[u8]) -> String {
    let mut result = String::new();
    let mut offset = 0;
    let line_width = 16;

    while offset < payload.len() {
        let end = std::cmp::min(offset + line_width, payload.len());
        let line = &payload[offset..end];

        result.push_str(&format!("{:04x}  ", offset));

        for (i, byte) in line.iter().enumerate() {
            result.push_str(&format!("{:02x} ", byte));
            if i == 7 {
                result.push_str(" ");
            }
        }

        if line.len() < line_width {
            let padding = (line_width - line.len()) * 3 + if line.len() > 7 { 1 } else { 0 };
            result.push_str(&" ".repeat(padding));
        }

        result.push_str(" ");
        for &byte in line {
            let c = if byte >= 32 && byte < 127 {
                byte as char
            } else {
                '.'
            };

            result.push(c);
        }
        result.push('\n');

        offset += line_width;
    }
    result
}

pub fn process_arp_packet(
    ethernet: &EthernetPacket,
    count_fp: &mut u32,
    t: String,
) -> Option<FormatedPacket> {
    if let Some(arp) = ArpPacket::new(ethernet.payload()) {
        *count_fp += 1;
        let src_mac = arp.get_sender_hw_addr();
        let dst_mac = arp.get_target_hw_addr();
        let operation = match arp.get_operation() {
            ArpOperations::Reply => "Reply",
            ArpOperations::Request => "Request",
            _ => "Unknown",
        };
        let info = format!(
            "src mac: {}, dst mac: {}, type: {}",
            src_mac, dst_mac, operation
        );
        return Some(FormatedPacket::new(
            *count_fp,
            IpAddr::V4(arp.get_sender_proto_addr()),
            IpAddr::V4(arp.get_target_proto_addr()),
            t,
            arp.packet().len(),
            info,
        ));
    };
    None
}

pub fn process_ipv6_packet(
    ethernet: &EthernetPacket,
    count_fp: &mut u32,
) -> Option<FormatedPacket> {
    if let Some(ipv6) = Ipv6Packet::new(ethernet.payload()) {
        let payload = ipv6.payload();
        let pr = process_ip_traffic(payload, ipv6.get_next_header());
        if let Some(data) = pr.0 {
            *count_fp += 1;
            return Some(FormatedPacket::new(
                *count_fp,
                IpAddr::V6(ipv6.get_source()),
                IpAddr::V6(ipv6.get_destination()),
                pr.1,
                payload.len(),
                data,
            ));
        }
    }
    None
}

pub fn process_ipv4_packet(
    ethernet: &EthernetPacket,
    count_fp: &mut u32,
) -> Option<FormatedPacket> {
    if let Some(ipv4) = Ipv4Packet::new(ethernet.payload()) {
        let payload = ipv4.payload();
        let pr = process_ip_traffic(payload, ipv4.get_next_level_protocol());
        if let Some(data) = pr.0 {
            *count_fp += 1;
            return Some(FormatedPacket::new(
                *count_fp,
                IpAddr::V4(ipv4.get_source()),
                IpAddr::V4(ipv4.get_destination()),
                pr.1,
                payload.len(),
                data,
            ));
        }
    }
    None
}

fn process_ip_traffic(payload: &[u8], protocol: IpNextHeaderProtocol) -> (Option<String>, String) {
    match protocol {
        IpNextHeaderProtocols::Tcp => (process_tcp_packet(payload), "TCP".to_string()),
        IpNextHeaderProtocols::Udp => (process_udp_packet(payload), "UDP".to_string()),
        IpNextHeaderProtocols::Icmp => (process_icmp_packet(payload), "ICMP".to_string()),
        IpNextHeaderProtocols::Icmpv6 => (process_icmpv6_packet(payload), "ICMPv6".to_string()),
        _ => (None, "".to_string()),
    }
}

fn process_tcp_packet(payload: &[u8]) -> Option<String> {
    if let Some(tcp) = TcpPacket::new(payload) {
        let src_port = tcp.get_source();
        let dst_port = tcp.get_destination();

        let flags = get_tcp_flags(&tcp);
        return Some(format!(
            "src port {}, dst port {}, flags {}",
            src_port, dst_port, flags
        ));
    }
    None
}

fn process_udp_packet(payload: &[u8]) -> Option<String> {
    if let Some(udp) = UdpPacket::new(payload) {
        let src_port = udp.get_source();
        let dst_port = udp.get_destination();
        return Some(format!("src port {}, dst port {}", src_port, dst_port));
    }
    None
}

fn process_icmp_packet(payload: &[u8]) -> Option<String> {
    if let Some(icmp) = IcmpPacket::new(payload) {
        return Some(format!("type {}", icmp.get_icmp_type().0));
    }
    None
}
fn process_icmpv6_packet(payload: &[u8]) -> Option<String> {
    if let Some(icmp) = Icmpv6Packet::new(payload) {
        return Some(format!("type {}", icmp.get_icmpv6_type().0));
    }
    None
}

fn get_tcp_flags(tcp: &TcpPacket) -> String {
    let flags = tcp.get_flags();
    let flag_names = [
        (TcpFlags::SYN, "SYN"),
        (TcpFlags::ACK, "ACK"),
        (TcpFlags::FIN, "FIN"),
        (TcpFlags::RST, "RST"),
        (TcpFlags::PSH, "PSH"),
        (TcpFlags::URG, "URG"),
    ];

    flag_names
        .iter()
        .filter_map(|&(flag, name)| (flags & flag != 0).then(|| name))
        .collect::<Vec<_>>()
        .join("|")
}
