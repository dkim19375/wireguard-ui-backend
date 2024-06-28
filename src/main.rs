use std::error::Error;
use std::thread;

mod data;
mod server;
mod wireguard;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Reading data file");
    let data = data::data_manager::read_json_file()?;
    data::data_manager::save_json_file(&data)?;

    println!("Starting server");
    server::start_server().await;

    // add something else later?

    loop {
        thread::park();
    }
}
