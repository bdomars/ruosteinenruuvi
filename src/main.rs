#![allow(unused_variables)]
#![allow(dead_code)]
use tokio::sync::broadcast;

use ruuvi::load_config;

mod tag;

#[tokio::main]
async fn main() -> ruuvi::Result<()> {
    let config = load_config("Tags.toml")?;
    let tagger = tag::Tagger { config };

    let (tx, mut rx) = broadcast::channel(16);
    tokio::spawn(ruuvi::scan_btle(tx));
    loop {
        let rm = rx.recv().await?;
        let tr = tagger.tag(rm).await;
        println!("{:#?}", tr);
    }
}
