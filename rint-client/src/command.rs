use std::error::Error;

use clap::{Args, FromArgMatches, Parser, Subcommand};
use rint_core::DEFAULT_PORT;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(author = "wengchengjian", name= "Rint-Cli", version = "1.0.0", about = "rint-rust客户端", long_about = None)]
#[command(next_line_help = true)]
pub struct Cli {
    #[clap(name = "hostname", long, default_value = "127.0.0.1")]
    pub host: String,

    #[clap(short, long, default_value_t = DEFAULT_PORT)]
    pub port: u16,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}
#[derive(Subcommand, Serialize, Debug, Clone, Deserialize)]
pub enum Commands {
    #[clap(about = "获取rint服务器信息")]
    Info {
        // 查询指定关键字信息
        #[clap(short, long)]
        key: Option<String>,
    },

    #[clap(about = "获取rint服务器版本")]
    Shutdown,

    #[clap(about = "ping")]
    Ping {
        #[clap(default_value = "Hello")]
        msg: String,
    },

    #[clap(about = "subscribe")]
    Subscribe(SubscribeArg),
    #[clap(about = "publish")]
    Publish(PublishArg),
}
#[derive(Serialize, Debug, Args, Clone, Deserialize)]
pub struct SubscribeArg {
    /// 消费topic
    #[clap(long)]
    topic: String,

    /// 指定消费者组
    #[clap(long)]
    group: Option<String>,

    /// 是否重头消费
    #[clap(long)]
    begging_offset: bool,

    // 指定消费偏移量
    #[clap(long)]
    assign_offset: Option<u64>,
}
