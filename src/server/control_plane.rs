use log::warn;
use crate::server::cache::Cache;
use crate::server::commands;
use crate::server::commands::CommandEnum;

pub fn process_command(command: CommandEnum, cache: &mut Cache) -> Box<dyn commands::CommandResponse> {
    match command {
        CommandEnum::Put { key, value, ttl } => {
            cache.put(&key, &value, ttl);
            let response = commands::PutResponse {};
            Box::new(response)
        }
        CommandEnum::Get { key } => {
            let value = cache.get(&key);
            let response = commands::GetResponse {
                key,
                value,
            };
            Box::new(response)
        }
        CommandEnum::Exists { key } => {
            let exists = cache.exists(&key);
            let response = commands::ExistsResponse { exists };
            Box::new(response)
        }
        CommandEnum::Exit {} => {
            warn!("Received EXIT command. Wrapping up.");
            panic!("Received EXIT command");
        }
    }
}