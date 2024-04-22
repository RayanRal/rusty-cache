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


pub trait CommandResponse {}

pub struct PutResponse {}

pub struct GetResponse {
    pub key: String,
    pub value: Option<String>,
}

pub struct ExistsResponse {
    pub exists: bool,
}

pub struct CommandNotFoundResponse {}

impl CommandResponse for PutResponse {}

impl CommandResponse for GetResponse {}

impl CommandResponse for ExistsResponse {}

impl CommandResponse for CommandNotFoundResponse {}
//
// // Implement the Command trait for each concrete command type
// impl Command for AddCommand {
//     fn execute(&self) {
//         println!("Executing AddCommand: {} + {} = {}", self.x, self.y, self.x + self.y);
//     }
// }
