use std::{collections::HashMap, io, path::PathBuf};

use etcetera::{BaseStrategy, choose_base_strategy};
use serde::{Deserialize, Serialize};

pub type Name = String;

pub const DEFAULT_NAME: &str = "sheep";
pub const STORE_FILE_NAME: &str = "sheeve.msgpack";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Store {
    name_counters: HashMap<Name, u32>,
    default_name: Name,
    #[serde(skip)]
    file_path: PathBuf,
}

pub fn get_path_of_store_file() -> PathBuf {
    let strategy = choose_base_strategy().unwrap();
    strategy.data_dir().join(STORE_FILE_NAME)
}

impl Store {
    fn new() -> Self {
        Store {
            name_counters: HashMap::new(),
            default_name: DEFAULT_NAME.into(),
            file_path: get_path_of_store_file(),
        }
    }

    pub fn open_or_create() -> io::Result<Self> {
        match Self::load() {
            Ok(s) => Ok(s),
            Err(e) if matches!(e.kind(), std::io::ErrorKind::NotFound) => {
                let store = Store::new();
                Ok(store)
            }
            Err(e) => Err(e),
        }
    }

    pub fn load() -> io::Result<Self> {
        let path = get_path_of_store_file();
        let file = std::fs::File::options().read(true).open(&path)?;
        let rd = std::io::BufReader::new(file);
        let mut store: Self = rmp_serde::from_read(rd).expect("could not deserialize the store");
        store.file_path = path;
        Ok(store)
    }

    pub fn save(&self) -> io::Result<()> {
        std::fs::write(
            &self.file_path,
            rmp_serde::to_vec(&self).expect("could not serialize the store"),
        )
    }

    pub fn default_name(&self) -> &Name {
        &self.default_name
    }

    pub fn set_default_name(&mut self, default_name: Name) {
        self.default_name = default_name;
    }

    fn get_mut_entry(&mut self, key: &Name) -> &mut u32 {
        self.name_counters.entry(key.to_owned()).or_insert(0)
    }

    pub fn take_number_for_key(&mut self, key: &Name) -> u32 {
        // not using the entry api because that requires that we own Name, which could be a longer
        // string
        let counter = self.get_mut_entry(key);
        // need to change the data first, then return the value. We cant change the data after the
        // function returns
        *counter += 1;
        *counter - 1
    }

    pub fn set_counter(&mut self, key: &Name, number: u32) {
        if number > 0 {
            *self.get_mut_entry(key) = number;
        } else {
            self.name_counters.remove(key);
        }
    }

    pub fn name_counters(&self) -> &HashMap<Name, u32> {
        &self.name_counters
    }
}
