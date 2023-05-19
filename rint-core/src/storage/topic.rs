use std::collections::HashMap;

pub struct Topic {
    pub name: String,

    pub partition: u32,

    pub replica_num: u32,
}

pub struct TopicMessage {
    pub partition: u32,

    pub offset: u64,

    pub message: Vec<u8>,

    pub timestamp: u64,
}

pub struct ConsumerGroup {
    pub group_name: String,

    pub topics: Vec<String>,

    ///
    /// topic name -> TopicOffset
    pub consumer_offsets: HashMap<String, TopicConsumerOffset>,
}

pub struct TopicConsumerOffset {
    pub current_offset: u64,
}
