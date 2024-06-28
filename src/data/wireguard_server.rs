use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardServerData {
    pub endpoint: String,
    pub address: Vec<String>,
    pub dns: Vec<String>,
    pub listen_port: u16,
    pub private_key: String,
    pub public_key: String,
    pub pre_up: Option<String>,
    pub post_up: Option<String>,
    pub pre_down: Option<String>,
    pub post_down: Option<String>,
    pub table: Option<String>,
    pub mtu: Option<u16>,
}
