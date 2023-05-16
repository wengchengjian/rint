pub struct ProducerRecord {
    topic: String,
    partition: u32,
    timestamp: u64,
    key: Vec<u8>,
    value: Vec<u8>,
}
