use log::{debug, info};
use rint_core::cmd::close::Close;
use rint_core::shutdown::Shutdown;
use rint_core::{cmd, Result};
use rint_core::{cmd::ping::Ping, connection::Connection, protocol::Message};
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::command::{PublishArg, SubscribeArg};

pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client> {
        let socket = TcpStream::connect(addr).await?;
        let connection = Connection::new(socket);

        Ok(Client { connection })
    }

    pub async fn close(&mut self) -> Result<()> {
        self.connection.close().await?;
        Ok(())
    }

    pub async fn ping(&mut self, msg: String) -> Result<()> {
        let ping = Ping::new(msg.into_bytes());

        debug!("{:?}", ping);
        let mut message = ping.into_message();

        self.connection.write_message(&mut message).await?;

        let res = self.read_response().await?;

        info!("{:?}", String::from_utf8(res.clone())?);
        Ok(())
    }

    pub async fn info(&mut self, key: Option<String>) -> Result<()> {
        Ok(())
    }

    pub async fn subscribe(&mut self, arg: SubscribeArg) -> Result<()> {
        Ok(())
    }

    pub async fn publish(&mut self, arg: PublishArg) -> Result<()> {
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        let shutdown = cmd::shutdown::Shutdown::new();

        debug!("{:?}", shutdown);
        let mut message = shutdown.into_message();

        self.connection.write_message(&mut message).await?;

        let res = self.read_response().await?;

        info!("{:?}", String::from_utf8(res.clone())?);
        Ok(())
    }

    pub async fn read_response(&mut self) -> Result<Vec<u8>> {
        let message = self.connection.read_message().await?;
        debug!("{:?}", message);

        match message {
            Some(message) => Ok(message.get_body().to_owned()),
            None => Err("connection reset by server".into()),
        }
    }
}
