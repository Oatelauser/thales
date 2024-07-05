use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::mem;

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
    fn hash<H: Hasher>(&self, state: &mut H) {
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

#[repr(transparent)]
pub(crate) struct KeyWrapper<Q: ?Sized>(Q);

impl<Q: ?Sized> KeyWrapper<Q> {
    pub fn new(key: &Q) -> &Self {
        unsafe { mem::transmute(key) }
    }
}

impl<Q: ?Sized + Hash> Hash for KeyWrapper<Q> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<Q: ?Sized + PartialEq> PartialEq for KeyWrapper<Q> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<Q: ?Sized + Eq> Eq for KeyWrapper<Q> {}

impl<K, Q> Borrow<K> for KeyWrapper<Q>
where
    Q: ?Sized,
    K: Borrow<Q>,
{
    fn borrow(&self) -> &K {
        self.0.borrow()
    }
}
