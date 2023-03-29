#![allow(unused_variables)]
#![allow(dead_code)]
use tokio::sync::broadcast;

use ruuvi::load_config;

#[tokio::main]
async fn main() -> ruuvi::Result<()> {
    let c = load_config("Tags.toml")?;

    let (tx, mut rx) = broadcast::channel(16);
    tokio::spawn(ruuvi::scan_btle(tx));
    loop {
        let rm = rx.recv().await;
        println!("{:#?}", rm);
    }
}
