//
// day04b.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

extern crate permutohedron;

use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use permutohedron::Heap;


fn main()
{
    let stdin = io::stdin();
    let mut valid = 0;
    let mut total = 0;
    for line in stdin.lock().lines().filter_map(io::Result::ok) {
        let mut seen_words = HashSet::new();
        let mut is_valid = true;
        'outer: for word in line.split_whitespace() {
            let mut word_chars: Vec<char> = word.chars().collect();
            let heap = Heap::new(&mut word_chars);
            for permutated_chars in heap {
                let permutation: String = permutated_chars.iter().collect();
                if seen_words.contains(permutation.as_str()) {
                    is_valid = false;
                    break 'outer;
                }
            }
            seen_words.insert(word);
        }
        if is_valid {
            valid += 1;
        }
        total += 1;
    }
    println!("{}/{} valid", valid, total);
}
