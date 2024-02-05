mod cache;

use cache::file::CacheStorage;
use std::time::Duration;

fn main() {
    let mut cache = CacheStorage::new(Duration::from_secs(5));
    cache.add("car", "white");
    
    loop {
        println!("{}", cache.get("car"));
    }    
}
