use crate::data::wireguard_data::WireGuardData;
use defguard_wireguard_rs::WGApi;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

mod data;
mod server;
mod wireguard;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Reading data file");
    let data = data::data_manager::read_json_file()?;
    data::data_manager::save_json_file(&data)?;

    println!("Preparing WireGuard");
    let if_name: String = if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") {
        "wg0".into()
    } else {
        "utun3".into()
    };
    let wg_api = WGApi::new(if_name.clone(), false)?;

    let app_values = Arc::new(Mutex::new(WireGuardAppValues {
        wg_api,
        wireguard_data: data,
    }));

    println!("Starting server");
    server::start_server(app_values.clone()).await;

    // add something else later?

    loop {
        thread::park();
    }
}

pub struct WireGuardAppValues {
    pub wg_api: WGApi,
    pub wireguard_data: WireGuardData,
}
