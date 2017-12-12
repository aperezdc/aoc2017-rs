//
// day12.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

#[macro_use]
extern crate failure;

use std::collections::{ HashMap, HashSet };
use std::io;
use std::io::prelude::*;
use std::str::FromStr;


#[derive(Debug)]
struct Pipe
{
    from: u32,
    to: Vec<u32>,
}

impl FromStr for Pipe
{
    type Err = ::failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if let Some(pos) = s.find("<->") {
            let (from_str, to_str) = s.split_at(pos);
            let mut to = Vec::new();
            for num_str in to_str[3..].split(',').map(str::trim) {
                to.push(num_str.parse()?);
            }
            Ok(Pipe { from: from_str.trim().parse()?, to: to })
        } else {
            bail!("Invalid pipe specification: '{}'", s)
        }
    }
}


fn add_connected_pipes(pipe_id: u32,
                       pipes: &mut HashMap<u32, Pipe>,
                       connected_pipes: &mut HashSet<u32>)
{
    // No more pipes to add.
    if pipes.is_empty() {
        return;
    }

    let pipe = if let Some(p) = pipes.remove(&pipe_id) { p } else {
        panic!("No pipe with id={} found", pipe_id);
    };

    connected_pipes.insert(pipe.from);
    for pipe_id in pipe.to {
        if !connected_pipes.contains(&pipe_id) {
            add_connected_pipes(pipe_id, pipes, connected_pipes);
        }
    }
}


fn main()
{
    let mut pipes = HashMap::new();

    // Read all pipes.
    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let p: Pipe = line.parse().unwrap();
        pipes.insert(p.from, p);
    }

    let mut connected_pipes = HashSet::new();
    add_connected_pipes(0, &mut pipes, &mut connected_pipes);
    
    println!("Connected to 0: {}, disconnected: {}",
             connected_pipes.len(), pipes.len());

    let mut ngroups = 1;  // Count the group connected to 0.
    while !pipes.is_empty() {
        connected_pipes.clear();  // Start with an empty set.
        let &pipe_id = pipes.keys().next().unwrap();
        add_connected_pipes(pipe_id, &mut pipes, &mut connected_pipes);
        ngroups += 1;
    }

    println!("Total groups: {}", ngroups);
}
