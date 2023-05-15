use std::{collections::HashMap};
use std::rc::Rc;
use crate::evictionpolicy::{EvictionPolicy, LruPolicy};

const CACHE_CAPACITY: usize = 1024;

pub struct Cache<K, V>
where
    P: EvictionPolicy<K, V>,
    K: Eq + std::hash::Hash + Clone,
    V: Default,
{
    capacity: usize,
    data: HashMap<K, (Rc<V>, usize)>,
    policy: Box<dyn EvictionPolicy<K, Rc<V>>>,
}

impl<K, V> Cache<K, V>
where
    P: EvictionPolicy<K, V>,
    K: Eq + std::hash::Hash + Clone,
    V: Default,
{
    pub fn new() -> Self {
        Cache {
            data: HashMap::new(),
            policy: Box::new(LruPolicy::new()),
            capacity: CACHE_CAPACITY,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<Rc<V>> {
        if let Some((value, _)) = self.data.get(key).cloned() {
            self.policy.on_access(&mut self.data, key);
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.policy.on_insert(&mut self.data, &key);
        let rc_value = Rc::new(value);
        self.data.insert(key, (rc_value, self.data.len()));
    }

    pub fn remove(&mut self, key: &K) -> Option<Rc<V>> {
        let value = self.data.remove(key)?;
        Some(value.0)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
