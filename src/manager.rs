use crate::util::{get_contents_of, run_command};
use serde::Deserialize;
use std::io;
use toml::{de::Error, from_str};

#[derive(Debug, Deserialize)]
pub struct Manager {
    installcmd: String,
    removecmd: String,
    synccmd: String,
    upgradecmd: String,
}

impl Manager {
    pub fn new(filename: String) -> Result<Self, Error> {
        from_str(&get_contents_of(&filename).unwrap())
    }

    pub fn install(&self, packages: String) -> io::Result<()> {
        let cmd = self.installcmd.replace(":#?", &packages);
        run_command(&cmd)
    }

    pub fn sync(&self, packages: String) -> io::Result<()> {
        let cmd = self.synccmd.replace(":#?", &packages);
        run_command(&cmd)
    }

    pub fn upgrade(&self, packages: String) -> io::Result<()> {
        let cmd = self.upgradecmd.replace(":#?", &packages);
        run_command(&cmd)
    }

    pub fn remove(&self, packages: String) -> io::Result<()> {
        let cmd = self.removecmd.replace(":#?", &packages);
        run_command(&cmd)
    }
}
