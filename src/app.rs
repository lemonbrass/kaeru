#![allow(dead_code)]
use clap::Parser;

use crate::{
    cli::*, error::Error, gen::GenerationManager, globals::*, manager::Manager,
    package_pack::PackagePack, util::*,
};
use std::{collections::HashMap, fs};

pub struct App {
    managers: HashMap<String, Manager>,
    package_packs: HashMap<String, PackagePack>,
    generation_manager: GenerationManager,
}

impl App {
    pub fn init() -> Self {
        if !Self::is_already_setup() && yesnoprompt(ASK_FOR_SETUP_MSG) {
            terminate_on_error(Self::setup());
        }
        let mut app = Self {
            managers: HashMap::new(),
            package_packs: HashMap::new(),
            generation_manager: GenerationManager::read(),
        };
        app.read_data();
        app.setup_cli();
        app
    }

    fn read_data(&mut self) {
        for manager in files_in_dir(&managers_dir(), ".toml").unwrap() {
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

    fn setup_cli(&mut self) {
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

    fn handle_generation(&mut self, gen: GenerationCommand) {
        match gen {
            GenerationCommand::Commit(messagedata) => {
                terminate_on_error(self.generation_manager.commit(messagedata.genmsg));
                self.generation_manager.save();
            }
            GenerationCommand::Rollback(geninfo) => {
                terminate_on_error(self.generation_manager.rollback(geninfo.genid));
                self.generation_manager.save();
            }
            GenerationCommand::Remove(geninfo) => {
                terminate_on_error(self.generation_manager.remove(geninfo.genid));
                self.generation_manager.save();
            }
            GenerationCommand::RemoveDuplicates => {
                self.generation_manager.remove_duplicates();
                self.generation_manager.save();
            }
            GenerationCommand::List => {
                self.generation_manager.list_gens();
            }
        }
    }
    fn handle_install(&self, _install: PkgData) {}
    fn handle_sync(&self, _sync: SyncPkg) {}
    fn handle_remove(&self, _remove: PkgData) {}

    fn is_already_setup() -> bool {
        if let Ok(res) = fs::exists(conf_file()) {
            return res;
        } else {
            return false;
        }
    }

    fn check_corruption(&self) -> Result<(), Error> {
        todo!();
    }

    fn setup() -> Result<(), Error> {
        mkdir_if_not_exists(&conf_dir()).unwrap();
        mkdir_if_not_exists(&managers_dir()).unwrap();
        mkdir_if_not_exists(&package_dir()).unwrap();
        mkdir_if_not_exists(&gen_dir()).unwrap();
        create_file_with_contents(&conf_file(), DEFAULT_CONFIG);
        println!("{}", SETUP_COMPLETE);
        Ok(())
    }
}
