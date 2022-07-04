#![allow(unused_variables)]
#![allow(dead_code)]

use std::{fs, vec::Vec};

use serde::{Deserialize, Serialize};
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
async fn main() -> ruuvi::Result<()>{
    let (tx, mut rx) = broadcast::channel(16);

    tokio::spawn(ruuvi::scan_btle(tx));

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
