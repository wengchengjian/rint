pub mod constant;
pub mod server;
use std::collections::HashMap;

use constant::SERVER_INFO;
use dashmap::DashMap;
use rint_core::protocol::Message;
use serde::{Deserialize, Serialize};

pub async fn query_server_info(message: &mut Message, all: bool, key: Option<String>) {
    if let Some(key) = key {
        let mut lockMap = SERVER_INFO.lock().await;
        let res = lockMap.entry(key).or_insert(HashMap::new());
        message.set_body(serde_json::to_vec(&res).unwrap());
    } else {
        message.set_body(serde_json::to_vec(&*SERVER_INFO.lock().await).unwrap());
    }
}

pub async fn shutdown(message: &mut Message, force: bool, slient: bool) {}

pub struct ServerInfo {}

#[derive(Serialize, Deserialize)]
pub struct DashMapWrapper<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    data: HashMap<K, V>,
}

impl<K, V> From<&DashMap<K, V>> for DashMapWrapper<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn from(map: &DashMap<K, V>) -> Self {
        let mut data = HashMap::new();

        for pair in map.iter() {
            data.insert(pair.key().clone(), pair.value().clone());
        }

        DashMapWrapper { data }
    }
}
