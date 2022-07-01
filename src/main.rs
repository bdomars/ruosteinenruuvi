use std::{error::Error};

use ruuvi::scan_btle;
use tokio::{sync::broadcast};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = broadcast::channel(16);

    tokio::spawn(async move {
        let r = scan_btle(tx).await;
        if r.is_err() {
            println!("defuk? {}", r.unwrap_err());
        }
    });

    loop {
        let rm = rx.recv().await;
        println!("Got a ruuvimessage:");
        println!("{:#?}", rm);
    }
}
