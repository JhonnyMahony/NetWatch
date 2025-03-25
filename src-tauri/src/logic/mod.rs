pub mod show_packets;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::{Channel, DataLinkReceiver, DataLinkSender};

pub fn get_default_interface() -> NetworkInterface {
    let interfaces = datalink::interfaces();
    interfaces
        .into_iter()
        .find(|iface| {
            iface.is_up() && !iface.is_loopback() && iface.ips.iter().any(|ip| ip.is_ipv4())
        })
        .expect("No available network interface found")
}

pub fn get_interface(interface_name: String) -> NetworkInterface {
    let interfaces = datalink::interfaces();
    interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .unwrap_or(get_default_interface())
}

pub fn create_chanel(
    interface: NetworkInterface,
) -> (Box<dyn DataLinkSender>, Box<dyn DataLinkReceiver>) {
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };
    (tx, rx)
}
