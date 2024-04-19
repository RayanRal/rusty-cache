use std::collections::HashMap;
use std::ptr::hash;


struct Cache {
    hash_map: HashMap<String, String>
}

impl Cache {

    pub fn add(&mut self, key: String, value: String) {
        self.hash_map.insert(key, value);
    }

    pub fn read(&self, key: String) -> Option<&String> {
        return self.hash_map.get(&key)
    }

    pub fn exists(&self, key: String) -> bool {
        return self.hash_map.contains_key(&key);
    }
    
}