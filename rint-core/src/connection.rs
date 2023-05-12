use std::io::{self, Cursor, Error};

use bytes::BytesMut;
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
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
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

        let data = MessageEncoder::encode(message).await;

        self.stream.write_all(&data).await?;
        self.stream.flush().await
    }

    pub async fn parse_message(&mut self) -> Result<Option<Message>, std::io::Error> {
        let mut cursor = Cursor::new(&self.buffer[..]);

        let message = match MessageDecoder::decode(&mut cursor).await {
            Ok(message) => Some(message),
            Err(_) => return Ok(None),
        };
        if let Some(mut message) = message {
            message.set_body(MessageDecompressor::decompress(&message.get_body()));
            return Ok(Some(message));
        }
        return Ok(None);
    }
}
