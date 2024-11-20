use crate::diff::GenDiff;
use crate::error::Error;
use crate::error::GEN_ERROR;
use crate::gen::*;
use crate::globals::{ERR_INVALID_GENID, GENERATION_FILE_EXT};
use crate::manager::Manager;
use crate::util::*;
use std::collections::{BTreeMap, HashSet};

pub struct GenerationManager {
    gens: BTreeMap<usize, Generation>,
    latest_gen: usize,
}

impl GenerationManager {
    pub fn read() -> Self {
        let genfiles = files_in_dir(&gen_dir(), GENERATION_FILE_EXT).unwrap();
        let mut manager = Self {
            gens: BTreeMap::new(),
            latest_gen: 0,
        };
        for gen in genfiles {
            let genname = get_filename(&gen).unwrap();
            let genid: usize = genname
                .strip_suffix(GENERATION_FILE_EXT)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            manager.gens.insert(genid, Generation::read(gen).unwrap());
            manager.latest_gen += 1;
        }

        manager
    }

    pub fn commit(&mut self, message: String) -> Result<(), Error> {
        let next_gen: Generation;
        if let Some(curr_gen) = self.gens.get(&self.latest_gen) {
            next_gen = Generation::create(message, &curr_gen)?;
        } else {
            next_gen = Generation::genesis(message);
        }
        self.latest_gen += 1;
        self.gens.insert(self.latest_gen, next_gen);
        self.renumber_gens();
        Ok(())
    }

    pub fn apply_changes(&mut self, genid: Option<usize>) {
        let curr_gen = self.gens.get(&self.latest_gen).unwrap();
        if curr_gen.applied {
            return ();
        }
        let prev_gen: Generation = self
            .gens
            .get(
                &(if genid.is_none() {
                    self.latest_gen - 1
                } else {
                    genid.unwrap()
                }),
            )
            .cloned()
            .unwrap_or_else(|| Generation::default(None));
        let diffs = GenDiff::from_gens(&curr_gen, &prev_gen);
        for diff in diffs {
            println!("{}", diff.manager);
            let mut manager = Manager::new(diff.manager);
            if !diff.newly_installed.is_empty() {
                manager.install(diff.newly_installed).unwrap();
            }
            if !diff.removed.is_empty() {
                manager.remove(diff.removed).unwrap();
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
