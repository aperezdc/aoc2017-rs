//
// day01b.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

extern crate aoc2017;

use std::io::{ self, Read };
use aoc2017::iter_digits;


fn main()
{
    let digits: Vec<_> = iter_digits(io::stdin().bytes()).collect();

    let mut sum = 0;
    let step = digits.len() / 2;

    for (index, value) in digits.iter().enumerate() {
        if *value == digits[(index + step) % digits.len()] {
            sum += *value;
        }
    }

    println!("{}", sum);
}
