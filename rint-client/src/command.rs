use std::error::Error;

use clap::{Parser, Subcommand};
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
        #[arg(value_parser = bytes_from_str)]
        msg: Option<Vec<u8>>,
    },
}

fn bytes_from_str(src: &str) -> Result<Vec<u8>, std::io::Error> {
    Ok(src.as_bytes().to_vec())
}
