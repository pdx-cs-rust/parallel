extern crate rand;
use self::rand::random;

const BLOCKSIZE: usize = 10 * 1024 * 1024;

pub fn randstats() -> (f64, f64) {
    let randblock: Vec<f64> =
        (0..BLOCKSIZE).map(|_| random() ).collect();
    let mut mean = 0.0;
    for v in &randblock {
        mean += *v;
    }
    let blocksize = BLOCKSIZE as f64;
    mean /= blocksize;
    let mut var = 0.0;
    for v in &randblock {
        let diff = *v - mean;
        var += diff * diff;
    }
    var /= blocksize * blocksize;
    (mean, var)
}
