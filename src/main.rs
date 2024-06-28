use std::error::Error;
use std::thread;

mod server;
mod data;
mod wireguard;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    server::start_server().await;
    
    // add something else later?
    
    loop {
        thread::park();
    }
}