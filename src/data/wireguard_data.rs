use crate::data::wireguard_client::WireGuardClientData;
use crate::data::wireguard_server::WireGuardServerData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardData {
    pub server: WireGuardServerData,
    pub clients: Vec<WireGuardClientData>,
}

impl WireGuardData {
    pub fn get_server_config(&self) -> String {
        let mut result = self.server.get_interface_config();
        for client in &self.clients {
            result += &format!("\n\n{}", client.get_server_peer_config());
        }
        result
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
