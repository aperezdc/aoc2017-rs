//
// day01.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

extern crate aoc2017;

use std::io::{ self, Read };
use aoc2017::iter_digits;


fn main()
{
    let mut digits = iter_digits(io::stdin().bytes());
    let first = digits.next().unwrap();
    let mut last = first;
    let mut sum = 0;

    loop {
        match digits.next() {
            None => break,
            Some(digit) => {
                if digit == last {
                    sum += last;
                }
                last = digit;
            },
        }
    }

    // Wrap around.
    if first == last {
        sum += last;
    }

    println!("{}", sum);
}
