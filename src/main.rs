mod cache;

use cache::file::CacheStorate;
use std::time::Duration;

fn main() {
    let mut cache = CacheStorate::new(Duration::from_secs(5));
    cache.add("car", "white");
    
    loop {
        println!("{}", cache.get("car"));
    }    
}
