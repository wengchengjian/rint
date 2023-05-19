pub mod client;
pub mod command;

use std::{
    fmt::format,
    io::Read,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    todo,
};

use crate::command::Cli;
use clap::Parser;
use client::Client;
use command::Commands;
use env_logger::{Builder, Target};
use log::debug;
use log::{error, info, warn, LevelFilter};
use rint_core::Result;
use tokio::io::AsyncWriteExt;

fn init_log() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.filter_level(LevelFilter::Info);
    builder.init();
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // 初始化日志
    init_log();

    let cli = Cli::parse();

    let host = cli.host;

    let port = cli.port;

    let addr = format!("{}:{}", host, port);

    let mut client = Client::connect(addr).await?;

    match cli.command.or(Some(Commands::Ping {
        msg: "Hello".into(),
    })) {
        Some(Commands::Ping { msg }) => client.ping(msg).await?,
        Some(Commands::Shutdown) => client.shutdown().await?,
        Some(Commands::Info { key }) => client.info(key).await?,
        Some(Commands::Subscribe(arg)) => client.subscribe(arg).await?,
        Some(Commands::Publish(arg)) => client.publish(arg).await?,
        None => (),
    };
    Ok(())
}
