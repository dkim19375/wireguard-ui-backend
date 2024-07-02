use netdev::Interface;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_interface")]
    pub interface: String,
    #[serde(default = "default_network_interface")]
    pub network_interface: String,
    #[serde(default = "default_address")]
    pub address: String,
    #[serde(default = "default_wireguard_config_path")]
    pub wireguard_config_path: String,
}

impl AppConfig {
    pub fn get_network_interface_name(&self) -> String {
        if !self.network_interface.is_empty() {
            return self.network_interface.to_owned();
        }
        netdev::get_default_interface().unwrap().name
    }

    pub fn get_wireguard_network_interface(&self) -> Option<Interface> {
        netdev::get_interfaces()
            .iter()
            .find(|interface| interface.name == self.interface)
            .map(Interface::to_owned)
    }
}

fn default_interface() -> String {
    if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") {
        "wg0".into()
    } else {
        "utun3".into()
    }
}

fn default_network_interface() -> String {
    "".to_string()
}

fn default_address() -> String {
    "0.0.0.0:6252".to_string()
}

fn default_wireguard_config_path() -> String {
    "/etc/wireguard/wg0.conf".to_string()
}
