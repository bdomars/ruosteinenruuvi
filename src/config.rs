use std::{fs, vec::Vec};

use macaddr::MacAddr6;
use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub address: MacAddr6,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub tags: Vec<Tag>,
}

pub fn load_config(filename: &str) -> Result<Config> {
    let contents = fs::read_to_string(filename)?;
    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}
