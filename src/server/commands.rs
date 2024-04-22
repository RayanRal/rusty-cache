pub trait Command {
    // fn execute(&self);
}

// Define concrete command types
pub struct Put {
    key: String,
    value: String,
}

pub struct Get {
    key: String,
}

pub struct Exists {
    key: String,
}

pub struct Exit {}

impl Command for Put {}

impl Command for Get {}

impl Command for Exists {}

impl Command for Exit {}


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
