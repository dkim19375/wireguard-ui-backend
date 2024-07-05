use std::error::Error;
use std::fmt::{Display, Formatter};

use netdev::Interface;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_wireguard_interface")]
    pub wireguard_interface: String,
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

    pub fn get_wireguard_network_interface(&self) -> Result<Interface, ConfigurationError> {
        netdev::get_interfaces()
            .iter()
            .find(|interface| interface.name == self.wireguard_interface)
            .map(Interface::to_owned)
            .ok_or_else(|| ConfigurationError {
                message: format!("WireGuard interface {} not found", self.wireguard_interface),
            })
    }
}

#[derive(Debug)]
pub struct ConfigurationError {
    pub message: String,
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ConfigurationError {}

fn default_wireguard_interface() -> String {
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
