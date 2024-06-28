use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardServerData {
    pub endpoint: String,
    pub address: String,
    pub listen_port: u16,
    pub private_key: String,
    pub public_key: String,
    pub post_up: String,
    pub post_down: String,
    pub table: String
}