#![allow(dead_code)]
use clap::Parser;

use crate::{
    cli::*, error::Error, globals::*, manager::Manager, package_pack::PackagePack, util::*,
};
use std::{collections::HashMap, fs};

pub struct App {
    managers: HashMap<String, Manager>,
    package_packs: HashMap<String, PackagePack>,
}

impl App {
    pub fn init() -> Self {
        let mut app = Self {
            managers: HashMap::new(),
            package_packs: HashMap::new(),
        };
        if !app.is_already_setup() && yesnoprompt(ASK_FOR_SETUP_MSG) {
            terminate_on_error(app.setup());
        }
        app.read_data();
        app.setup_cli();
        app
    }

    fn read_data(&mut self) {
        for manager in files_in_dir(&managers_dir(), ".toml").unwrap() {
            println!("Found manager: {}", manager);
            self.managers.insert(
                get_filename(&manager).unwrap().replace(".toml", ""),
                Manager::new(manager).unwrap(),
            );
        }

        for package_pack in files_in_dir(&&package_dir(), ".toml").unwrap() {
            println!("Found package pack: {}", package_pack);
            self.package_packs.insert(
                get_filename(&package_pack).unwrap().replace(".toml", ""),
                PackagePack::new(package_pack).unwrap(),
            );
        }
    }

    fn setup_cli(&self) {
        let cli = Cli::parse();
        if let Some(command) = cli.command {
            match command {
                Commands::Gen(gen) => self.handle_generation(gen),
                Commands::Install(install) => self.handle_install(install),
                Commands::Remove(remove) => self.handle_remove(remove),
                Commands::Sync(sync) => self.handle_sync(sync),
            }
        }
    }

    fn handle_generation(&self, gen: GenerationCommand) {}
    fn handle_install(&self, install: InstallPkg) {}
    fn handle_sync(&self, sync: SyncPkg) {}
    fn handle_remove(&self, remove: RemovePkg) {}

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
