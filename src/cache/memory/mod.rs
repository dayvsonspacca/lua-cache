use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

pub struct MemoryStorage {
    ttl: Duration,
    data: HashMap<String, (String, SystemTime)>,
}

#[allow(dead_code)]
impl MemoryStorage {
    pub fn new(ttl: Duration) -> MemoryStorage {
        MemoryStorage {
            ttl,
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.data.insert(
            key.to_string(),
            (value.to_string(), SystemTime::now() + self.ttl),
        );
    }

    pub fn get(&mut self, key: &str) -> String {
        if let Some(cache) = self.data.get(key) {
            if SystemTime::now() < cache.1 {
                return cache.0.to_string();
            }
            self.invalidate(key);
        }
        String::new()
    }

    fn invalidate(&mut self, key: &str) {
        self.data.remove(key);
    }
}
