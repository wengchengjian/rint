use log::info;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::Sender;

use crate::{connection::Connection, protocol::Message, Result};

use super::Command;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Shutdown;

impl Shutdown {
    pub fn new() -> Shutdown {
        return Shutdown;
    }

    pub async fn apply(self, conn: &mut Connection, notify_shutdown: &Sender<()>) -> Result<()> {
        let msg: String = String::from("Ok");
        let mut message = Message::new(0, msg.into_bytes());
        // Write the response back to the client
        conn.write_message(&mut message).await?;

        notify_shutdown.send(()).unwrap();
        Ok(())
    }

    pub fn into_message(self) -> Message {
        let cmd = Command::Shutdown(self);
        Message::new(0, serde_json::to_vec(&cmd).unwrap())
    }
}
