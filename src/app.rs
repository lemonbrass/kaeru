#![allow(dead_code)]
use crate::util::*;
use crate::{cli::*, error::Error, genman::GenerationManager, globals::*, manager::Manager};
use clap::Parser;
use std::{collections::HashMap, fs};

pub struct App {
    managers: HashMap<String, Manager>,
    generation_manager: GenerationManager,
}

impl App {
    pub fn init() -> Self {
        if !Self::is_already_setup() && yesnoprompt(ASK_FOR_SETUP_MSG) {
            terminate_on_error(Self::setup());
        }
        let mut app = Self {
            managers: HashMap::new(),
            generation_manager: GenerationManager::read(),
        };
        app.read_data();
        app.setup_cli();
        app.generation_manager.save();
        for (_, manager) in app.managers.iter() {
            manager.save();
        }
        app
    }

    fn read_data(&mut self) {
        for manager in files_in_dir(&managers_dir(), ".toml").unwrap() {
            self.managers.insert(
                get_filename(&manager).unwrap().replace(".toml", ""),
                Manager::new(manager),
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
                Commands::Upgrade(upgrade) => self.handle_upgrade(upgrade),
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
            GenerationCommand::Apply(gendata) => {
                self.generation_manager
                    .apply_changes(gendata.genid.map(|gendata| gendata));
            }
            GenerationCommand::Diff(diffdata) => {
                println!(
                    "Diff between Gen {} and Gen {}",
                    diffdata.genid1, diffdata.genid2
                );
            }
        }
    }
    fn handle_install(&mut self, install: PkgData) {
        self.managers
            .get_mut(&install.manager)
            .expect("The specified manager not found.")
            .install(install.pkg_names)
            .unwrap();
    }
    fn handle_sync(&self, sync: SyncPkg) {
        self.managers.get(&sync.manager).unwrap().sync().unwrap();
    }
    fn handle_upgrade(&self, upgrade: SyncPkg) {
        self.managers
            .get(&upgrade.manager)
            .unwrap()
            .upgrade()
            .unwrap();
    }
    fn handle_remove(&mut self, remove: PkgData) {
        self.managers
            .get_mut(&remove.manager)
            .unwrap()
            .install(remove.pkg_names)
            .unwrap();
    }

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
        mkdir_if_not_exists(&gen_dir()).unwrap();
        create_file_with_contents(&conf_file(), DEFAULT_CONFIG);
        println!("{}", SETUP_COMPLETE);
        Ok(())
    }
}
