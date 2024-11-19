use std::collections::HashMap;

use crate::util::get_contents_of;
use serde::{Deserialize, Serialize};
use toml::{de::Error, from_str};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    manager: Option<ManagerConfig>,
    package: Option<PackageConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManagerConfig {
    call_order: Option<Vec<String>>,
    setup_cmds: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageConfig {
    oninstall: Option<HashMap<String, String>>,
}

impl Config {
    pub fn read(filename: String) -> Result<Self, Error> {
        from_str(&get_contents_of(&filename).unwrap())
    }
}
