// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

/// Return the mean and variance of the given block.
pub fn stats(block: &[f64]) -> (f64, f64) {
    assert!(!block.is_empty());
    let mut mean = 0.0;
    for v in block {
        mean += *v;
    }
    let blocksize = block.len() as f64;
    mean /= blocksize;
    let mut var = 0.0;
    for v in block {
        let diff = *v - mean;
        var += diff * diff;
    }
    var /= blocksize;
    (mean, var)
}
