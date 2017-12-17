//
// day14.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

extern crate aoc2017;

use aoc2017::day10::KnotHash;
use std::io;
use std::io::prelude::*;


fn make_row_hash(key: &str, row: u16) -> String {
    let input = format!("{}-{}", key, row);
    let mut kh = KnotHash::new();
    kh.rounds(input.as_bytes());
    format!("{:x}", kh)
}

fn hex_char_to_u8(c: char) -> u8 {
    match c {
        '0' => 0x0,
        '1' => 0x1,
        '2' => 0x2,
        '3' => 0x3,
        '4' => 0x4,
        '5' => 0x5,
        '6' => 0x6,
        '7' => 0x7,
        '8' => 0x8,
        '9' => 0x9,
        'a' | 'A' => 0xA,
        'b' | 'B' => 0xB,
        'c' | 'C' => 0xC,
        'd' | 'D' => 0xD,
        'e' | 'E' => 0xE,
        'f' | 'F' => 0xF,
        _ => panic!("Non-hex character: '{}'", c),
    }
}

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
