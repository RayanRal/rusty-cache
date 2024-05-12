use serde::{Deserialize, Serialize};
use crate::server::cache::{Key, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    ErrorProcessingCommand {},
}

