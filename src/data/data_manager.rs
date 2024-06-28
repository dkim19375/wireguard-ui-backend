use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use crate::data::wireguard_data::WireGuardData;

pub fn read_json_file() -> Result<WireGuardData, Box<dyn Error>> {
    let mut file = File::create_new("data.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let json: WireGuardData = serde_json::from_str(&data)?;
    Ok(json)
}

pub fn save_json_file(data: &WireGuardData) -> Result<(), Box<dyn Error>> {
    let mut file = File::create_new("data.json")?;
    let json = serde_json::to_string_pretty(data)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}