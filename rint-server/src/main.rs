use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use rint_core::config::Config;
use rint_core::Result;
use rint_server::server;
use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::atomic::AtomicU8;
use tokio::net::TcpListener;
use tokio::signal;

fn init_log() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.filter_level(LevelFilter::Info);
    builder.init();
}

///
/// 0-> not started
/// 1-> starting
/// 2->
static mut STATUS: AtomicU8 = AtomicU8::new(0);

pub enum ServerStatus {
    NotStarted,
    Starting,
    Running,
    Stopping,
    Stopped,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    init_log();

    // 获取执行的命令的第一个参数
    let path = env::args()
        .nth(1)
        .or_else(|| Some("rint.toml".to_string()))
        .unwrap();
    // 如果没有指定路径，则使用默认参数

    let config = Config::parse_from(&path);

    let listener = TcpListener::bind((
        IpAddr::from_str(std::str::from_utf8(&config.ip.as_bytes())?)?,
        config.port,
    ))
    .await?;
    info!("rint server is starting at {}:{}", config.ip, config.port);

    server::run(listener, signal::ctrl_c()).await;

    Ok(())
}
