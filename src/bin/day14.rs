//
// day14.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

extern crate aoc2017;

use aoc2017::day14::{ make_row_hash, hex_char_to_u8 };
use std::io;
use std::io::prelude::*;


fn hex_string_bits(s: &str) -> u32 {
    let mut bits = 0;
    for c in s.chars() {
        let mut num = hex_char_to_u8(c);
        while num != 0 {
            if num & 0x1 == 0x1 {
                bits += 1;
            }
            num >>= 1;
        }
    }
    bits
}


fn main()
{
    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let input_key = line.trim();
        let mut used_bits = 0;
        for row in 0 .. 128 {
            let row_hash = make_row_hash(input_key, row);
            used_bits += hex_string_bits(&row_hash);
        }
        println!("Used cells: {}", used_bits);
    }
}
