use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardClientData {
    pub name: String,
    // stored in server & client configs
    pub preshared_key: Option<String>,
    // stored in server config
    pub public_key: String,
    pub server_allowed_ips: Vec<String>,
    pub persistent_keep_alive: Option<u16>,
    // stored in client config
    pub private_key: String,
    pub address: String,
    pub client_allowed_ips: Vec<String>,
    pub dns: Option<String>,
}