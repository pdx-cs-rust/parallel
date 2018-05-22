mod randstat;
use randstat::randstats;

extern crate rand;
use self::rand::random;

const BLOCKSIZE: usize = 10 * 1024 * 1024;

fn make_rands() -> Vec<f64> {
    (0..BLOCKSIZE).map(|_| random() ).collect()
}

fn sequential(n: usize) {
    for _ in 0..n {
        let rands = make_rands();
        println!("{:?}", randstats(&rands));
    }
}

fn fork_join(n: usize) {
    let mut tids = Vec::new();
    for _ in 0..n {
        let tid = std::thread::spawn(|| {
            let rands = make_rands();
            randstats(&rands)
        });
        tids.push(tid);
    }
    for tid in tids {
        println!("{:?}", tid.join().unwrap());
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "sequential" => sequential(8),
        "fork_join" => fork_join(8),
        _ => panic!("unknown method"),
    }
}
