pub enum CommandEnum {
    PutCommand(Put),
    GetCommand(Get),
    ExistsCommand(Exists),
    ExitCommand(Exit),
}

pub struct Put {
    pub key: String,
    pub value: String,
    pub ttl: u64,
}

pub struct Get {
    pub key: String,
}

pub struct Exists {
    pub key: String,
}

pub struct Exit {}


pub trait CommandResponse {
    fn serialize(&self) -> String;
}

pub struct PutResponse {}

pub struct GetResponse {
    pub key: String,
    pub value: Option<String>,
}

pub struct ExistsResponse {
    pub exists: bool,
}

pub struct CommandNotFoundResponse {}

impl CommandResponse for PutResponse {
    fn serialize(&self) -> String {
        return String::from("OK");
    }
}

impl CommandResponse for GetResponse {
    fn serialize(&self) -> String {
        return match &self.value {
            Some(v) => {
                let message = format!("Got {}", v);
                String::from(message)
            }
            None => {
                String::from("Key not found")
            }
        };
    }
}


impl CommandResponse for ExistsResponse {
    fn serialize(&self) -> String {
        return match &self.exists {
            true => {
                String::from("OK")
            }
            false => String::from("Key not found")
        };
    }
}

impl CommandResponse for CommandNotFoundResponse {
    fn serialize(&self) -> String {
        return String::from("Command not found");
    }
}

pub const DEFAULT_TTL: u64 = 60;

pub fn deserialize_command(input: String) -> CommandEnum {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let command = parts.get(0);

    return match command {
        Some(&"set") => {
            let key = String::from(parts[1]);
            let value = String::from(parts[2]);
            let ttl = if let Some(ttl_value) = parts.get(3) {
                ttl_value.parse::<u64>().unwrap_or_else(|_| {
                    DEFAULT_TTL
                })
            } else { DEFAULT_TTL };
            CommandEnum::PutCommand(Put { key, value, ttl })
        }
        Some(&"get") => {
            let key = String::from(parts[1]);
            CommandEnum::GetCommand(Get { key })
        }
        Some(&"exists") => {
            let key = String::from(parts[1]);
            CommandEnum::ExistsCommand(Exists { key })
        }
        Some(&"exit") => {
            CommandEnum::ExitCommand(Exit {})
        }
        _ => {
            // TODO: proper handling
            panic!("Command {command:#?} not found.");
        }
    };
}