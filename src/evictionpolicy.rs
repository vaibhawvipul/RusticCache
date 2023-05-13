use std::collections::HashMap;

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

    pub fn touch(&mut self, key: &str) {
        if let Some(&pos) = self.map.get(key) {
            self.list.remove(pos);
            self.list.push(key.to_string());
            self.update_map();
        } else {
            self.list.push(key.to_string());
            self.map.insert(key.to_string(), self.list.len() - 1);
        }
    }

    pub fn evict(&mut self) -> Option<String> {
        if let Some(key) = self.list.get(0) {
            let pos = self.map.remove(*key).unwrap();
            self.list.remove(pos);
            self.update_map();
            Some(key.clone())
        } else {
            None
        }
    }

    fn update_map(&mut self) {
        for (i, key) in self.list.iter().enumerate() {
            self.map.insert(key.to_string(), i);
        }
    }
}
