#![allow(dead_code)]
use clap::Parser;

use crate::{
    cli::Cli, error::Error, globals::*, manager::Manager, package_pack::PackagePack, util::*,
};
use std::{collections::HashMap, fs};

pub struct App {
    managers: HashMap<String, Manager>,
    package_packs: HashMap<String, PackagePack>,
}

impl App {
    pub fn init() -> Self {
        let app = Self {
            managers: HashMap::new(),
            package_packs: HashMap::new(),
        };
        if !app.is_already_setup() && yesnoprompt(ASK_FOR_SETUP_MSG) {
            terminate_on_error(app.setup());
        }
        app.setup_cli();
        app
    }

    fn setup_cli(&self) {
        Cli::parse();
    }

    fn is_already_setup(&self) -> bool {
        if let Ok(res) = fs::exists(conf_file()) {
            return res;
        } else {
            return false;
        }
    }

    fn check_corruption(&self) -> Result<(), Error> {
        todo!();
    }

    fn setup(&self) -> Result<(), Error> {
        mkdir_if_not_exists(&conf_dir()).unwrap();
        mkdir_if_not_exists(&managers_dir()).unwrap();
        mkdir_if_not_exists(&package_dir()).unwrap();
        mkdir_if_not_exists(&gen_dir()).unwrap();
        create_file_with_contents(&conf_file(), DEFAULT_CONFIG);
        println!("{}", SETUP_COMPLETE);
        Ok(())
    }
}
