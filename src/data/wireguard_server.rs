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

impl WireGuardServerData {
    pub fn get_interface_config(&self) -> String {
        let mut result = String::from("[Interface]");
        result += &format!("\nAddress = {}", self.address.join(","));
        result += &format!("\nListenPort = {}", self.listen_port);
        result += &format!("\nPrivateKey = {}", self.private_key);
        if !self.dns.is_empty() {
            result += &format!("\nDNS = {}", self.dns.join(","));
        }
        let mut second_part = String::new();
        if let Some(table) = &self.table {
            second_part += &format!("\nTable = {}", table);
        }
        if let Some(mtu) = self.mtu {
            second_part += &format!("\nMTU = {}", mtu);
        }
        if let Some(pre_up) = &self.pre_up {
            second_part += &format!("\nPreUp = {}", pre_up);
        }
        if let Some(post_up) = &self.post_up {
            second_part += &format!("\nPostUp = {}", post_up);
        }
        if let Some(pre_down) = &self.pre_down {
            second_part += &format!("\nPreDown = {}", pre_down);
        }
        if let Some(post_down) = &self.post_down {
            second_part += &format!("\nPostDown = {}", post_down);
        }
        if !second_part.is_empty() {
            result += &format!("\n{second_part}");
        }
        result
    }
}
