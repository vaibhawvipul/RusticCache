use std::collections::HashMap;
use std::time::{Duration, Instant};

mod evictionpolicy;

pub struct Cache<K, V> {
    capacity: usize,
    data: HashMap<K, (V, Instant)>,
    eviction_policy: Box<dyn EvictionPolicy<K, V>>,
}

impl<K, V> Cache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: HashMap::new(),
            eviction_policy: Box::new(evictionpolicy::LRU::new()),
        }
    }

    pub fn with_policy<P: 'static + EvictionPolicy<K, V>>(mut self, policy: P) -> Self {
        self.eviction_policy = Box::new(policy);
        self
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.eviction_policy.on_insert(&mut self.data, &key);
        self.data.insert(key, (value, Instant::now()));
        self.eviction_policy.on_access(&mut self.data, &key);
        self.evict();
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((value, instant)) = self.data.get_mut(key) {
            self.eviction_policy.on_access(&mut self.data, key);
            if instant.elapsed() < Duration::from_secs(1) {
                return Some(value);
            }
        }
        None
    }

    fn evict(&mut self) {
        while self.data.len() > self.capacity {
            let key = self.eviction_policy.on_evict(&mut self.data);
            self.data.remove(&key);
        }
    }
}