use serde::{Deserialize, Serialize};

use crate::{connection::Connection, parse::Parser, protocol::Message, shutdown::Shutdown, Result};

use self::{ping::Ping, unknown::Unknown};

pub mod ping;
pub mod unknown;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Command {
    Ping(Ping),
    Unknown(Unknown),
}

impl Command {
    pub fn from_message(message: Message) -> Result<Command> {
        let parser = Parser::new(message)?;

        parser.parse()
    }

    pub async fn apply(self, conn: &mut Connection, shutdown: &mut Shutdown) -> crate::Result<()> {
        match self {
            Command::Ping(ping) => ping.apply(conn, shutdown).await,
            Command::Unknown(unknown) => unknown.apply(conn, shutdown).await,
        }
    }
}
