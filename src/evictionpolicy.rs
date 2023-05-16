use std::collections::HashMap;
use std::hash::Hash;

pub trait EvictionPolicy<K, V> 
where
    K: Eq + Hash + Clone,
    V: Default + Clone,
{
    fn on_insert(&mut self, data: &mut HashMap<K, (V, usize)>, key: &K);
    fn on_access(&mut self, data: &mut HashMap<K, (V, usize)>, key: &K);
    fn on_evict(&mut self, data: &mut HashMap<K, (V, usize)>) -> K;
}

pub struct LruPolicy {
    indexes: HashMap<usize, Option<usize>>,
}

impl LruPolicy {
    pub fn new(capacity: usize) -> Self {
        LruPolicy {
            indexes: HashMap::with_capacity(capacity),
        }
    }

    fn update_indexes(&mut self, index: usize) {
        for (_, value) in self.indexes.iter_mut() {
            if let Some(idx) = value {
                if *idx == index {
                    *value = None;
                } else if *idx < index {
                    *idx += 1;
                }
            }
        }

        self.indexes.insert(0, Some(index));
    }
}

impl<K, V: Clone> EvictionPolicy<K, V> for LruPolicy
where
    K: Eq + Hash + Clone,
    V: Default + Clone,
{
    fn on_insert(&mut self, data: &mut HashMap<K, (V, usize)>, key: &K) {
        if !data.contains_key(key) && data.len() >= CACHE_CAPACITY {
            self.on_evict(data);
        }
    }

    fn on_access(&mut self, data: &mut HashMap<K, (V, usize)>, key: &K) {
        if let Some((value, index)) = data.remove(key) {
            data.insert(key.clone(), (value, data.len()));
            //update_values(data, index);
            self.update_indexes(index);
        }
    }

    fn on_evict(&mut self, data: &mut HashMap<K, (V, usize)>) -> K {
        let key = data.keys().next().cloned().unwrap();
        data.remove(&key);
        key
    }
}

const CACHE_CAPACITY: usize = 10;
