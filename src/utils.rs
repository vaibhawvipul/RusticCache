use rand::{Rng, thread_rng};

pub fn generate_id() -> String {
    let mut rng = thread_rng();
    let id: String = rng.gen_range(0..1000000).to_string();
    id
}

pub fn str_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}