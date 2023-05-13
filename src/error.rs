use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Cache is full")]
    CacheFull,
    #[error("Cache key not found")]
    KeyNotFound,
    // Add more custom error types here
}
