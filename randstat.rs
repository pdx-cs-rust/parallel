pub fn randstats(randblock: &[f64]) -> (f64, f64) {
    let mut mean = 0.0;
    for v in randblock {
        mean += *v;
    }
    let blocksize = randblock.len() as f64;
    mean /= blocksize;
    let mut var = 0.0;
    for v in randblock {
        let diff = *v - mean;
        var += diff * diff;
    }
    var /= blocksize * blocksize;
    (mean, var)
}
