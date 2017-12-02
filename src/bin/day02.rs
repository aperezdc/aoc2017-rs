//
// day02.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

use std::io;
use std::io::prelude::*;
use std::str::FromStr;


fn main()
{
    let mut checksum = 0;

    let stdin = io::stdin();
    for row in stdin.lock().lines().filter_map(io::Result::ok) {
        let mut rowmin = i32::max_value();
        let mut rowmax = i32::min_value();
        for value in row.split_whitespace().map(|s| i32::from_str(s).unwrap()) {
            if value < rowmin {
                rowmin = value;
            }
            if value > rowmax {
                rowmax = value;
            }
        }
        checksum += rowmax - rowmin;
    }

    println!("{}", checksum);
}
