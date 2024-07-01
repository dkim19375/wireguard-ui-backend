use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::wireguard_client::WireGuardClientData;
use crate::data::wireguard_server::WireGuardServerData;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WireGuardData {
    pub server: Option<WireGuardServerData>,
    #[serde(default = "Vec::new")]
    pub clients: Vec<WireGuardClientData>,
}

impl WireGuardData {
    pub fn get_server_config(&self) -> Option<String> {
        let server = match self.server {
            Some(ref server) => server,
            None => return None,
        };
        let mut result = String::new();
        result += &String::from("# Generated from WireGuard UI\n");
        result += &String::from("# Do not edit manually!\n\n");

        result += &server.get_interface_config();
        for client in &self.clients {
            result += &format!("\n\n{}", client.get_server_peer_config());
        }
        Some(result + "\n")
    }

    pub fn get_client_config(&self, uuid: &Uuid) -> Option<WireGuardClientData> {
        for client in &self.clients {
            if &client.uuid == uuid {
                return Some(client.clone());
            }
        }
        None
    }
}
