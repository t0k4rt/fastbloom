use std::ops::Index;

use bit_vec::BitVec;
use fastmurmur3::murmur3_x64_128;
use smallvec::SmallVec;

use crate::builder::FilterBuilder;

#[inline]
fn bit_set(bit_set: &mut BitVec, value: &[u8], m: u128, k: u64) {
    // let len = m >> 5;
    let hash1 = (murmur3_x64_128(value, 0) % m) as u64;
    let hash2 = (murmur3_x64_128(value, 32) % m) as u64;

    let m = m as u64;
    for i in 1..k {
        let mo = ((hash1 + i * hash2) % m) as usize;
        bit_set.set(mo, true);
    };
    bit_set.set(hash1 as usize, true);
}

#[inline]
fn bit_check(bit_set: &BitVec, value: &[u8], m: u128, k: u64) -> bool {
    let hash1 = (murmur3_x64_128(value, 0) % m) as u64;
    let hash2 = (murmur3_x64_128(value, 32) % m) as u64;
    let mut res = *bit_set.index(hash1 as usize);
    for i in 1..k {
        if !res { return false; }
        let mo = ((hash1 + i * hash2) % m as u64) as usize;
        res = res && *bit_set.index(mo);
    }
    res
}

#[derive(Clone)]
#[derive(Debug)]
pub struct BloomFilter {
    config: FilterBuilder,
    bit_set: BitVec,
}

impl BloomFilter {
    /// Build a Bloom filter form [FilterBuilder].
    ///
    /// # Examples:
    ///
    /// ```
    /// use fastbloom_rs::{BloomFilter, FilterBuilder};
    ///
    /// let builder = FilterBuilder::new(100_000_000, 0.01);
    /// let bloom = BloomFilter::new(builder);
    /// ```
    pub fn new(mut config: FilterBuilder) -> Self {
        config.complete();
        let bit_set = BitVec::from_elem(config.size as usize, false);
        BloomFilter { config, bit_set }
    }

    pub fn from_bit_vec(bit_vec: &BitVec, hashes: u32) -> Self {
        let mut config = FilterBuilder::from_size_and_hashes(bit_vec.len() as u64, hashes);
        config.complete();
        BloomFilter { config, bit_set: bit_vec.clone() }
    }

    pub fn from_u8_array(array: &[u8], hashes: u32) -> Self {
        let mut config = FilterBuilder::from_size_and_hashes((array.len() * 8) as u64, hashes);
        config.complete();
        BloomFilter { config, bit_set: BitVec::from_bytes(array) }
    }


    /// Returns the configuration/builder of the Bloom filter.
    /// # Examples
    ///
    /// ```
    /// use fastbloom_rs::{BloomFilter, FilterBuilder};
    ///
    /// let bloom = FilterBuilder::new(100_000_000, 0.01).build_bloom_filter();
    /// let builder = bloom.config();
    /// ```
    ///
    pub fn config(&self) -> FilterBuilder {
        self.config.clone()
    }

    pub fn hashes(&self) -> u32 {
        self.config.hashes
    }

    pub fn add(&mut self, element: &[u8]) {
        bit_set(&mut self.bit_set, element, self.config.size as u128, self.config.hashes as u64);
    }

    pub fn clear(&mut self) {
        self.bit_set.clear();
    }

    pub fn contains(&self, element: &[u8]) -> bool {
        bit_check(&self.bit_set, element, self.config.size as u128, self.config.hashes as u64)
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        self.bit_set.get(index)
    }

    pub fn set(&mut self, index: usize, to: bool) {
        self.bit_set.set(index, to)
    }

    pub fn get_bit_vec(&self) -> BitVec {
        self.bit_set.clone()
    }

    pub fn get_u8_array(&self) -> Vec<u8> {
        self.bit_set.to_bytes()
    }

    #[cfg(feature = "use_alloc")]
    pub fn get_u32_array(&self) -> Vec<u32> {
        self.bit_set.blocks().collect_vec()
    }

    /// Performs the union operation on two compatible bloom filters. This is achieved through a bitwise OR operation on
    /// their bit vectors. This operations is lossless, i.e. no elements are lost and the bloom filter is the same that
    /// would have resulted if all elements wer directly inserted in just one bloom filter.
    pub fn union(&mut self, other: &BloomFilter) -> bool {
        if self.compatible(other) {
            self.bit_set.or(&other.bit_set);
            true
        } else { false }
    }

    pub fn intersect(&mut self, other: &BloomFilter) -> bool {
        if self.compatible(other) {
            self.bit_set.and(&other.bit_set);
            true
        } else { false }
    }

    pub fn is_empty(&self) -> bool {
        self.bit_set.is_empty()
    }

    pub fn set_bit_vec(&mut self, bit_vec: BitVec) {
        assert_eq!(self.config.size, bit_vec.capacity() as u64);
        self.bit_set = bit_vec
    }

    fn compatible(&self, other: &BloomFilter) -> bool {
        self.config.is_compatible_to(&other.config)
    }
}

#[test]
fn shift_test() {
    assert_eq!(32 >> 5, 1);
}