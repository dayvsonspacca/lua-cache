mod cache;

use cache::file::FileStorage;
use std::time::Duration;

fn main() {
    let mut cache = FileStorage::new(Duration::from_secs(5));
    cache.add("car", "white");
    
    loop {
        println!("{}", cache.get("car"));
    }    
}
