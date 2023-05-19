use std::{sync::atomic::AtomicI64, todo};

use crate::storage::segment::RintMessageLog;

use super::sender::Sender;

pub struct ProducerRecord {
    topic: String,
    partition: u32,
    timestamp: u64,
    key: Vec<u8>,
    value: Vec<u8>,
}

pub static PRODUCER_CLIENT_ID_SEQUENCE: AtomicI64 = AtomicI64::new(0);

pub struct RintProducer {
    pub client_id: u64,

    pub max_request_size: u64,

    pub total_memory_size: u64,

    pub sender: Sender,

    pub max_block_time: u64,
}

impl RintProducer {
    pub fn send_u8(&self, content: Vec<u8>) -> RintMessageLog {
        todo!()
    }

    pub fn send_string(&self, content: String) -> RintMessageLog {
        todo!()
    }

    pub fn send<T>(&self, content: T) -> RintMessageLog
    where
        T: Into<Vec<u8>>,
    {
        let bytes: Vec<u8> = content.into();
        todo!()
    }
}
