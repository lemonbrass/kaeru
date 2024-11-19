use crate::manager::Manager;
use crate::util::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{BTreeMap, HashSet};

#[derive(Serialize, Deserialize)]
pub struct Generation {
    pub snapshot: BTreeMap<String, ConfFile>,
    pub epoch: i64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct ConfFile {
    content: String,
    pub path: String,
}

pub struct Diff {
    pub manager: Manager,
    pub install: Vec<String>,
    pub remove: Vec<String>,
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
