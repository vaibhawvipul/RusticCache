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
    list: Vec<String>,
    map: HashMap<String, usize>,
}

impl LruPolicy {
    pub fn new() -> LruPolicy {
        LruPolicy {
            list: Vec::new(),
            map: HashMap::new(),
        }
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
            update_indexes(data, index);
        }
    }

    fn on_evict(&mut self, data: &mut HashMap<K, (V, usize)>) -> K {
        let key = data.keys().next().cloned().unwrap();
        data.remove(&key);
        key
    }
}

fn update_indexes<K, V>(data: &mut HashMap<K, (V, usize)>, start_index: usize)
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: Default + Clone,
{
    // First loop: borrow `data` as immutable and update the indexes
    for (_, (_, index)) in data.iter_mut() {
        if *index > start_index {
            *index -= 1;
        } else if *index == start_index {
            *index = data.len() - 1;
        }
    }

    // Second loop: borrow `data` as mutable and update the values
    for (key, (value, index)) in data.iter_mut() {
        if *index == data.len() {
            *index = start_index;
            let old_value = std::mem::replace(value, V::default());
            data.insert(key.clone(), (old_value.clone(), start_index));
        }
    }
}

const CACHE_CAPACITY: usize = 10;
