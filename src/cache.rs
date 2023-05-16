use std::collections::HashMap;
use std::hash::Hash;
use crate::evictionpolicy::EvictionPolicy;

pub struct Cache<K, V, P>
where
    P: EvictionPolicy<K, V>,
    K: Eq + Hash + Clone,
    V: Default + Clone,
{
    capacity: usize,
    data: HashMap<K, (V, usize)>,
    policy: P,
}

impl<K, V, P> Cache<K, V, P>
where
    P: EvictionPolicy<K, V>,
    K: Eq + Hash + Clone,
    V: Default + Clone,
{
    pub fn new(policy: P, capacity: usize) -> Self {
        Cache {
            data: HashMap::new(),
            policy,
            capacity,
        }
    }

    pub fn with_data(policy: P, capacity: usize, data: HashMap<K, (V, usize)>) -> Self {
        Cache {
            data,
            policy,
            capacity,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some((value, _)) = self.data.get(key).cloned() {
            self.policy.on_access(&mut self.data, key);
            Some(value.clone())
        } else {
            None
        }
    }      

    pub fn insert(&mut self, key: K, value: V) {
        self.policy.on_insert(&mut self.data, &key);
        self.data.insert(key, (value, self.data.len()));
        if self.data.len() > self.capacity {
            self.evict();
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some((value, _)) = self.data.remove(key) {
            Some(value)
        } else {
            None
        }
    }

    fn evict(&mut self) {
        let key = self.policy.on_evict(&mut self.data);
        self.data.remove(&key);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
