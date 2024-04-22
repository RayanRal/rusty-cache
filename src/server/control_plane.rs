use log::warn;
use crate::server::cache::Cache;
use crate::server::commands;

pub fn process_command(input: &String, cache: &mut Cache) -> Box<dyn commands::CommandResponse> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let command = parts.get(0);

    return match command {
        Some(&"set") => {
            let key = String::from(parts[1]);
            let value = String::from(parts[2]);
            cache.put(&key, &value);
            let response = commands::PutResponse {};
            Box::new(response) // "Called set with: {key} -> {value}");
        }
        Some(&"get") => {
            let key = String::from(parts[1]);
            let value = cache.get(&key).map(|s| s.clone());
            let response = commands::GetResponse {
                key,
                value,
            };
            Box::new(response)
        }
        Some(&"exists") => {
            let key = String::from(parts[1]);
            let exists = cache.exists(&key);
            let response = commands::ExistsResponse { exists };
            Box::new(response)
        }
        Some(&"exit") => {
            warn!("Received EXIT command. Wrapping up.");
            panic!("Received EXIT command");
        }
        _ => {
            warn!("Command {command:#?} not found.");
            Box::new(commands::CommandNotFoundResponse {})
        }
    };
}