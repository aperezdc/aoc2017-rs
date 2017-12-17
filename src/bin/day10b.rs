//
// day10b.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

extern crate aoc2017;

use aoc2017::day10::KnotHash;
use std::io;
use std::io::prelude::*;


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
