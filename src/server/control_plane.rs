use log::warn;
use crate::server::cache::Cache;
use crate::server::commands::{CmdResponse, CommandsEnum};
use crate::server::{commands, requests};
use crate::server::requests::RequestsEnum;

pub fn process_client_request(request: RequestsEnum, cache: &mut Cache) -> Box<dyn requests::ReqResponse> {
    match request {
        RequestsEnum::Put { key, value, ttl } => {
            cache.put(&key, &value, ttl);
            let response = requests::PutResponse {};
            Box::new(response)
        }
        RequestsEnum::Get { key } => {
            let value = cache.get(&key);
            let response = requests::GetResponse {
                key,
                value,
            };
            Box::new(response)
        }
        RequestsEnum::Exists { key } => {
            let exists = cache.exists(&key);
            let response = requests::ExistsResponse { exists };
            Box::new(response)
        }
        RequestsEnum::Exit {} => {
            warn!("Received EXIT command. Wrapping up.");
            panic!("Received EXIT command");
        }
    }
}

pub fn process_command(command: CommandsEnum, cache: &mut Cache) {
    match command {
        CommandsEnum::JoinCluster { .. } => {}
        CommandsEnum::LeaveCluster { .. } => {}
        CommandsEnum::GetClusterState { .. } => {}
        CommandsEnum::GetKeysForBucket { .. } => {}
    }
}