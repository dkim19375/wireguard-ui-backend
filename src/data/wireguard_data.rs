use serde::{Deserialize, Serialize};
use crate::data::wireguard_client::WireGuardClientData;
use crate::data::wireguard_server::WireGuardServerData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardData {
    pub server: WireGuardServerData,
    pub clients: Vec<WireGuardClientData>,
}