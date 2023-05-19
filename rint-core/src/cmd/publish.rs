use clap::Args;
use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    connection::Connection, protocol::Message, shutdown::Shutdown, storage::partition::Partition,
    Result,
};

use super::Command;

#[derive(Serialize, Debug, Args, Clone, Deserialize)]
pub struct PublishArg {
    /// 消费topic
    #[clap(long)]
    topic: String,

    /// 指定发布消息的分区
    partition: Option<u64>,

    /// 如果partition没有指定通过key的hash对分区数取余计算分区
    key: Option<String>,
}

impl PublishArg {
    pub fn new(topic: String, partition: Option<u64>, key: Option<String>) -> PublishArg {
        return PublishArg {
            topic,
            partition,
            key,
        };
    }

    pub async fn apply(self, conn: &mut Connection, context: &mut RintServerContext) -> Result<()> {
        let partition_num = Topic::get_partition_num(&self.topic);

        let partition = Partition::compute_partition(&self.partition, &self.key, partition_num);

        info!(
            "publish ->  topic:{}, partition:{}, ",
            self.topic, partition
        );

        let mut message = Message::new(0, format!("pong {}", msg).into_bytes());
        // Write the response back to the client
        conn.write_message(&mut message).await?;

        Ok(())
    }

    pub fn into_message(self) -> Message {
        let cmd = Command::Ping(self);
        Message::new(0, serde_json::to_vec(&cmd).unwrap())
    }
}
