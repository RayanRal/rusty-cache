use crate::server::cache::{Key, Value};
use crate::server::requests::RequestsEnum::{Exists, Exit, Get, Put};

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

impl RequestsEnum {
    pub fn serialize(&self) -> String {
        match self {
            Put { key, value, ttl } => { format!("put {key} {value} {ttl}") }
            Get { key } => { format!("get {key}") }
            Exists { key } => { format!("exists {key}") }
            Exit {} => String::from("exit")
        }
    }
}

pub trait ReqResponse {
    fn serialize(&self) -> String;
}

pub struct PutResponse {}

pub struct GetResponse {
    pub key: Key,
    pub value: Option<Value>,
}

pub struct ExistsResponse {
    pub exists: bool,
}

pub struct CommandNotFoundResponse {}

impl ReqResponse for PutResponse {
    fn serialize(&self) -> String {
        String::from("OK")
    }
}

impl ReqResponse for GetResponse {
    fn serialize(&self) -> String {
        match &self.value {
            Some(v) => {
                format!("Got {}", v)
            }
            None => {
                String::from("Key not found")
            }
        }
    }
}


impl ReqResponse for ExistsResponse {
    fn serialize(&self) -> String {
        match &self.exists {
            true => {
                String::from("OK")
            }
            false => String::from("Key not found")
        }
    }
}

impl ReqResponse for CommandNotFoundResponse {
    fn serialize(&self) -> String {
        String::from("Command not found")
    }
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