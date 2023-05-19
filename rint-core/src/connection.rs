use std::{
    collections::HashMap,
    io::{self, Cursor},
};

use bytes::{Buf, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

use crate::protocol::{
    Message, MessageCompressor, MessageDecoder, MessageDecompressor, MessageEncoder,
};

#[derive(Debug)]
pub struct Connection {
    pub stream: BufWriter<TcpStream>,

    pub buffer: BytesMut,

    pub session: Session,
}

#[derive(Debug)]
pub struct Session {
    pub producer_context: ProducerContext,
}

#[derive(Debug)]
pub struct ProducerContext {
    pub produce_topic_seq: HashMap<String, u64>,
}

impl ProducerContext {
    pub fn new() -> ProducerContext {
        return ProducerContext {
            produce_topic_seq: HashMap::new(),
        };
    }
    pub fn get_seq_and_add(&mut self, topic: &str) -> u64 {
        let seq = self.produce_topic_seq.entry(topic.to_string()).or_insert(0);
        *seq += 1;

        return seq.clone();
    }
}

impl Session {
    pub fn new() -> Session {
        return Session {
            producer_context: ProducerContext::new(),
        };
    }
}
impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
            session: Session::new(),
        }
    }

    pub async fn has_data(&self) -> bool {
        self.stream.buffer().len() > 0
    }

    pub async fn read_message(&mut self) -> crate::Result<Option<Message>> {
        loop {
            // Attempt to parse a frame from the buffered data. If enough data
            // has been buffered, the frame is returned.
            if let Some(message) = self.parse_message().await? {
                return Ok(Some(message));
            }

            // There is not enough buffered data to read a frame. Attempt to
            // read more data from the socket.
            //
            // On success, the number of bytes is returned. `0` indicates "end
            // of stream".
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    pub async fn write_message(&mut self, message: &mut Message) -> io::Result<()> {
        let body = MessageCompressor::compress(message.get_body());
        message.set_body(body);

        let data = MessageEncoder::encode(message);

        self.stream.write_all(&data).await?;
        self.stream.flush().await
    }

    pub fn write_message_sync(&mut self, message: &mut Message) -> io::Result<()> {
        let body = MessageCompressor::compress(message.get_body());
        message.set_body(body);

        let data = MessageEncoder::encode(message);

        self.stream.write_all(&data);
        self.stream.flush();
        Ok(())
    }

    pub async fn parse_message(&mut self) -> Result<Option<Message>, std::io::Error> {
        let mut cursor = Cursor::new(&self.buffer[..]);

        cursor.set_position(0);

        let message = match MessageDecoder::decode(&mut cursor).await {
            Ok(message) => Some(message),
            Err(_) => return Ok(None),
        };
        let len = cursor.position() as usize;

        self.buffer.advance(len);

        if let Some(mut message) = message {
            message.set_body(MessageDecompressor::decompress(&message.get_body()));
            return Ok(Some(message));
        }

        return Ok(None);
    }

    pub async fn close(&mut self) -> io::Result<()> {
        self.stream.shutdown().await?;
        Ok(())
    }
}
