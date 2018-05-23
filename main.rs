// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

mod randstat;
use randstat::randstats;

extern crate rand;
use self::rand::random;

use std::sync::Arc;

extern crate rayon;
use self::rayon::prelude::*;

use std::sync::mpsc::channel;

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

fn arc(n: usize) {
    let rands = Arc::new(make_rands());
    let mut tids = Vec::new();
    for _ in 0..n {
        let this_rands = rands.clone();
        let tid = std::thread::spawn(move || {
            randstats(&this_rands)
        });
        tids.push(tid);
    }
    for tid in tids {
        println!("{:?}", tid.join().unwrap());
    }
}

fn rayon(n: usize) {
    let inits: Vec<()> = (0..n).map(|_| ()).collect();
    let blocks: Vec<Vec<f64>> = inits
        .par_iter()
        .map(|()| {
            make_rands()
        })
        .collect();
    let results: Vec<(f64, f64)> = blocks
        .par_iter()
        .map(|block| randstats(block))
        .collect();
    for r in &results {
        println!("{:?}", *r);
    }
}

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
                this_send.send(randstats(&rands)).unwrap();
            });
            tids.push(tid);
        }
    }
    for tid in tids {
        tid.join().unwrap();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "sequential" => sequential(8),
        "fork_join" => fork_join(8),
        "arc" => arc(8),
        "rayon" => rayon(8),
        "channel" => demo_channel(8),
        _ => panic!("unknown method"),
    }
}
