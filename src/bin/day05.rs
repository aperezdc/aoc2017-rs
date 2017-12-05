//
// day05.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

use std::io;
use std::io::prelude::*;
use std::str::FromStr;


fn main()
{
    let stdin = io::stdin();
    let mut jumplist: Vec<i32> = stdin.lock()
        .lines()
        .filter_map(io::Result::ok)
        .map(|line| i32::from_str(&line))
        .filter_map(Result::ok)
        .collect();

    let mut pc: i32 = 0;
    let mut steps = 0;

    while pc >= 0 && (pc as usize) < jumplist.len() {
        steps += 1;
        let jump = jumplist[pc as usize];
        jumplist[pc as usize] += 1;  // Increment jump.
        pc += jump;                  // Apply jump.
    }

    println!("steps: {}", steps);
}
