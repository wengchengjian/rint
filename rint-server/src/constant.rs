use std::{collections::HashMap, sync::Arc};

use dashmap::DashMap;
use lazy_static::lazy_static;
use serde::Serialize;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref SERVER_INFO: Arc<Mutex<HashMap<String, HashMap<String, String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}
