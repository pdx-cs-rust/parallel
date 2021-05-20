// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

mod frandom;
mod stats;

use stats::stats;
use frandom::*;

use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver};

use lazy_static::lazy_static;
use rayon::prelude::*;

lazy_static! {
    static ref RNG: GlobalRng = GlobalRng::new(GlobalRng::STD_SEED);
}

/// We will work with blocks of data of this size.
const BLOCKSIZE: usize = 10 * 1024 * 1024;

/// Make a block of random floats.
fn make_rands() -> Vec<f64> {
    let mut rng = LocalRng::new(&RNG);
    (0..BLOCKSIZE).map(|_| rng.frandom()).collect()
}

/// Generate and do stats for `n` blocks sequentially.
fn sequential(n: usize) {
    for _ in 0..n {
        let rands = make_rands();
        println!("{:?}", stats(&rands));
    }
}

/// Generate and do stats for `n` blocks in `n` separate threads.
/// Communicate stats using `thread::join()`.
fn fork_join(n: usize) {
    let mut tids = Vec::new();
    for _ in 0..n {
        let tid = std::thread::spawn(|| {
            let rands = make_rands();
            stats(&rands)
        });
        tids.push(tid);
    }
    for tid in tids {
        println!("{:?}", tid.join().unwrap());
    }
}

/// Generate and do stats for a single block `n` times in `n` separate threads.  Share the block
/// readonly via an `Arc`. Communicate stats using `thread::join()`.
fn arc(n: usize) {
    let rands = Arc::new(make_rands());
    let mut tids = Vec::new();
    for _ in 0..n {
        let this_rands = rands.clone();
        let tid = std::thread::spawn(move || stats(&this_rands));
        tids.push(tid);
    }
    for tid in tids {
        println!("{:?}", tid.join().unwrap());
    }
}

/// Generate and save `n` blocks, then generate the stats for each block using `rayon` iterators.
fn rayon(n: usize) {
    let inits: Vec<()> = std::iter::repeat(()).take(n).collect();
    let blocks: Vec<Vec<f64>> = inits.par_iter().map(|()| make_rands()).collect();
    let results: Vec<(f64, f64)> = blocks.par_iter().map(|block| stats(block)).collect();
    for r in &results {
        println!("{:?}", *r);
    }
}

/// Generate and do stats for `n` blocks in `n` separate threads. Make the
/// threads communicate stat results via an `mpsc::channel`.
fn demo_channel(n: usize) {
    let mut tids = Vec::new();
    {
        // Need send to be dropped so that the channel will
        // be closed.
        let (send, receive) = channel();
        let tid = std::thread::spawn(move || {
            for v in receive {
                println!("{:?}", v);
            }
        });
        tids.push(tid);
        for _ in 0..n {
            let this_send = send.clone();
            let tid = std::thread::spawn(move || {
                let rands = make_rands();
                this_send.send(stats(&rands)).unwrap();
            });
            tids.push(tid);
        }
    }
    for tid in tids {
        tid.join().unwrap();
    }
}

/// Make a random block *b*. Then make `n` more random blocks and update
/// each element *e* of *b* with the maximum of *e* and the corresponding
/// element of the new block. Thus, at the end *b* will contain the
/// maximum of `n`+1 randomly-generated values.
fn sequential_pipeline(n: usize) {
    let mut rands = make_rands();
    for _ in 0..n {
        let more_rands = make_rands();
        for i in 0..rands.len() {
            rands[i] = rands[i].max(more_rands[i]);
        }
        println!("{:?}", stats(&rands));
    }
}

/// Make a random block *b*. Then move it through an `n`-stage pipeline. At each stage, make each
/// element *e* of *b* be the maximum of *e* and a new random value. After the final stage, *b*
/// will contain the maximum of `n`+1 randomly-generated values.
fn pipeline(n: usize) {
    let mut tids = Vec::new();
    let mut this_receive: Option<Receiver<_>> = None;
    for j in 0..n {
        let (send, next_receive) = channel();
        let tid = std::thread::spawn(move || {
            let mut rands = match this_receive {
                None => make_rands(),
                Some(receive) => receive.recv().unwrap(),
            };
            let more_rands = make_rands();
            for i in 0..rands.len() {
                rands[i] = rands[i].max(more_rands[i]);
            }
            println!("{:?}", stats(&rands));
            if j < n - 1 {
                send.send(rands).unwrap();
            }
        });
        tids.push(tid);
        this_receive = Some(next_receive);
    }
    for tid in tids {
        tid.join().unwrap();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = if args.len() == 3 {
        args[2].parse().unwrap()
    } else {
        10
    };
    match args[1].as_str() {
        "sequential" => sequential(n),
        "fork_join" => fork_join(n),
        "arc" => arc(n),
        "rayon" => rayon(n),
        "channel" => demo_channel(n),
        "sequential_pipeline" => sequential_pipeline(n),
        "pipeline" => pipeline(n),
        _ => panic!("unknown method"),
    }
}
