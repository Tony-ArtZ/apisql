use serde_json::Value as Json;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct CacheEntry {
    pub value: Json,
    pub status_code: u16,
    pub timestamp: Instant,
    pub ttl: Duration,
}

#[derive(Default)]
pub struct Cache {
    store: HashMap<String, CacheEntry>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&CacheEntry> {
        self.store.get(key).and_then(|entry| {
            if entry.timestamp.elapsed() < entry.ttl {
                Some(entry)
            } else {
                None
            }
        })
    }

    pub fn insert(&mut self, key: String, entry: CacheEntry) {
        self.store.insert(key, entry);
    }
}
