pub enum PartitionStateChange {
    /// 该状态表示分区没有被创建过或创建后被删除了。
    NonExistentPartition,
    /// 分区刚创建后，处于这个状态。此状态下分区已经分配了副本，但是还没有选举 leader，也没有 ISR 列表。
    NewPartition,
    /// 一旦这个分区的 leader 被选举出来，将处于这个状态。
    OnlinePartition,
    /// 当分区的 leader 宕机，转移到这个状态。
    OfflinePartition,
}

pub struct Partition {
    pub num: u64,

    pub max_offset: u64,
}

impl Partition {
    pub fn compute_partition(partition: Option<u64>, key: Option<String>, num: u64) -> u64 {
        if let Some(partition) = partition {
            return partition;
        }
    }
}
