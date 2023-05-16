use std::io::Cursor;

use log::info;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub struct Message {
    header: MessageHeader,
    body: Vec<u8>,
}

#[derive(Debug)]
pub struct MessageHeader {
    pub version: u8,
    pub magic: u32,
    pub msg_type: u8,
    pub msg_len: u32,
}

impl MessageHeader {
    pub fn new() -> Self {
        MessageHeader {
            version: 0,
            magic: 0,
            msg_type: 0,
            msg_len: 0,
        }
    }
}

impl Message {
    pub fn new(msg_type: u8, body: Vec<u8>) -> Self {
        Message {
            header: MessageHeader {
                version: version(),
                magic: magic(),
                msg_type,
                msg_len: body.len() as u32,
            },
            body,
        }
    }

    pub fn empty() -> Self {
        Message {
            header: MessageHeader {
                version: version(),
                magic: magic(),
                msg_type: 0,
                msg_len: 0,
            },
            body: Vec::new(),
        }
    }

    pub fn set_header(&mut self, header: MessageHeader) {
        self.header = header;
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.header.msg_len = body.len() as u32;
        self.body = body;
    }

    pub fn get_header(&self) -> &MessageHeader {
        &self.header
    }

    pub fn get_body(&self) -> &Vec<u8> {
        &self.body
    }
}

pub struct MessageDecoder;
pub struct MessageEncoder;

pub struct MessageCompressor;

pub struct MessageDecompressor;

impl MessageCompressor {
    pub fn compress(data: &[u8]) -> Vec<u8> {
        let compressed = compress_prepend_size(data);
        return compressed;
    }
}

impl MessageDecompressor {
    pub fn decompress(data: &[u8]) -> Vec<u8> {
        let uncompressed = decompress_size_prepended(&data).unwrap();
        return uncompressed;
    }
}

impl MessageDecoder {
    pub async fn decode(reader: &mut Cursor<&[u8]>) -> Result<Message, std::io::Error> {
        let mut msg = Message::empty();
        let mut header = MessageHeader::new();

        header.version = reader.read_u8().await?;
        header.magic = reader.read_u32().await?;
        header.msg_type = reader.read_u8().await?;
        header.msg_len = reader.read_u32().await?;
        let mut data = vec![0; header.msg_len as usize];
        AsyncReadExt::read_exact(reader, &mut data).await?;
        msg.set_header(header);
        msg.set_body(data);
        info!("{}", reader.position());
        Ok(msg)
    }
}

impl MessageEncoder {
    pub fn encode(msg: &mut Message) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(msg.header.version);
        buf.append(&mut msg.header.magic.to_be_bytes().to_vec());
        buf.push(msg.header.msg_type);
        buf.append(&mut msg.header.msg_len.to_be_bytes().to_vec());

        buf.append(&mut msg.body);
        buf
    }
}

fn version() -> u8 {
    1
}

fn magic() -> u32 {
    0x12345678
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_compress() {
        let input: &[u8] = b"Hello people, what's up?";
        let compressed = compress_prepend_size(input);
        let uncompressed = decompress_size_prepended(&compressed).unwrap();
        assert_eq!(input, uncompressed);
    }
}
