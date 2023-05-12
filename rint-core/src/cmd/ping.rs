use log::debug;
use serde::{Deserialize, Serialize};

use crate::{connection::Connection, protocol::Message, shutdown::Shutdown, Result};

use super::Command;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Ping {
    msg: Vec<u8>,
}

impl Ping {
    pub fn new(msg: Vec<u8>) -> Ping {
        return Ping { msg };
    }

    pub async fn apply(self, conn: &mut Connection, shutdown: &mut Shutdown) -> Result<()> {
        let msg: String = String::from_utf8(self.msg).unwrap();
        debug!("ping {}", msg);

        let mut message = Message::new(0, format!("pong {}", msg).into_bytes());
        // Write the response back to the client
        conn.write_message(&mut message).await?;

        Ok(())
    }

    pub fn into_message(self) -> Message {
        let cmd = Command::Ping(self);
        Message::new(0, serde_json::to_vec(&cmd).unwrap())
    }
}
