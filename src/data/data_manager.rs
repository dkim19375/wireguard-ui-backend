use crate::data::config::AppConfig;
use crate::data::wireguard_data::WireGuardData;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};

pub fn read_config_file() -> Result<AppConfig, Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true) // required to set truncate to false
        .truncate(false)
        .create(true)
        .open("config.yaml")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let json: AppConfig = serde_yaml::from_str(&data)?;
    Ok(json)
}

pub fn save_config_file(config: &AppConfig) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("config.yaml")?;
    let yaml = serde_yaml::to_string(config)?;
    file.write_all(yaml.as_bytes())?;
    Ok(())
}

pub fn read_json_file() -> Result<WireGuardData, Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true) // required to set truncate to false
        .truncate(false)
        .create(true)
        .open("data.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    if data.trim().is_empty() {
        data = "{}".into();
    }

    let json: WireGuardData = serde_json::from_str(&data)?;
    Ok(json)
}

pub fn save_json_file(data: &WireGuardData) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("data.json")?;
    let json = serde_json::to_string_pretty(data)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn save_wireguard_config(data: &WireGuardData, file_path: &String) -> Result<(), io::Error> {
    let mut file = File::create(file_path)?;
    let config = data.get_server_config();
    file.write_all(config.unwrap_or_default().as_bytes())?;
    Ok(())
}
