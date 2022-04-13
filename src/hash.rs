use std::hash::{BuildHasher, Hasher};

use metrohash::MetroHash64;

#[derive(Default)]
pub struct PassThroughHasher {
    value: u64,
}

impl Hasher for PassThroughHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!()
    }

    fn write_u64(&mut self, i: u64) {
        self.value = i;
    }
}

impl BuildHasher for PassThroughHasher {
    type Hasher = Self;

    fn build_hasher(&self) -> Self {
        Self::default()
    }
}

pub fn hash_of<T>(t: &T) -> u64
where
    T: std::hash::Hash,
{
    let mut hasher = MetroHash64::default();
    t.hash(&mut hasher);
    hasher.finish()
}
