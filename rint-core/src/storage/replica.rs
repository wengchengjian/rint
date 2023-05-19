pub enum ReplicaStateChange {
    /// 创建 topic 和分区分配后创建 replicas，此时，replica 只能获取到成为 follower 状态变化请求。
    NewReplica,
    /// 当replica 成为 parition 的 assingned replicas 时，其状态变为 OnlineReplica, 即一个有效的 OnlineReplica。
    OnlineReplica,
    /// 当一个 replica 下线，进入此状态，这一般发生在 broker 宕机的情况下；
    OfflineReplica,
    /// Replica 成功删除后，replica 进入 NonExistentReplica 状态。
    NonExistentReplica,
}
