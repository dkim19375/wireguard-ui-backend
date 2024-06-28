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

impl WireGuardClientData {
    pub fn get_server_config(&self) -> String {
        let mut result = format!("# Name: {}", self.name);
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

    pub fn get_client_config(&self, server_public_key: &String, server_endpoint: &String) -> String {
        let mut result = format!("# Name: {}", self.name);
        result += "\n[Interface]";
        result += &format!("\nPrivateKey = {}", self.private_key);
        result += &format!("\nAddress = {}", self.address);
        if let Some(dns) = &self.dns {
            result += &format!("\nDNS = {dns}");
        }
        result += "\n\n[Peer]";
        result += &format!("\nPublicKey = {server_public_key}");
        if let Some(preshared_key) = &self.preshared_key {
            result += &format!("\nPresharedKey = {preshared_key}");
        }
        result += &format!("\nAllowedIPs = {}", self.client_allowed_ips.join(","));
        result += &format!("\nEndpoint = {server_endpoint}");
        result
    }
}
