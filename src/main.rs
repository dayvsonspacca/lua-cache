mod cache;

use cache::memory::MemoryStorage;
use std::time::Duration;

fn main() {
    let mut cache = MemoryStorage::new(Duration::from_secs(1));
    cache.add("car", "white");

    loop {
        println!("{}", cache.get("car"));
    }
}
