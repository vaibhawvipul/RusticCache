# RusticCache

RusticCache is an open-source cache store built using the Rust programming language. It is designed to be fast, efficient, and easy to use. RusticCache provides an in-memory cache that stores key-value pairs for quick retrieval.

## Build and Run

- `cargo build`
- `cargo run`

## It is still under-development. 

## Features (To-do)
- Fast and efficient in-memory cache store
- Supports key-value pair storage and retrieval
- Thread-safe operations for concurrent access
- Configurable cache size and expiration time for items
- Simple and easy-to-use API for integration with other Rust projects

## Usage
Here's a simple example of how to use RusticCache in your Rust project:

```
use rusticcache::CacheStore;

fn main() {
    // Create a new cache store with a maximum size of 100 items
    let mut cache = CacheStore::new(100);

    // Insert a key-value pair into the cache
    cache.insert("key1", "value1");

    // Retrieve a value from the cache using its key
    let value = cache.get("key1");

    println!("Value of key1: {}", value.unwrap());
}
```

## License
RusticCache is released under the Apache license. See [LICENSE](https://github.com/vaibhawvipul/RusticCache/blob/main/LICENSE) for more information.

## Contributing
Contributions to RusticCache are always welcome! If you find a bug or have a feature request, please open an issue. If you want to contribute code, please fork the repository, make your changes, and submit a pull request.