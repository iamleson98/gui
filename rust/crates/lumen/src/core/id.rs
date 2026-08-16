use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
#[repr(transparent)]
pub struct Id(u64);
impl Id {
    pub const NULL: Self = Self(0);
    pub fn new(s: &str) -> Self {
        use std::hash::{Hash, Hasher};
        let mut h = ahash::AHasher::default();
        s.hash(&mut h);
        Id(h.finish() | 1)
    }
    pub fn derive(self, salt: &str) -> Self {
        use std::hash::{Hash, Hasher};
        let mut h = ahash::AHasher::default();
        self.0.hash(&mut h);
        salt.hash(&mut h);
        Id(h.finish())
    }
    pub fn derive_index(self, idx: usize) -> Self {
        use std::hash::{Hash, Hasher};
        let mut h = ahash::AHasher::default();
        self.0.hash(&mut h);
        idx.hash(&mut h);
        Id(h.finish())
    }
    pub fn unique() -> Self {
        static C: AtomicU64 = AtomicU64::new(1 << 63);
        Id(C.fetch_add(1, Ordering::Relaxed))
    }
}
impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({:#018x})", self.0)
    }
}
impl PartialEq for Id {
    fn eq(&self, o: &Self) -> bool {
        self.0 == o.0
    }
}
impl Eq for Id {}
impl std::hash::Hash for Id {
    fn hash<H: std::hash::Hasher>(&self, s: &mut H) {
        self.0.hash(s);
    }
}
impl Clone for Id {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for Id {}
