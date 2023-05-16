mod cache;
mod evictionpolicy;

use cache::Cache;
use evictionpolicy::LruPolicy;

fn main() {
    // Create a new cache with LRU eviction policy
    let mut cache = Cache::new(LruPolicy::new(10), 10);


    // Insert key-value pairs into the cache
    cache.insert("key1", "value1");
    cache.insert("key2", "value2");
    cache.insert("key3", "value3");

    // Get values from the cache
    println!("Value for key 'key1': {:?}", cache.get(&"key1"));
    println!("Value for key 'key2': {:?}", cache.get(&"key2"));
    println!("Value for key 'key3': {:?}", cache.get(&"key3"));

    // print cache len
    println!("Cache len: {}", cache.len());

    // Remove a key from the cache
    cache.remove(&"key2");

    // Get the updated values from the cache
    println!("Value for key 'key1': {:?}", cache.get(&"key1"));
    println!("Value for key 'key2': {:?}", cache.get(&"key2"));
    println!("Value for key 'key3': {:?}", cache.get(&"key3"));
}
