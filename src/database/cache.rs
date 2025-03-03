use mongodb::bson::Document;
use mongodb::{Collection, Database};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct Cache {
    pub collection: HashMap<String, Collection<Document>>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            collection: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, collection: Collection<Document>) {
        self.collection.insert(name, collection);
    }

    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }

    pub fn get_collection(&self, name: String) -> Option<&Collection<Document>> {
        self.collection.get(&name)
    }
}

pub static GLOBAL_CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| Mutex::new(Cache::new()));

pub static GLOBAL_DATABASE: Lazy<Mutex<Option<Database>>> = Lazy::new(|| Mutex::new(None));

pub const CHATCOLLECTION: &str = "Chat";
pub const USERCOLLECTION: &str = "UserAccounts";
