use std::mem::MaybeUninit;

/// LFU节点对象
pub(crate) struct Node<K, V> {
    pub key: MaybeUninit<K>,
    pub value: MaybeUninit<V>,

    /// 访问频次
    pub counter: usize,

    /// 过期时间（单位s）
    ///
    /// 如果为[u64::MAX]，则表示不过期
    #[cfg(feature = "ttl")]
    pub expire: u64,
}

impl<K, V> Node<K, V> {
    #[inline]
    pub fn new(k: K, v: V, counter: usize) -> Self {
        Self {
            key: MaybeUninit::new(k),
            value: MaybeUninit::new(v),
            counter,
            #[cfg(feature = "ttl")]
            expire: u64::MAX,
        }
    }

    #[inline]
    #[cfg(feature = "ttl")]
    pub fn is_expire(&self) -> bool {

    }
}

