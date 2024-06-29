use std::error::Error;
use std::io;
use std::process::Command;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use defguard_wireguard_rs::key::Key;
use defguard_wireguard_rs::WireguardInterfaceApi;

use crate::data::wireguard_peer::WireGuardPeer;
use crate::WireGuardAppValues;

pub fn get_peers(
    app_values: Arc<Mutex<WireGuardAppValues>>,
) -> Result<Vec<WireGuardPeer>, Box<dyn Error>> {
    let app_values = app_values.lock().unwrap();
    let raw_peers = &app_values.wg_api.read_interface_data()?.peers;
    let mut peers = Vec::<WireGuardPeer>::new();

    for client in &app_values.wireguard_data.clients {
        let key = Key::from_str(&client.public_key)?;
        let raw_peer = &raw_peers[&key];
        peers.push(WireGuardPeer {
            name: client.name.clone(),
            uuid: client.uuid,
            server_allowed_ips: raw_peer.allowed_ips.clone(),
            address: client.address.clone(),
            protocol_version: raw_peer.protocol_version,
            endpoint: raw_peer.endpoint,
            dns: client.dns.clone(),
            transmitted_bytes: raw_peer.tx_bytes,
            received_bytes: raw_peer.rx_bytes,
            last_handshake: raw_peer.last_handshake,
        })
    }

    Ok(peers)
}

pub fn restart_wireguard(interface: &String) {
    Command::new("wg-quick").arg("down").arg(interface).output().expect("Failed to start WireGuard");
    Command::new("wg-quick").arg("up").arg(interface).output().expect("Failed to start WireGuard");
}