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

use std::sync::mpsc::{channel, Receiver};

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
    let inits: Vec<usize> = (0..n).collect();
    let results: Vec<(usize, (f64, f64))> = inits
        .par_iter()
        .map(|i| {
            let block = make_rands();
            (*i, randstats(&block))
        })
        .collect();
    for (i, r) in &results {
        println!("{} {:?}", *i, *r);
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

fn sequential_pipeline(n: usize) {
    let mut rands = make_rands();
    for _ in 0..n {
        let more_rands = make_rands();
        for i in 0..rands.len() {
            // Cannot use std::cmp::max() on floats.
            rands[i] = rands[i].max(more_rands[i]);
        }
        println!("{:?}", randstats(&rands));
    }
}

fn pipeline(n: usize) {
    let mut tids = Vec::new();
    let mut this_receive: Option<Receiver<_>> = None;
    for j in 0..n {
        let (send, next_receive) = channel();
        let tid = std::thread::spawn(move || {
            let mut rands =
                match this_receive {
                    None => make_rands(),
                    Some(receive) => receive.recv().unwrap(),
                };
            let more_rands = make_rands();
            for i in 0..rands.len() {
                rands[i] = rands[i].max(more_rands[i]);
            }
            println!("{:?}", randstats(&rands));
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
        8
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
