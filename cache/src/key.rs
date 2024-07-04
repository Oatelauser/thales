use std::hash::Hash;

#[repr(transparent)]
pub(crate) struct KeyRef<K>(*const K);

impl<K> KeyRef<K> {
    pub fn new(k: *const K) -> Self {
        Self(k)
    }
}

impl<K> Clone for KeyRef<K> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<K: Hash> Hash for KeyRef<K> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe {
            (*self.0).hash(state);
        }
    }
}

impl<K: PartialEq> PartialEq for KeyRef<K> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { (*self.0).eq(&*other.0) }
    }
}

impl<K: Eq> Eq for KeyRef<K> {}

impl<K: Eq> PartialOrd for KeyRef<K> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
