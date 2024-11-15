use crate::util::get_contents_of;
use serde::{Deserialize, Serialize};
use toml::{de::Error, from_str};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    manager: ManagerConfig,
    package: PackageConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManagerConfig {
    call_order: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageConfig {
    package_order: Vec<String>,
}

impl Config {
    pub fn read(filename: String) -> Result<Self, Error> {
        from_str(&get_contents_of(&filename).unwrap())
    }
}
