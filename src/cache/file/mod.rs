use std::{
    collections::HashMap,
    fs,
    time::{Duration, SystemTime},
};

use rand::Rng;

pub struct CacheStorate {
    key: String,
    ttl: Duration,
    data: HashMap<String, (String, SystemTime)>,
}

impl CacheStorate {
    pub fn new(ttl: Duration) -> CacheStorate {
        let key = rand::thread_rng().gen::<u64>().to_string();

        match fs::create_dir(format!("lua-cache/{}", key)) {
            Ok(_) => println!("Criada"),
            Err(e) => println!("Erro {}", e),
        }

        CacheStorate {
            key,
            ttl,
            data: HashMap::new(),
        }
    }
}
