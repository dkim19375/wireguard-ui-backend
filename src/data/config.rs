use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_interface")]
    pub interface: String,
    #[serde(default = "default_address")]
    pub address: String,
    #[serde(default = "default_wireguard_config_path")]
    pub wireguard_config_path: String,
}

fn default_interface() -> String {
    if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") {
        "wg0".into()
    } else {
        "utun3".into()
    }
}

fn default_address() -> String {
    "0.0.0.0:6252".to_string()
}

fn default_wireguard_config_path() -> String {
    "/etc/wireguard/wg0.conf".to_string()
}
