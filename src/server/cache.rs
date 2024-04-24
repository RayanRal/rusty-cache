use std::cmp::Reverse;
use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use log::debug;
use priority_queue::PriorityQueue;


pub struct Cache {
    hash_map: Arc<Mutex<HashMap<String, String>>>,
    // this design makes cache itself tightly coupled to eviction mechanism
    // this is not ideal, and should be refactored out later
    ttl_queue: Arc<Mutex<PriorityQueue<String, Reverse<SystemTime>>>>,
}

impl Cache {
    pub fn new() -> Cache {
        let hash_map: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
        let ttl_queue: Arc<Mutex<PriorityQueue<String, Reverse<SystemTime>>>> = Arc::new(Mutex::new(PriorityQueue::new()));

        let hash_map_clone = hash_map.clone();
        let ttl_queue_clone = ttl_queue.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(2));

                let cur_time = SystemTime::now();
                while let Some((key, expiration_time)) = ttl_queue_clone.lock().unwrap().peek() {
                    if expiration_time.0 >= cur_time {
                        debug!("It's not yet time to expire {key}");
                        break;
                    }
                    debug!("{key} expired, removing");
                    hash_map_clone.lock().unwrap().remove(key);
                    ttl_queue_clone.lock().unwrap().pop();
                }
            }
        });

        Cache {
            hash_map,
            ttl_queue,
        }
    }
    pub fn put(&mut self, key: &String, value: &String, ttl: u64) {
        self.hash_map.lock().unwrap().insert(key.to_string(), value.to_string());
        let expiration_time = SystemTime::now().add(Duration::from_secs(ttl));
        // pushing to the queue existing key overwrites its expiration time
        self.ttl_queue.lock().unwrap().push(key.to_string(), Reverse(expiration_time));
    }

    pub fn get(&self, key: &String) -> Option<String> {
        return self.hash_map.lock().unwrap().get(key).cloned();
    }

    pub fn exists(&self, key: &String) -> bool {
        return self.hash_map.lock().unwrap().contains_key(key);
    }
}