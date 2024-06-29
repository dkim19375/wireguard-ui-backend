use defguard_wireguard_rs::net::IpAddrMask;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardPeer {
    pub name: String,
    #[serde(serialize_with = "uuid::serde::simple::serialize")]
    pub uuid: Uuid,
    pub server_allowed_ips: Vec<IpAddrMask>,
    pub address: String,
    pub protocol_version: Option<u32>,
    pub endpoint: Option<SocketAddr>,
    pub dns: Option<String>,
    pub transmitted_bytes: u64,
    pub received_bytes: u64,
    pub last_handshake: Option<SystemTime>,
}
