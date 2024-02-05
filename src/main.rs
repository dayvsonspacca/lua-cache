mod cache;

use std::time::Duration;
use cache::file::CacheStorate;

fn main() {
    let mut cache = CacheStorate::new(Duration::from_secs(60));
}
