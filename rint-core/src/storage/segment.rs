use tokio::fs::File;

/// 默认segment大小, 超过这个大小会创建新的segment文件
pub static DEFAULT_SEGMENT_SIZE: u64 = 107370;

/// 默认segment文件过期时间, 过期后自动清理
pub static DEFAULT_EXPIRE_HOURS: u64 = 7 * 24;

/// 默认稀疏索引间隔
pub static DEFAULT_SPARSE_INTERVAL: u64 = 50;

pub struct Segment {
    /// segment所属topic
    pub topic: String,
    /// segment所在分区
    pub partition: String,
    /// segment数据
    pub data: SegmentLog,
    /// segment索引
    pub index: OffsetIndex,
    /// segment时间索引
    pub time_index: TimeIndex,
    /// segment配置
    pub conf: SegmentConf,
}

pub struct SegmentConf {
    /// 最大存储容量
    pub max_size: u64,

    /// 过期时间戳
    pub expire: u64,
}

pub struct SegmentLog {
    /// log文件名称
    pub name: String,

    /// log文件大小
    pub size: u64,

    /// segment最小offset
    pub min_offset: u64,

    /// segment最大offset
    pub max_offset: u64,
}

pub struct OffsetIndex {}
pub struct TimeIndex {}

pub struct RintMessageLog {
    pub offset: u64,

    pub position: u64,

    pub timestamp: u64,

    pub content: Vec<u8>,
}

pub struct RintMessageIndex {
    pub offset: u64,

    pub position: u64,
}
