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
    pub dns: Option<String>,
}

impl WireGuardClientData {
    pub fn get_server_peer_config(&self) -> String {
        let mut result = format!("# Name: {}", self.name);
        result += &format!("\n#UUID: {}", self.uuid);
        result += "\n[Peer]";
        result += &format!("\nPublicKey = {}", self.public_key);
        if let Some(preshared_key) = &self.preshared_key {
            result += &format!("\nPresharedKey = {preshared_key}");
        }
        result += &format!("\nAllowedIPs = {}", self.server_allowed_ips.join(","));
        if let Some(persistent_keep_alive) = self.persistent_keep_alive {
            result += &format!("\nPersistentKeepAlive = {persistent_keep_alive}");
        }
        result
    }

    pub fn get_client_config(
        &self,
        server_public_key: &String,
        server_endpoint: &String,
    ) -> String {
        let mut result = format!("# Name: {}", self.name);
        let prefix = if self.enabled { "\n" } else { "\n#" };
        result += &format!("{}[Interface]", prefix);
        result += &format!("{}PrivateKey = {}", prefix, self.private_key);
        result += &format!("{}Address = {}", prefix, self.address);
        if let Some(dns) = &self.dns {
            result += &format!("{}DNS = {dns}", prefix);
        }
        result += &format!("{}{}[Peer]", prefix, prefix);
        result += &format!("{}PublicKey = {server_public_key}", prefix);
        if let Some(preshared_key) = &self.preshared_key {
            result += &format!("{}PresharedKey = {preshared_key}", prefix);
        }
        result += &format!(
            "{}AllowedIPs = {}",
            prefix,
            self.client_allowed_ips.join(",")
        );
        result += &format!("{}Endpoint = {server_endpoint}", prefix);
        result
    }
}
