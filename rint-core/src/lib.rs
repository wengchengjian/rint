pub mod cmd;
pub mod config;
pub mod connection;
pub mod encrypt;
pub mod error;
pub mod parse;
pub mod partition;
pub mod protocol;
pub mod replica;
pub mod shutdown;
pub mod topic;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;

/// 默认端口 如果没有指定的话
pub const DEFAULT_PORT: u16 = 8796;

pub const MAX_CONNECTIONS: u16 = 256;
