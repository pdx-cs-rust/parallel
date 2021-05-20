// Copyright © 2017 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.
    
//! Terrible LCG PRNG because Rust and global state.
//! http://nuclear.llnl.gov/CNP/rng/rngman/node4.html

use std::sync::atomic::{AtomicU64, Ordering::SeqCst};

pub struct GlobalRng(AtomicU64);

impl GlobalRng {
    pub const STD_SEED: u64 = 0x123456789abcdef0;

    /// Make a new global RNG.
    pub fn new(seed: u64) -> Self {
        Self(AtomicU64::new(seed))
    }

    /// Produce a pseudo-random integer. Will likely be slow in
    /// the presence of contention.
    pub fn random(&self) -> u64 {
        loop {
            let current = self.0.load(SeqCst);
            let new = current
                .wrapping_mul(2862933555777941757u64)
                .wrapping_add(3037000493u64);
            if self.0.compare_exchange(current, new, SeqCst, SeqCst).is_ok() {
                return new;
            }
        }
    }
}

#[test]
fn test_global_rng() {
    let rng = GlobalRng::new(GlobalRng::STD_SEED);
    let mut last = 0;
    for _ in 0..100 {
        let cur = rng.random();
        assert!(cur != last);
        last = cur;
    }
}

pub struct LocalRng(u64);

impl LocalRng {
    /// Produce a new local rng seeded from the global rng.
    pub fn new(global: &GlobalRng) -> Self {
        Self(global.random())
    }

    /// Produce a pseudo-random u64.
    pub fn random(&mut self) -> u64 {
        let old = self.0;
        self.0 = self.0
            .wrapping_mul(2862933555777941757u64)
            .wrapping_add(3037000493u64);
        old
    }

    /// Produce a pseudo-random floating point number in the range [0..1].
    pub fn frandom(&mut self) -> f64 {
        self.random() as f64 / (!0u64 as f64)
    }
}

#[test]
fn test_local_rng() {
    let rng = GlobalRng::new(GlobalRng::STD_SEED);
    let mut rng = LocalRng::new(&rng);
    let mut last = 0.0;
    for _ in 0..100 {
        let cur = rng.frandom();
        assert!(cur != last);
        last = cur;
    }
}
