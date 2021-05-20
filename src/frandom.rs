// Copyright © 2017 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.
    
//! Terrible LCG PRNG because Rust and global state.
//! http://nuclear.llnl.gov/CNP/rng/rngman/node4.html

use std::sync::atomic::{AtomicU64, Ordering::SeqCst};

pub struct GlobalRng(AtomicU64);

pub struct LocalRng(u64);

fn next_rand(n: u64) -> u64 {
    n
        .wrapping_mul(2862933555777941757u64)
        .wrapping_add(3037000493u64)
}

impl GlobalRng {
    pub const STD_SEED: u64 = 0x123456789abcdef0;

    /// Make a new global rng with the given seed.
    pub const fn from_seed(seed: u64) -> Self {
        Self(AtomicU64::new(seed))
    }

    /// Make a new global rng.
    pub const fn new() -> Self {
        Self::from_seed(Self::STD_SEED)
    }

    /// Make a new local rng seeded from this global rng.
    pub fn local_rng(&self) -> LocalRng {
        LocalRng::from_seed(self.random() ^ self.random())
    }

    /// Produce a pseudo-random integer. Will likely be slow in
    /// the presence of contention.
    pub fn random(&self) -> u64 {
        loop {
            let current = self.0.load(SeqCst);
            let new = next_rand(current);
            if self.0.compare_exchange(current, new, SeqCst, SeqCst).is_ok() {
                return new;
            }
        }
    }
}

impl Default for GlobalRng {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_global_rng() {
    let rng = GlobalRng::new();
    let mut last = 0;
    for _ in 0..100 {
        let cur = rng.random();
        assert!(cur != last);
        last = cur;
    }
}

impl LocalRng {
    /// Produce a new local rng with the given seed.
    pub fn from_seed(seed: u64) -> Self {
        Self(seed)
    }

    /// Produce a pseudo-random u64.
    pub fn random(&mut self) -> u64 {
        let old = self.0;
        self.0 = next_rand(old);
        old
    }

    /// Produce a pseudo-random floating point number in the range [0..1].
    pub fn frandom(&mut self) -> f64 {
        self.random() as f64 / (!0u64 as f64)
    }
}

#[test]
fn test_local_rng() {
    let rng = GlobalRng::new();
    let mut rng = rng.local_rng();
    let mut last = 0.0;
    for _ in 0..100 {
        let cur = rng.frandom();
        assert!(cur != last);
        last = cur;
    }
}
