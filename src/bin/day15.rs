//
// day15.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

#![feature(slice_patterns)]

use std::io;
use std::io::prelude::*;
use std::str::FromStr;


struct Generator
{
    last: u64,
    factor: u64,
}


impl Generator
{
    const DIVIDER: u64 = 2147483647;
    const FACTORA: u64 = 16807;
    const FACTORB: u64 = 48271;

    fn new(start: u64, factor: u64) -> Self {
        Self { last: start, factor: factor }
    }

    fn next(&mut self) -> u64 {
        self.last = (self.last * self.factor) % Self::DIVIDER;
        self.last
    }
}


const NPAIRS: usize = 40 * 1000 * 1000;


fn main()
{
    let stdin = io::stdin();

    for line in stdin.lock().lines().filter_map(Result::ok) {
        let values: Vec<_> = line.split_whitespace()
            .map(u64::from_str).filter_map(Result::ok).collect();
        assert_eq!(values.len(), 2);

        let mut a = Generator::new(values[0], Generator::FACTORA);
        let mut b = Generator::new(values[1], Generator::FACTORB);

        let mut matches = 0;
        for _ in 0 .. NPAIRS {
            if a.next() & 0xFFFF == b.next() & 0xFFFF {
                matches += 1;
            }
        }

        println!("{} matches", matches);
    }
}
