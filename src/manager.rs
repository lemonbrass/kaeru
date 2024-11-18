use crate::util::{get_contents_of, overwrite_contents_of, run_command};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io;
use toml::{from_str, to_string_pretty};

#[derive(Debug, Deserialize, Serialize)]
pub struct Manager {
    installcmd: String,
    removecmd: String,
    synccmd: String,
    upgradecmd: String,
    pub items: HashSet<String>,

    #[serde(skip)]
    pub file: String,
}

impl Manager {
    pub fn new(filename: String) -> Self {
        let mut manager: Self = from_str(&get_contents_of(&filename).unwrap()).unwrap();
        manager.file = filename;
        manager
    }

    pub fn install(&mut self, packages: Vec<String>) -> io::Result<()> {
        let cmd = self.installcmd.replace(":#?", &packages.join(" "));
        self.items.extend(packages);
        run_command(&cmd)
    }

    pub fn sync(&self) -> io::Result<()> {
        run_command(&self.synccmd)
    }

    pub fn upgrade(&self) -> io::Result<()> {
        run_command(&self.upgradecmd)
    }

    pub fn remove(&mut self, packages: Vec<String>) -> io::Result<()> {
        let cmd = self.removecmd.replace(":#?", &packages.join(" "));
        for pack in packages {
            self.items.remove(&pack);
        }
        run_command(&cmd)
    }
    pub fn save(&self) {
        overwrite_contents_of(&self.file, &to_string_pretty(self).unwrap()).unwrap();
    }
}
