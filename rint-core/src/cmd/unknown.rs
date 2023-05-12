use log::debug;
use serde::{Deserialize, Serialize};

use crate::{connection::Connection, protocol::Message, shutdown::Shutdown, Result};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Unknown {
    cmd: Vec<u8>,
}

impl Unknown {
    pub fn new(cmd: Vec<u8>) -> Unknown {
        return Unknown { cmd };
    }

    pub async fn apply(self, conn: &mut Connection, shutdown: &mut Shutdown) -> Result<()> {
        let msg: String = String::from_utf8(self.cmd).unwrap();
        debug!("unknown cmd {}", msg);

        let mut message = Message::new(0, format!("unknown cmd {}", msg).into_bytes());
        // Write the response back to the client
        conn.write_message(&mut message).await?;

        Ok(())
    }
}
