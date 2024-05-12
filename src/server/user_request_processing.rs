use std::io::{BufRead, BufReader, BufWriter, Write};
use log::{info, warn};
use crate::server::cache::Cache;
use crate::server::cluster::{Cluster, NodeId};
use crate::server::requests::{ReqResponseEnum, RequestsEnum};

pub fn process_client_request(request: RequestsEnum,
                              cache: &mut Cache,
                              cluster: &mut Cluster,
) -> ReqResponseEnum {
    match request.clone() {
        RequestsEnum::Put { key, value, ttl } => {
            let is_key_local = cluster.is_key_local(&key);
            if is_key_local {
                cache.put(&key, &value, ttl);
                ReqResponseEnum::Put {}
            } else {
                let target_node = cluster.get_node_for_key(&key);
                redirect_request(cluster, target_node, request.clone())
            }
        }
        RequestsEnum::Get { key } => {
            let is_key_local = cluster.is_key_local(&key);
            if is_key_local {
                let value = cache.get(&key);
                ReqResponseEnum::Get {
                    key,
                    value,
                }
            } else {
                let target_node = cluster.get_node_for_key(&key);
                redirect_request(cluster, target_node, request.clone())
            }
        }
        RequestsEnum::Exists { key } => {
            let is_key_local = cluster.is_key_local(&key);
            if is_key_local {
                let exists = cache.exists(&key);
                ReqResponseEnum::Exists { exists }
            } else {
                let target_node = cluster.get_node_for_key(&key);
                redirect_request(cluster, target_node, request.clone())
            }
        }
        RequestsEnum::Exit {} => {
            warn!("Received EXIT command. Wrapping up.");
            panic!("Received EXIT command");
        }
    }
}

// TODO: this function probably shouldn't be here
fn redirect_request(cluster: &mut Cluster, target_node: NodeId, request: RequestsEnum) -> ReqResponseEnum {
    let connection_or_none = cluster.get_node_connection(&target_node);
    match connection_or_none {
        None => {
            warn!("Received EXIT command. Wrapping up.");
            ReqResponseEnum::ErrorProcessingCommand {}
        }
        Some(arc_conn) => {
            let stream = arc_conn.lock().unwrap();

            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut writer = BufWriter::new(stream.try_clone().unwrap());

            let mut request_str = serde_json::to_string(&request).unwrap();
            request_str.push('\n');

            writer.write_all(request_str.as_bytes()).unwrap();
            writer.flush().unwrap();

            let mut s = String::new();
            reader.read_line(&mut s).unwrap();
            info!("Received response: {s}");
            let response: ReqResponseEnum = serde_json::from_str(&s).unwrap();
            response
        }
    }
}