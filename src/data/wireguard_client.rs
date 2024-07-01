use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardClientData {
    pub name: String,
    #[serde(serialize_with = "uuid::serde::simple::serialize")]
    pub uuid: Uuid,
    pub enabled: bool,
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
    pub dns: Vec<String>,
}

impl WireGuardClientData {
    pub fn get_server_peer_config(&self) -> String {
        let mut result = format!("# Name: {}", self.name);
        result += &format!("\n# UUID: {}", self.uuid);
        let prefix = if self.enabled { "\n" } else { "\n# " };
        result += &format!("{}[Peer]", prefix);
        result += &format!("{}PublicKey = {}", prefix, self.public_key);
        if let Some(preshared_key) = &self.preshared_key {
            result += &format!("{}PresharedKey = {preshared_key}", prefix);
        }
        result += &format!(
            "{}AllowedIPs = {}",
            prefix,
            self.server_allowed_ips.join(",")
        );
        if let Some(persistent_keep_alive) = self.persistent_keep_alive {
            result += &format!("{}PersistentKeepAlive = {persistent_keep_alive}", prefix);
        }
        result
    }

    pub fn get_client_config(
        &self,
        server_public_key: &String,
        server_endpoint: &String,
    ) -> String {
        let mut result = format!("# Name: {}", self.name);
        result += "[Interface]";
        result += &format!("PrivateKey = {}", self.private_key);
        result += &format!("Address = {}", self.address);
        if !self.dns.is_empty() {
            result += &format!("DNS = {}", self.dns.join(","));
        }
        result += "[Peer]";
        result += &format!("PublicKey = {server_public_key}");
        if let Some(preshared_key) = &self.preshared_key {
            result += &format!("PresharedKey = {preshared_key}");
        }
        result += &format!("AllowedIPs = {}", self.client_allowed_ips.join(","));
        result += &format!("Endpoint = {server_endpoint}");
        result
    }
}
