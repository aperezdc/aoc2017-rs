//
// day04.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

use std::collections::HashSet;
use std::io;
use std::io::prelude::*;


fn main()
{
    let stdin = io::stdin();
    let mut valid = 0;
    let mut total = 0;
    for line in stdin.lock().lines().filter_map(io::Result::ok) {
        let mut seen_words = HashSet::new();
        let mut is_valid = true;
        for word in line.split_whitespace() {
            if seen_words.contains(word) {
                is_valid = false;
                break;
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
