# RusticCache

RusticCache is an open-source cache store built using the Rust programming language. It is designed to be fast, efficient, and easy to use. RusticCache provides an in-memory cache that stores key-value pairs for quick retrieval.

## Build and Run

- `cargo build`
- `cargo run`

## It is still under active development. It works though.

## Features (To-do)
- Fast and efficient in-memory cache store
- Supports key-value pair storage and retrieval
- Thread-safe operations for concurrent access
- Configurable cache size and expiration time for items
- Simple and easy-to-use API for integration with other Rust projects

## Usage

### steps to use rustic_cache in local repos - 
Here's a simple example of how to use RusticCache in your Rust project:

Generate the crate by following the steps mentioned earlier:

1. Clone the repository: `git clone https://github.com/vaibhawvipul/rustic_cache.git`
2. Navigate to the repository: `cd rustic_cache`
3. Build the crate: `cargo build --release`
4. After building the crate, a compiled crate file will be generated in the __target/release__ directory. The crate file will have the __.rlib__ extension.

In your other Rust projects, you can add the generated crate as a dependency in the Cargo.toml file. Open the __Cargo.toml__ file of your project and add the following line under the [dependencies] section:

`rustic_cache = { path = "/path/to/rustic_cache/target/release" }`

Replace __/path/to/rustic_cache__ with the actual path to the cloned rustic_cache repository on your machine.

Save the __Cargo.toml__ file. Cargo will now be able to find and use the rustic_cache crate in your project.

In your Rust code, you can import and use the rustic_cache crate by adding the following line at the top of your source file:

`use rustic_cache::CacheStore;`

You can now use the CacheStore type and other functionalities provided by the rustic_cache crate in your project.

Remember to update the __/path/to/rustic_cache__ with the correct path to the generated crate on your machine. Additionally, ensure that you have the necessary permissions to access the rustic_cache directory and its files.

By following these steps, you'll be able to import and use the rustic_cache crate in your other Rust projects locally.

### Example
```
use rustic_cache::CacheStore;

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