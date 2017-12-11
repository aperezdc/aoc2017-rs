//
// day10b.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

use std::fmt;
use std::io;
use std::io::prelude::*;

struct KnotHash 
{
    list: [u8; 256],
    pos: usize,
    skip: usize,
}

impl KnotHash
{
    const ROUNDS: usize = 64;
    const XORITEMS: usize = 16;
    const XORGROUPS: usize = 256 / Self::XORITEMS;

    fn new() -> Self {
        let mut kh = KnotHash { list: [0u8; 256], pos: 0, skip: 0 };
        for i in 0 .. kh.list.len() {
            kh.list[i] = i as u8;
        }
        kh
    }

    fn apply(&mut self, n: u8) {
        // 1. Reverse the order of elements pos..n
        let len = self.list.len();
        let mut i = self.pos;
        let mut j = self.pos + (n as usize) - 1;
        while i < j {
            self.list.swap(i % len, j % len);
            i += 1;
            j -= 1;
        }

        // 2. Increase position by n+skip
        self.pos = (self.pos + (n as usize) + self.skip) % self.list.len();

        // 3. Increase skip by one.
        self.skip += 1;
    }

    #[inline]
    fn rounds(&mut self, input: &[u8]) {
        for _ in 0..Self::ROUNDS {
            for &b in input {
                self.apply(b);
            }
            // Apply the standard tail lengths.
            for &b in &[17, 31, 73, 47, 23] {
                self.apply(b);
            }
        }
    }
}

impl fmt::LowerHex for KnotHash
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for group in 0 .. Self::XORGROUPS {
            let startpos = group * Self::XORITEMS;
            let mut xor = 0;
            for x in &self.list[startpos .. startpos + Self::XORITEMS] {
                xor ^= x;
            }
            write!(f, "{:02x}", xor)?;
        }
        Ok(())
    }
}


fn trim<'a>(s: &'a [u8]) -> &'a [u8] {
    let mut l = 0;
    while l < s.len() && s[l].is_ascii_whitespace() {
        l += 1;
    }
    let mut r = s.len();
    while r > l && s[r - 1].is_ascii_whitespace() {
        r -= 1;
    }
    return &s[l..r];
}


fn main()
{
    let mut input = Vec::new();
    let stdin = io::stdin();
    stdin.lock().read_to_end(&mut input).unwrap();
    let mut kh = KnotHash::new();
    kh.rounds(trim(&input));
    println!("{:x}", kh);
}
