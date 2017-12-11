//
// day09.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

#![feature(io)]

use std::io;
use std::io::prelude::*;


fn main()
{
    let mut in_comment = false;
    let mut skip_next = false;
    let mut ngroups = 0;
    let mut nchars = 0;
    let mut score = 0;
    let mut score_stack = Vec::new();
    score_stack.push(0);

    let stdin = io::stdin();
    for ch in stdin.lock().chars().filter_map(Result::ok) {
        if skip_next {
            skip_next = false;
            continue;
        }
        match ch {
            '{' if !in_comment => {
                ngroups += 1;
                // Score for a group is "one more than the score of the
                // group that contains it".
                let current_score = score_stack.last().unwrap() + 1;
                score_stack.push(current_score);
            },
            '}' if !in_comment => {
                score += score_stack.pop().unwrap();
            },
            '!' => skip_next = true,
            '<' if !in_comment => in_comment = true,
            '>' if in_comment => in_comment = false,
            _ if in_comment => nchars += 1,
            _ => (),  // Do nothing.
        };
    }
    println!("Groups: {}", ngroups);
    println!(" Score: {}", score);
    println!("NChars: {}", nchars);
}
