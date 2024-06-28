use crate::data::wireguard_client::WireGuardClientData;
use crate::data::wireguard_server::WireGuardServerData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardData {
    pub server: WireGuardServerData,
    pub clients: Vec<WireGuardClientData>,
}
