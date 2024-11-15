use crate::util::get_contents_of;
use serde::{Deserialize, Serialize};
use toml::{de::Error, from_str};

#[derive(Deserialize, Serialize)]
pub struct PackagePack {
    name: String,
    items: Vec<String>,
    manager: String,
}

impl PackagePack {
    pub fn new(filename: String) -> Result<Self, Error> {
        from_str(&get_contents_of(&filename).unwrap())
    }
}
