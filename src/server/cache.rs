use std::collections::HashMap;


pub struct Cache {
    hash_map: HashMap<String, String>,
}

impl Cache {

    pub fn new() -> Cache {
        return Cache {
            hash_map: HashMap::new(),
        }
    }
    pub fn put(&mut self, key: &String, value: &String) {
        self.hash_map.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        return self.hash_map.get(key);
    }

    pub fn exists(&self, key: &String) -> bool {
        return self.hash_map.contains_key(key);
    }
}