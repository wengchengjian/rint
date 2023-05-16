use std::{future::Future, sync::Arc, time::Duration};

use log::{debug, error, info};
use rint_core::{
    cmd::Command, connection::Connection, shutdown::Shutdown, Result, MAX_CONNECTIONS,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{
        broadcast::{self, Sender},
        mpsc, Semaphore,
    },
    time,
};

#[derive(Debug)]
pub struct Listener {
    listener: TcpListener,

    limit_connections: Arc<Semaphore>,

    notify_shutdown: broadcast::Sender<()>,

    shutdown_complete_tx: mpsc::Sender<()>,
}

#[derive(Debug)]
pub struct Handler {
    connection: Connection,

    shutdown: Shutdown,

    notify_shutdown: Sender<()>,

    _shutdown_complete: mpsc::Sender<()>,
}

pub async fn run(listener: TcpListener, shutdown: impl Future) {
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);

    let mut shut = notify_shutdown.subscribe();

    let mut server = Listener {
        listener,
        limit_connections: Arc::new(Semaphore::new(MAX_CONNECTIONS.into())),
        notify_shutdown,
        shutdown_complete_tx,
    };
    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                error!("failed to accept, reason:{}", err);
            }
        }
        _ = shutdown => {
            // The shutdown signal has been received.
            info!("shutting down");
        }
        _shut = shut.recv() => {
            info!("shutting down");

        }
    }

    let Listener {
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = server;

    // When `notify_shutdown` is dropped, all tasks which have `subscribe`d will
    // receive the shutdown signal and can exit
    drop(notify_shutdown);
    // Drop final `Sender` so the `Receiver` below can complete
    drop(shutdown_complete_tx);

    // 等待所有的活跃连接完成处理
    let _ = shutdown_complete_rx.recv().await;
}

impl Listener {
    pub async fn run(&mut self) -> Result<()> {
        info!("accepting inbound connections");

        loop {
            let permit = self
                .limit_connections
                .clone()
                .acquire_owned()
                .await
                .unwrap();

            let socket = self.accept().await?;

            let mut handler = Handler {
                // Initialize the connection state. This allocates read/write
                // buffers to perform redis protocol frame parsing.
                connection: Connection::new(socket),

                notify_shutdown: self.notify_shutdown.clone(),
                // Receive shutdown notifications.
                shutdown: Shutdown::new(self.notify_shutdown.subscribe()),

                // Notifies the receiver half once all clones are
                // dropped.
                _shutdown_complete: self.shutdown_complete_tx.clone(),
            };

            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    error!("connection error: {}", err);
                }

                drop(permit);
            });
        }
    }

    pub async fn accept(&mut self) -> Result<TcpStream> {
        let mut backoff = 1;

        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => {
                    if backoff > 64 {
                        return Err(err.into());
                    }
                }
            }

            time::sleep(Duration::from_secs(backoff)).await;

            backoff = backoff * 2;
        }
    }
}

impl Handler {
    pub async fn run(&mut self) -> Result<()> {
        while !self.shutdown.is_shutdown() {
            let maybe_message = tokio::select!(
                res = self.connection.read_message() => res?,
                _ = self.shutdown.recv() => return Ok(())
            );

            let message = match maybe_message {
                Some(message) => message,
                None => return Ok(()),
            };

            let cmd = Command::from_message(message)?;
            debug!("{:?}", cmd);

            cmd.apply(
                &mut self.connection,
                &mut self.shutdown,
                &self.notify_shutdown,
            )
            .await?;
        }
        Ok(())
    }
}
