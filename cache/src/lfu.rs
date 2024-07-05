use std::mem::MaybeUninit;

/// LFU节点
pub(crate) struct Node<K, V> {
    pub key: MaybeUninit<K>,
    pub value: MaybeUninit<V>,
    // 访问频次
    pub counter: usize,
    #[cfg(feature = "ttl")]
    pub expire: u64,
}

