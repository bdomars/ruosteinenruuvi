#![allow(unused_variables)]
#![allow(dead_code)]

use std::{fs, vec::Vec};

use serde::{Deserialize, Serialize};
use ruuvi::scan_btle;
use tokio::{sync::broadcast};
use macaddr::MacAddr6;

#[derive(Serialize, Deserialize, Debug)]
struct Tag {
    name: String,
    #[serde(with = "serde_with::rust::display_fromstr")]
    address: MacAddr6,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    tags: Vec<Tag>,
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(16);

    tokio::spawn(async move {
        match scan_btle(tx).await {
            Ok(_) => println!("scan_btle completed with ok!"),
            Err(e) => println!("scan_btle completed with error: {}", e),
        };
    });

    let contents = fs::read_to_string("Tags.toml")
    .expect("Something went wrong reading the file");

    //let config: Config = toml::from_str(&contents).unwrap();
    let config: Config = toml::from_str(&contents).unwrap();
    println!("{:#?}", config);

    loop {
        let rm = rx.recv().await;
        println!("Got a ruuvimessage:");
        println!("{:#?}", rm);
    }
}
