use std::collections::HashSet;

use crate::{gen::Generation, manager::Manager};

pub struct GenDiff {
    pub manager: String,
    pub newly_installed: Vec<String>,
    pub removed: Vec<String>,
}

impl GenDiff {
    pub fn from_gens(gen1: &Generation, gen2: &Generation) -> Vec<Self> {
        let mut diffs = Vec::new();
        let mut files = HashSet::new();
        for file in gen1.snapshot.keys() {
            files.insert(file);
        }
        for file in gen2.snapshot.keys() {
            files.insert(file);
        }

        for file in files {
            let gen1file = gen1.snapshot.get(file.as_str());
            let gen2file = gen2.snapshot.get(file.as_str());
            if gen2file.is_some() && gen1file.is_some() {
                let gen1manager: Manager = toml::from_str(&gen1file.unwrap().content).unwrap();
                let gen2manager: Manager = toml::from_str(&gen2file.unwrap().content).unwrap();
                let installed = gen1manager
                    .items
                    .difference(&gen2manager.items)
                    .map(|ptr| ptr.clone());
                let removed = gen2manager
                    .items
                    .difference(&gen1manager.items)
                    .map(|ptr| ptr.clone());
                diffs.push(GenDiff {
                    newly_installed: installed.collect(),
                    manager: file.clone(),
                    removed: removed.collect(),
                });
            } else if gen2file.is_none() && gen1file.is_some() {
                let manager: Manager = toml::from_str(&gen1file.unwrap().content).unwrap();
                diffs.push(GenDiff {
                    newly_installed: Vec::new(),
                    removed: manager.items.into_iter().collect(),
                    manager: file.clone(),
                });
            } else if gen2file.is_some() && gen1file.is_none() {
                let manager: Manager = toml::from_str(&gen2file.unwrap().content).unwrap();
                diffs.push(GenDiff {
                    newly_installed: manager.items.into_iter().collect(),
                    removed: Vec::new(),
                    manager: file.clone(),
                });
            }
        }
        diffs
    }
}
