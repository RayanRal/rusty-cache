use serde::{Deserialize, Serialize};
use crate::server::cache::{Key, Value};

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestsEnum {
    Put {
        key: Key,
        value: Value,
        ttl: u64,
    },
    Get {
        key: Key,
    },
    Exists {
        key: Key,
    },
    Exit,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReqResponseEnum {
    Put,
    Get {
        key: Key,
        value: Option<Value>,
    },
    Exists {
        exists: bool,
    },
    CommandNotFound {},
}


pub const DEFAULT_TTL: u64 = 60;

pub fn deserialize_request(input: String) -> RequestsEnum {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let command = parts.first();

    return match command {
        Some(&"set") => {
            let key = String::from(parts[1]);
            let value = String::from(parts[2]);
            let ttl = if let Some(ttl_value) = parts.get(3) {
                ttl_value.parse::<u64>().unwrap_or({
                    DEFAULT_TTL
                })
            } else { DEFAULT_TTL };
            RequestsEnum::Put { key, value, ttl }
        }
        Some(&"get") => {
            let key = String::from(parts[1]);
            RequestsEnum::Get { key }
        }
        Some(&"exists") => {
            let key = String::from(parts[1]);
            RequestsEnum::Exists { key }
        }
        Some(&"exit") => {
            RequestsEnum::Exit {}
        }
        _ => {
            // TODO: proper handling
            panic!("Request {command:#?} not found.");
        }
    };
}