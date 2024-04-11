use core::time;
use std::thread::sleep;

use ping_rs::error::PingError;

use ping_rs::device::{Common, Ping360, PingDevice};


#[tokio::main]
async fn main() -> Result<(), PingError> {
    // Create a new Ping360 device instance
    let mut common = Common::new();

    let mut ping360 = Ping360::default();

    // Set the device ID
    ping360.get_head();



    match ping360.get_firmware().await {
        Ok(version) => {
            println!("Firmware version: {:?}", version);
        }
        Err(err) => {
            eprintln!("Error getting firmware version: {:?}", err);
        }
    }

    Ok(())
}