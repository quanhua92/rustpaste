use std::{collections::HashMap, sync::Arc};

use async_graphql::futures_util::lock::Mutex;

use crate::Paste;

pub struct PasteStorage {
    data: HashMap<String, Paste>,
}

pub type Storage = Arc<Mutex<PasteStorage>>;

impl Default for PasteStorage {
    fn default() -> Self {
        PasteStorage {
            data: HashMap::new(),
        }
    }
}

impl PasteStorage {
    pub fn insert(&mut self, key: &str, value: &Paste) -> Option<Paste> {
        self.data.insert(key.to_string(), value.clone())
    }

    pub fn remove(&mut self, key: &str) -> Option<Paste> {
        self.data.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<Paste> {
        self.data.get(key).cloned()
    }

    pub fn get_all(&self) -> Vec<Paste> {
        self.data.values().cloned().collect()
    }
}
