//
// day10.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

use std::io;
use std::io::prelude::*;
use std::str::FromStr;


#[derive(Debug)]
struct CircleString
{
    list: Vec<u32>,
    pos: usize,
    skip: usize,
}

impl CircleString
{
    fn new(size: usize) -> Self
    {
        assert!(size <= u32::max_value() as usize);
        let mut cs = CircleString {
            list: Vec::with_capacity(size),
            pos: 0,
            skip: 0,
        };
        for value in 0..size {
            cs.list.push(value as u32);
        }
        cs
    }

    fn apply(&mut self, n: usize)
    {
        // 1. Reverse the order of elements pos..n
        let len = self.list.len();
        let mut i = self.pos;
        let mut j = self.pos + n - 1;
        while i < j {
            self.list.swap(i % len, j % len);
            i += 1;
            j -= 1;
        }

        // 2. Increase position by n+skip
        self.pos = (self.pos + n + self.skip) % self.list.len();

        // 3. Increase skip by one.
        self.skip += 1;
    }
}


fn main()
{
    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let mut cs = CircleString::new(256);
        for length in line.split(',').map(usize::from_str).filter_map(Result::ok) {
            cs.apply(length);
        }
        println!("{}", cs.list[0] * cs.list[1]);
    }
}
