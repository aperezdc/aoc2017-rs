//
// day06.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;


struct Memory
{
    banks: Vec<u32>,
}

impl Memory
{
    fn new(banks: Vec<u32>) -> Self {
        Self { banks: banks }
    }

    fn find_most_blocks(&self) -> (usize, u32) {
        let mut max_blocks = 0;
        let mut max_blocks_index = 0;
        for (index, blocks) in self.banks.iter().enumerate() {
            if *blocks > max_blocks {
                max_blocks_index = index;
                max_blocks = *blocks;
            }
        }
        (max_blocks_index, max_blocks)
    }

    fn reallocate(&mut self) {
        let (mut index, mut nblocks) = self.find_most_blocks();

        // 1. Empty the bank
        self.banks[index] = 0;

        // 2. Redistribute among banks, starting with the next
        while nblocks > 0 {
            index = (index + 1) % self.banks.len();  // Move to next bank index
            self.banks[index] += 1;                  // Add one block to that bank
            nblocks -= 1;                            // One less block pending!
        }
    }

    fn id(&self) -> String {
        self.banks.iter().fold(None, |acc, &item| {
            if let Some(s) = acc {
                Some(format!("{},{}", s, item))
            } else {
                Some(format!("{}", item))
            }
        }).unwrap()
    }
}

impl fmt::Display for Memory
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mem<{}>{:?}", self.banks.len(), self.banks)
    }
}


fn main()
{
    let stdin = io::stdin();
    for row in stdin.lock().lines().filter_map(io::Result::ok) {
        let mut mem = Memory::new(row.split_whitespace()
                                     .map(u32::from_str)
                                     .filter_map(Result::ok)
                                     .collect());
        let mut states = HashMap::new();
        // states.insert(mem.id(), 0);

        for step in 1.. {
            mem.reallocate();
            let id = mem.id();
            if let Some(prev_step) = states.get(&id) {
                println!("steps: {}, cycle: {}, state: {}",
                         step, step - prev_step, mem);
                break;
            }
            states.insert(id, step);
        }
    }
}
