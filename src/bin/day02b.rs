//
// day02b.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

extern crate aoc2017;

use aoc2017::{ rows_of_digits, Permutations };
use std::io;


fn main()
{
    let stdin = io::stdin();
    let mut sum = 0;
    for row in rows_of_digits(stdin.lock()) {
        for (a, b) in row.permutations() {
            if a != b && a % b == 0 {
                sum += a / b;
                break;
            }
        }
    }
    println!("{}", sum);
}
