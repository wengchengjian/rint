use log::{debug, info};
use rint_core::Result;
use rint_core::{cmd::ping::Ping, connection::Connection, protocol::Message};
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client> {
        let socket = TcpStream::connect(addr).await?;

        let connection = Connection::new(socket);

        Ok(Client { connection })
    }

    pub async fn ping(&mut self, msg: Option<Vec<u8>>) -> Result<()> {
        let ping = Ping::new(msg.unwrap_or_else(|| "hello".into()));

        debug!("{:?}", ping);
        let mut message = ping.into_message();

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
