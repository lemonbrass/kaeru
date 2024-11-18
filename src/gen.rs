use crate::error::GEN_ERROR;
use crate::globals::ERR_INVALID_GENID;
use crate::util::*;
use crate::{error::Error, globals::ERR_NO_CHANGES_TO_COMMIT};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, HashSet};

pub struct GenerationManager {
    gens: HashMap<usize, Generation>,
    latest_gen: usize,
}

#[derive(Serialize, Deserialize)]
struct Generation {
    snapshot: HashMap<String, ConfFile>,
    epoch: i64,
    message: String,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
struct ConfFile {
    content: String,
    pub path: String,
    is_new: bool,
}

impl Generation {
    pub fn read(file: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&get_contents_of(&file).unwrap())
    }

    fn default(message: Option<String>) -> Self {
        Self {
            snapshot: HashMap::new(),
            epoch: epoch_time_secs(),
            message: if message.is_none() {
                "".into()
            } else {
                message.unwrap()
            },
        }
    }

    pub fn create(prev_gen: &Self, message: String) -> Self {
        let mut gen = Self::default(Some(message));
        for file in files_in_dirs(all_backup_files(), ".toml").unwrap() {
            let file_name = get_filename(&file)
                .unwrap()
                .strip_suffix(".toml")
                .unwrap()
                .to_string();
            let is_newly_created = prev_gen.snapshot.get(&file_name).is_none();
            let conf_file = ConfFile::new(&file, is_newly_created);
            gen.snapshot.insert(file_name, conf_file);
        }
        gen
    }
    // First "root" Generation
    pub fn genesis() -> Self {
        let mut genesis = Self::default(Some("GENESIS".into()));

        for file in files_in_dirs(all_backup_files(), ".toml").unwrap() {
            let file_name = get_filename(&file)
                .unwrap()
                .strip_suffix(".toml")
                .unwrap()
                .to_string();
            let conf_file = ConfFile::new(&file, true);
            genesis.snapshot.insert(file_name, conf_file);
        }

        genesis
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn restore(&self) {
        remove_all_files_in_dirs(all_backup_files()).unwrap();
        for (_, file) in self.snapshot.iter() {
            println!("RESTORING: {}", &file.path);
            overwrite_contents_of(&file.path, &file.content).unwrap();
        }
    }
}
impl GenerationManager {
    pub fn read() -> Self {
        let genfiles = files_in_dir(&gen_dir(), ".json").unwrap();
        let mut manager = Self {
            gens: HashMap::new(),
            latest_gen: 0,
        };
        for gen in genfiles {
            let genname = get_filename(&gen).unwrap();
            let genid: usize = genname
                .strip_suffix(".json")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            println!("LOADING: {} {}", genname, genid);
            manager.gens.insert(genid, Generation::read(gen).unwrap());
            manager.latest_gen += 1;
        }

        manager
    }

    pub fn commit(&mut self, message: String) -> Result<(), Error> {
        let next_gen;
        if let Some(curr_gen) = self.gens.get(&self.latest_gen) {
            next_gen = Generation::create(curr_gen, message);
            if next_gen.snapshot == curr_gen.snapshot {
                return Err(Error::new(ERR_NO_CHANGES_TO_COMMIT, GEN_ERROR));
            }
        } else {
            next_gen = Generation::genesis();
        }
        self.latest_gen += 1;
        self.gens.insert(self.latest_gen, next_gen);
        Ok(())
    }

    pub fn save(&self) {
        remove_all_files_in_dir(&gen_dir()).unwrap();
        for (genid, gen) in self.gens.iter() {
            let savename = format!("{}{}.json", &gen_dir(), genid);
            println!("SAVING: {}", savename);
            create_file_with_contents(&savename, &gen.as_json());
        }
    }

    pub fn rollback(&mut self, genid: usize) -> Result<(), Error> {
        if let Some(gen) = self.gens.get(&genid) {
            gen.restore();
            self.commit(format!("ROLLBACK: {}", gen.message))?;
            Ok(())
        } else {
            Err(Error::new(ERR_INVALID_GENID, GEN_ERROR))
        }
    }

    pub fn remove(&mut self, genid: usize) -> Result<(), Error> {
        if let None = self.gens.remove(&genid) {
            Err(Error::new(ERR_INVALID_GENID, GEN_ERROR))
        } else {
            Ok(())
        }
    }

    pub fn remove_duplicates(&mut self) {
        let mut seem = HashSet::new();
        self.gens
            .retain(|_, gen| seem.insert(serde_json::to_string(&gen.snapshot).unwrap()));
    }

    pub fn list_gens(&self) {
        for (id, gen) in self.gens.iter() {
            println!("{}: {} @ {}", id, gen.message, epoch_to_str(gen.epoch));
        }
    }

    pub fn renumber_gens(&mut self) {}
}

impl ConfFile {
    pub fn new(file: &str, is_new: bool) -> Self {
        Self {
            content: get_contents_of(file).unwrap(),
            path: file.to_string(),
            is_new,
        }
    }
}
