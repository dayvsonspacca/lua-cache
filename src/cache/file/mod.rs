use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    time::{Duration, SystemTime},
};

use rand::Rng;

pub struct CacheStorate {
    key: String,
    ttl: Duration,
    data: HashMap<String, SystemTime>,
}

impl CacheStorate {
    pub fn new(ttl: Duration) -> CacheStorate {
        let key = rand::thread_rng().gen::<u64>().to_string();

        match fs::create_dir_all(format!("lua-cache/{}", key)) {
            Ok(_) => CacheStorate {
                key,
                ttl,
                data: HashMap::new(),
            },
            Err(e) => panic!("{}", e),
        }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.data
            .insert(key.to_string(), SystemTime::now() + self.ttl);

        let mut file = match File::create(format!("lua-cache/{}/{}", self.key, key)) {
            Ok(file) => file,
            Err(e) => panic!("Failed to create cache file: {}", e),
        };

        match file.write_all(value.as_bytes()) {
            Ok(_) => {}
            Err(e) => panic!("Failed to write in cache file: {}", e),
        }
    }
}

impl Drop for CacheStorate {
    fn drop(&mut self) {
        match fs::remove_dir_all(format!("lua-cache/{}", self.key)) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to remove CacheStorage with id: {}", self.key);
                println!("Error: {}", e);
            }
        }
    }
}
