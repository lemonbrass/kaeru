use crate::error::GEN_ERROR;
use crate::globals::ERR_INVALID_GENID;
use crate::manager::Manager;
use crate::util::*;
use crate::{error::Error, globals::ERR_NO_CHANGES_TO_COMMIT};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{BTreeMap, HashSet};

pub struct GenerationManager {
    gens: BTreeMap<usize, Generation>,
    latest_gen: usize,
}

#[derive(Serialize, Deserialize)]
struct Generation {
    snapshot: BTreeMap<String, ConfFile>,
    epoch: i64,
    message: String,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
struct ConfFile {
    content: String,
    pub path: String,
}

struct Diff {
    manager: Manager,
    install: Vec<String>,
    remove: Vec<String>,
}

impl Generation {
    pub fn read(file: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&get_contents_of(&file).unwrap())
    }

    fn default(message: Option<String>) -> Self {
        Self {
            snapshot: BTreeMap::new(),
            epoch: epoch_time_secs(),
            message: if message.is_none() {
                "".into()
            } else {
                message.unwrap()
            },
        }
    }

    pub fn create(message: String) -> Self {
        let mut gen = Self::default(Some(message));
        for file in files_in_dir(&managers_dir(), ".toml").unwrap() {
            let file_name = get_filename(&file)
                .unwrap()
                .strip_suffix(".toml")
                .unwrap()
                .to_string();
            let conf_file = ConfFile::new(&file);
            gen.snapshot.insert(file_name, conf_file);
        }
        gen
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn restore(&self) {
        remove_all_files_in_dir(&managers_dir()).unwrap();
        for (_, file) in self.snapshot.iter() {
            overwrite_contents_of(&file.path, &file.content).unwrap();
        }
    }
}
impl GenerationManager {
    pub fn read() -> Self {
        let genfiles = files_in_dir(&gen_dir(), ".json").unwrap();
        let mut manager = Self {
            gens: BTreeMap::new(),
            latest_gen: 0,
        };
        for gen in genfiles {
            let genname = get_filename(&gen).unwrap();
            let genid: usize = genname
                .strip_suffix(".json")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            manager.gens.insert(genid, Generation::read(gen).unwrap());
            manager.latest_gen += 1;
        }

        manager
    }

    pub fn commit(&mut self, message: String) -> Result<(), Error> {
        let next_gen = Generation::create(message);
        if let Some(curr_gen) = self.gens.get(&self.latest_gen) {
            if next_gen.snapshot == curr_gen.snapshot {
                return Err(Error::new(ERR_NO_CHANGES_TO_COMMIT, GEN_ERROR));
            }
        }
        self.latest_gen += 1;
        self.gens.insert(self.latest_gen, next_gen);
        self.renumber_gens();
        Ok(())
    }

    pub fn apply_changes(&mut self) {
        let next_gen = self.gens.get(&self.latest_gen).unwrap();
        if let Some(prev_gen) = self.gens.get(&(self.latest_gen - 1)) {
            let diffs = Diff::compare_gens(&prev_gen, &next_gen);
            for mut diff in diffs {
                if !diff.install.is_empty() {
                    diff.manager.install(diff.install).unwrap();
                }
                if !diff.remove.is_empty() {
                    diff.manager.remove(diff.remove).unwrap();
                }
                diff.manager.save();
            }
        }
    }

    pub fn save(&mut self) {
        self.renumber_gens();
        remove_all_files_in_dir(&gen_dir()).unwrap();
        for (genid, gen) in self.gens.iter() {
            let savename = format!("{}{}.json", &gen_dir(), genid);
            create_file_with_contents(&savename, &gen.as_json());
        }
    }

    pub fn rollback(&mut self, genid: usize) -> Result<(), Error> {
        if let Some(gen) = self.gens.get(&genid) {
            gen.restore();
            self.commit(gen.message.clone())?;
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
        self.renumber_gens();
    }

    pub fn list_gens(&self) {
        let max_msg_len = self
            .gens
            .values()
            .map(|gen| gen.message.len())
            .max()
            .unwrap_or(0);

        for (id, gen) in self.gens.iter() {
            println!(
                "{:2}: {:<width$} @ {}",
                id,
                gen.message,
                epoch_to_str(gen.epoch),
                width = max_msg_len + 5
            );
        }
    }

    pub fn renumber_gens(&mut self) {
        let mut sorted_gens: Vec<(usize, Generation)> =
            std::mem::take(&mut self.gens).into_iter().collect();
        sorted_gens.sort_by_key(|(_, gen)| gen.epoch);
        let mut new_gens = BTreeMap::new();
        for (new_id, (_, gen)) in sorted_gens.into_iter().enumerate() {
            new_gens.insert(new_id + 1, gen);
        }
        self.gens = new_gens;
        self.latest_gen = self.gens.len();
    }
}

impl Diff {
    pub fn compare_gens(gen1: &Generation, gen2: &Generation) -> Vec<Self> {
        let mut diffs = Vec::new();
        let mut managers = HashSet::new();
        managers.extend(gen1.snapshot.keys());
        managers.extend(gen2.snapshot.keys());

        for manager in managers {
            if let Some(gen1manager) = gen1.snapshot.get(&manager.to_string()) {
                if let Some(gen2manager) = gen2.snapshot.get(&manager.to_string()) {
                    let manager1: Manager = toml::from_str(&gen1manager.content).unwrap();
                    let mut manager2: Manager = toml::from_str(&gen2manager.content).unwrap();
                    manager2.file = gen1manager.path.clone();
                    let install = manager2
                        .items
                        .difference(&manager1.items)
                        .into_iter()
                        .map(|ptr| ptr.clone())
                        .collect();
                    let remove = manager1
                        .items
                        .difference(&manager2.items)
                        .into_iter()
                        .map(|ptr| ptr.clone())
                        .collect();
                    diffs.push(Diff {
                        install,
                        manager: manager2,
                        remove,
                    });
                }
            }
        }

        diffs
    }
}

impl ConfFile {
    pub fn new(file: &str) -> Self {
        Self {
            content: get_contents_of(file).unwrap(),
            path: file.to_string(),
        }
    }
}
