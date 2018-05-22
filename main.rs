mod randstat;
use randstat::randstats;

extern crate rand;
use self::rand::random;

const BLOCKSIZE: usize = 10 * 1024 * 1024;

fn make_rands(n: usize) -> Vec<Vec<f64>> {
    let mut result = Vec::new();
    for _ in 0..n {
        let block: Vec<f64> =
            (0..BLOCKSIZE).map(|_| random() ).collect();
        result.push(block);
    }
    result
}

fn sequential(n: usize) {
    let rands = make_rands(n);
    for i in 0..n {
        println!("{:?}", randstats(&rands[i]));
    }
}

fn main() {
    sequential(8);
}
