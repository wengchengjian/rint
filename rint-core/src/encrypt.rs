pub trait Decoder {
    fn decode(&self, data: &[u8]) -> Vec<u8>;
}

pub trait Encoder {
    fn encode(&self, data: &[u8]) -> Vec<u8>;
}
