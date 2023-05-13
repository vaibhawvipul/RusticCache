// utils.rs

// Define your utility functions here

// For example, you could define a function to generate a random ID for cache entries:
use rand::{Rng, thread_rng};

pub fn generate_id() -> String {
    let mut rng = thread_rng();
    let id: String = rng.gen_range(0..1000000).to_string();
    id
}

// Or you could define a function to convert a string to bytes:
pub fn str_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

// Define any other utility functions you need for your project here
