use std::cmp::Reverse;
use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use log::info;
use priority_queue::PriorityQueue;


pub struct Cache {
    hash_map: Arc<Mutex<HashMap<String, String>>>,
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
                        info!("It's not yet time to expire {key}");
                        break;
                    }
                    info!("{key} expired, removing");
                    hash_map_clone.lock().unwrap().remove(key);
                    ttl_queue_clone.lock().unwrap().pop();
                }
            }
        });


        let cache = Cache {
            hash_map,
            ttl_queue,
        };
        return cache;
    }
    pub fn put(&mut self, key: &String, value: &String) {
        self.hash_map.lock().unwrap().insert(key.to_string(), value.to_string());
        let expiration_time = SystemTime::now().add(Duration::from_secs(15));
        self.ttl_queue.lock().unwrap().push(key.to_string(), Reverse(expiration_time));
    }

    pub fn get(&self, key: &String) -> Option<String> {
        return self.hash_map.lock().unwrap().get(key).map(|s| s.clone());
    }

    pub fn exists(&self, key: &String) -> bool {
        return self.hash_map.lock().unwrap().contains_key(key);
    }
}