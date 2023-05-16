use serde::{Deserialize, Serialize};

use crate::{protocol::Message, shutdown::Shutdown, Result};

use super::Command;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Close;

impl Close {
    pub fn new() -> Close {
        return Close;
    }

    pub async fn apply(self, shutdown: &mut Shutdown) -> Result<()> {
        shutdown.is_shutdown = true;
        Ok(())
    }

    pub fn into_message(self) -> Message {
        let cmd = Command::Close(self);
        Message::new(0, serde_json::to_vec(&cmd).unwrap())
    }
}
