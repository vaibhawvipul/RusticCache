use rustycache::Cache;
use std::{thread, time};

fn main() {
    // Create a new cache that can hold up to 10 items, with an expiration time of 10 seconds
    let mut cache = Cache::new(10, time::Duration::from_secs(10));

    // Insert some values into the cache
    cache.insert("key1", "value1");
    cache.insert("key2", "value2");
    cache.insert("key3", "value3");

    // Wait for 5 seconds
    thread::sleep(time::Duration::from_secs(5));

    // Retrieve a value from the cache
    let value1 = cache.get("key1");
    println!("Value for key1: {:?}", value1);

    // Wait for another 6 seconds
    thread::sleep(time::Duration::from_secs(6));

    // Retrieve a value from the cache again
    let value2 = cache.get("key1");
    println!("Value for key1: {:?}", value2);

    // Insert a new value into the cache
    cache.insert("key4", "value4");

    // Print all the values in the cache
    for (key, value) in cache.iter() {
        println!("{}: {}", key, value);
    }
}
