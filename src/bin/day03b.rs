//
// day03b.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

extern crate aoc2017;

use aoc2017::day03::grid_size_for_cell;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::ops::Add;
use std::str::FromStr;


struct SquareGrid<T: Clone>
{
    data: Vec<T>,
    size: usize,
}

impl<T: Clone> SquareGrid<T> {
    fn new(size: usize, value: T) -> Self {
        SquareGrid {
            data: vec![value; size * size],
            size: size,
        }
    }

    #[inline] fn size(&self) -> usize { self.size }
    #[inline] fn center(&self) -> usize { self.size / 2 }

    fn get(&self, x: usize, y: usize) -> T {
        if x >= self.size {
            panic!("Index out of bounds: x={} >= {}", x, self.size);
        }
        if y >= self.size {
            panic!("Index out of bounds: y={} >= {}", y, self.size);
        }
        unsafe { self.data.get_unchecked(self.size * y + x) }.clone()
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        if x >= self.size {
            panic!("Index out of bounds: x={} >= {}", x, self.size);
        }
        if y >= self.size {
            panic!("Index out of bounds: y={} >= {}", y, self.size);
        }
        unsafe { self.data.get_unchecked_mut(self.size * y + x) }
    }

    #[inline]
    fn set(&mut self, x: usize, y: usize, value: T) {
        *self.get_mut(x, y) = value;
    }

    #[inline]
    fn is_valid_index(&self, x: usize, y: usize) -> bool {
        x < self.size && y < self.size
    }
}


impl<T: Clone + Add<Output=T>> SquareGrid<T> {
    const COVER_STEPS: [(isize, isize); 8] = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    fn covered_sum(&self, x: usize, y: usize) -> T {
        let mut sum = self.get(x, y);  // Center point.
        for &(dx, dy) in Self::COVER_STEPS.iter() {
            let (cx, cy) = ((x as isize + dx) as usize,
                            (y as isize + dy) as usize);
            if self.is_valid_index(cx, cy) {
                sum = sum + self.get(cx, cy);
            }
        }
        sum
    }

    fn update_sum(&mut self, x: usize, y: usize) -> T {
        let sum = self.covered_sum(x, y);
        self.set(x, y, sum.clone());
        sum
    }
}


impl<T: Clone + fmt::Display> fmt::Debug for SquareGrid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.size() {
            for col in 0..self.size() {
                write!(f, "{:4}", self.get(col, self.size - row - 1))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    EAST,
    NORTH,
    WEST,
    SOUTH,
}

impl Direction {
    fn left(&self) -> Self {
        match *self {
            Direction::EAST => Direction::NORTH,
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
        }
    }

    fn ahead(&self, x: usize, y: usize) -> (usize, usize) {
        match *self {
            Direction::EAST  => (x + 1, y),
            Direction::NORTH => (x, y + 1),
            Direction::WEST  => (x - 1, y),
            Direction::SOUTH => (x, y - 1),
        }
    }
}


fn calculate(value: u32) -> u32
{
    let size = grid_size_for_cell(value);
    let mut grid = SquareGrid::new(size as usize, 0);
    let mut x = grid.center();
    let mut y = grid.center();

    // Initial value.
    grid.set(x, y, 1);
    assert_eq!(1, grid.covered_sum(x, y));

    // Move to the first position which must get filled.
    let mut d = Direction::EAST;
    let (sx, sy) = d.ahead(x, y);
    x = sx;
    y = sy;
    grid.update_sum(x, y);

    while grid.get(x, y) <= value {
        let left = d.left();
        let (lx, ly) = left.ahead(x, y);
        if grid.get(lx, ly) == 0 {
            // Not filled: Can turn left.
            d = left;
            x = lx;
            y = ly;
        } else {
            // Already filled, just go ahead.
            let (ax, ay) = d.ahead(x, y);
            x = ax;
            y = ay;
        };
        grid.update_sum(x, y);
    }

    grid.get(x, y)
}


fn main()
{
    let stdin = io::stdin();
    for row in stdin.lock().lines().filter_map(io::Result::ok) {
        for value in row.split_whitespace().map(|s| u32::from_str(s).unwrap()) {
            println!("{}", calculate(value));
        }
    }
}
