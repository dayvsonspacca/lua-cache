use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    time::{Duration, SystemTime},
};

use rand::Rng;

pub struct FileStorage {
    key: String,
    ttl: Duration,
    data: HashMap<String, SystemTime>,
}

#[allow(dead_code)]
impl FileStorage {
    pub fn new(ttl: Duration) -> FileStorage {
        let key = rand::thread_rng().gen::<u64>().to_string();

        match fs::create_dir_all(format!("lua-cache/{}", key)) {
            Ok(_) => FileStorage {
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

    pub fn get(&mut self, key: &str) -> String {
        if let Some(expiration_time) = self.data.get(key) {
            if SystemTime::now() < *expiration_time {
                if let Ok(mut file) = File::open(format!("lua-cache/{}/{}", self.key, key)) {
                    let mut content = String::new();
                    if let Ok(_) = file.read_to_string(&mut content) {
                        return content;
                    } else {
                        println!("Failed to read content from cache with key: {}", key);
                    }
                } else {
                    println!("Failed to locate cache with key: {}", key);
                }
            }
            self.invalidate(key);
        }
        String::new()
    }

    fn invalidate(&mut self, key: &str) {
        self.data.remove(key);

        match fs::remove_file(format!("lua-cache/{}/{}", self.key, key)) {
            Ok(_) => {}
            Err(e) => eprintln!("Error invaliting cache: {}", e),
        }
    }
}

impl Drop for FileStorage {
    fn drop(&mut self) {
        match fs::remove_dir_all(format!("lua-cache/{}", self.key)) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to remove FileStorage with id: {}", self.key);
                println!("Error: {}", e);
            }
        }
    }
}
