//
// day03.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

extern crate aoc2017;

use aoc2017::day03::grid_size_for_cell;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;


// For a memory grid, the center point is always "1", and the bottom-right
// corner is "size * size = maxindex":
//
//   17  16  15  14  13
//   18   5   4   3  12
//   19   6  [1]  2  11
//   20   7   8  [9] 10
//   21  22  23  24 [25]  = size * size
//
// The position we are searching for is always in the outer ring. We can
// calculate the position by walking "backwards" in the spiral: The number
// of steps is (size * size - cellindex).
//
fn manhattan_distance_for_cell(cellindex: u32) -> i64
{
    let size = grid_size_for_cell(cellindex);

    // XXX: Probably this can be further simplified.
    let (x, y) = {
        let steps = size * size - cellindex;
        if steps < size {
            // Bottom row.
            (size - steps - 1, 0)
        } else {
            let steps = steps - (size - 1);
            if steps < size {
                // Left column.
                (0, steps)
            } else {
                let steps = steps - (size - 1);
                if steps < size {
                    // Top column.
                    (steps, size - 1)
                } else {
                    let steps = steps - (size - 1);
                    assert!(steps < size);
                    // Right column.
                    (size - 1, size - steps - 1)
                }
            }
        }
    };

    let center = (size / 2) as i64;

    (center - x as i64).abs() + (center - y as i64).abs()
}


fn main()
{
    let stdin = io::stdin();
    for row in stdin.lock().lines().filter_map(io::Result::ok) {
        for cellindex in row.split_whitespace().map(|s| u32::from_str(s).unwrap()) {
            println!("{}", manhattan_distance_for_cell(cellindex));
        }
    }
}
