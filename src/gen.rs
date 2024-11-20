use crate::error::GEN_ERROR;
use crate::globals::ERR_NO_CHANGES_TO_COMMIT;
use crate::util::*;
use crate::{error::Error, globals::MANAGER_FILE_EXT};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Generation {
    pub snapshot: HashMap<String, ConfFile>,
    pub epoch: i64,
    pub message: String,
    pub applied: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConfFile {
    pub content: String,
    pub path: String,
    pub epoch: i64, // time modified/commited
}

impl Generation {
    pub fn read(file: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&get_contents_of(&file).unwrap())
    }

    pub fn default(message: Option<String>) -> Self {
        Self {
            snapshot: HashMap::new(),
            epoch: epoch_time_secs(),
            message: if message.is_none() {
                "".into()
            } else {
                message.unwrap()
            },
            applied: false,
        }
    }

    pub fn create(message: String, prev_gen: &Generation) -> Result<Self, Error> {
        let mut gen = Self::default(Some(message));
        for file in files_in_dir(&managers_dir(), MANAGER_FILE_EXT).unwrap() {
            let contents = get_contents_of(&file).unwrap();
            if let Some(prev_content) = prev_gen.snapshot.get(&file) {
                if contents == prev_content.content {
                    gen.snapshot.insert(file, prev_content.clone());
                }
            } else {
                let conffile = ConfFile::from_contents(&file, contents);
                gen.snapshot.insert(file, conffile);
            }
        }

        if gen.snapshot == prev_gen.snapshot {
            return Err(Error::new(ERR_NO_CHANGES_TO_COMMIT, GEN_ERROR));
        }
        Ok(gen)
    }

    pub fn genesis(message: String) -> Self {
        let mut gen = Self::default(Some(message));

        for file in files_in_dir(&managers_dir(), MANAGER_FILE_EXT).unwrap() {
            let conffile = ConfFile::new(&file, epoch_time_secs());
            gen.snapshot.insert(file, conffile);
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

impl ConfFile {
    pub fn new(file: &str, epoch: i64) -> Self {
        Self {
            content: get_contents_of(file).unwrap(),
            path: file.to_string(),
            epoch,
        }
    }
    pub fn from_contents(file: &str, content: String) -> Self {
        Self {
            content,
            path: file.to_string(),
            epoch: epoch_time_secs(),
        }
    }
}
