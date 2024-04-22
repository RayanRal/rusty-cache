use log::warn;
use crate::server::cache::Cache;
use crate::server::commands;
use crate::server::commands::CommandEnum;

pub fn process_command(command: CommandEnum, cache: &mut Cache) -> Box<dyn commands::CommandResponse> {
    return match command {
        CommandEnum::PutCommand(commands::Put { key, value }) => {
            cache.put(&key, &value);
            let response = commands::PutResponse {};
            Box::new(response)
        }
        CommandEnum::GetCommand(commands::Get { key }) => {
            let value = cache.get(&key).map(|s| s.clone());
            let response = commands::GetResponse {
                key,
                value,
            };
            Box::new(response)
        }
        CommandEnum::ExistsCommand(commands::Exists { key }) => {
            let exists = cache.exists(&key);
            let response = commands::ExistsResponse { exists };
            Box::new(response)
        }
        CommandEnum::ExitCommand(commands::Exit {}) => {
            warn!("Received EXIT command. Wrapping up.");
            panic!("Received EXIT command");
        }
    };
}