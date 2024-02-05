mod cache;

use cache::file::CacheStorate;
use std::time::Duration;

fn main() {
    let mut cache = CacheStorate::new(Duration::from_secs(60));
    cache.add("car", "white");
    cache.add("city", "new-york");
}
