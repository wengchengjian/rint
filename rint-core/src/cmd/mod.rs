use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::Sender;

use crate::{connection::Connection, parse::Parser, protocol::Message, shutdown::Shutdown, Result};

use self::{close::Close, ping::Ping, unknown::Unknown};

pub mod close;
pub mod ping;
pub mod publish;
pub mod shutdown;
pub mod subscribe;
pub mod unknown;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Command {
    Ping(Ping),
    Unknown(Unknown),
    Close(Close),
    Shutdown(shutdown::Shutdown),
}

impl Command {
    pub fn from_message(message: Message) -> Result<Command> {
        let parser = Parser::new(message)?;

        parser.parse()
    }

    pub async fn apply(
        self,
        conn: &mut Connection,
        shutdown: &mut Shutdown,
        notify_shutdown: &Sender<()>,
    ) -> crate::Result<()> {
        match self {
            Command::Ping(ping) => ping.apply(conn, shutdown).await,
            Command::Unknown(unknown) => unknown.apply(conn, shutdown).await,
            Command::Close(close) => close.apply(shutdown).await,
            Command::Shutdown(shutdown) => shutdown.apply(conn, notify_shutdown).await,
        }
    }
}
