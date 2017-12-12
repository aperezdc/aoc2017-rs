//
// day11.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

// Awesome resource on hex grid tiling algorithms:
//    https://www.redblobgames.com/grids/hexagons/

#[macro_use]
extern crate failure;

use std::io;
use std::io::prelude::*;
use std::str::FromStr;


#[derive(Debug)]
pub enum Dir {
    N(u32),
    NE(u32),
    SE(u32),
    S(u32),
    SW(u32),
    NW(u32),
}

impl FromStr for Dir
{
    type Err = ::failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        Ok(match s {
            "n"  => Dir::N(1),
            "ne" => Dir::NE(1),
            "se" => Dir::SE(1),
            "s"  => Dir::S(1),
            "sw" => Dir::SW(1),
            "nw" => Dir::NW(1),
            _    => bail!("invalid direction: {}", s),
        })
    }
}


mod coord {
    use super::Dir;

    #[derive(Default, Debug, Copy, Clone)]
    pub struct Axis { q: isize, r: isize }

    impl Axis {
        #[inline]
        pub fn new(q: isize, r: isize) -> Self { Axis { q: q, r: r } }

        #[inline]
        fn to_cube(&self) -> Cube { Cube::new(self.q, -self.q - self.r, self.r) }

        pub fn add(&self, d: Dir) -> Self {
            match d {
                Dir::N(s)  => Axis::new(self.q, self.r - s as isize),
                Dir::NE(s) => Axis::new(self.q + s as isize, self.r - s as isize),
                Dir::SE(s) => Axis::new(self.q + s as isize, self.r),
                Dir::S(s)  => Axis::new(self.q, self.r + s as isize),
                Dir::SW(s) => Axis::new(self.q - s as isize, self.r + s as isize),
                Dir::NW(s) => Axis::new(self.q - s as isize, self.r),
            }
        }

        pub fn distance_to(&self, other: &Axis) -> usize {
            self.to_cube().distance_to(&other.to_cube())
        }
    }

    struct Cube { x: isize, y: isize, z: isize }

    impl Cube {
        #[inline]
        pub fn new(x: isize, y: isize, z: isize) -> Self {
            assert_eq!(0, x + y + z);
            Cube { x: x, y: y, z: z }
        }

        #[inline]
        pub fn distance_to(&self, other: &Cube) -> usize {
            ((self.x - other.x).abs() as usize +
             (self.y - other.y).abs() as usize +
             (self.z - other.z).abs() as usize) / 2
        }
    }
}


fn main()
{
    let stdin = io::stdin();
    let mut pos = coord::Axis::default();
    let mut max_steps = 0;
    for s in stdin.lock().split(0x2C /* comma */)
            .filter_map(Result::ok)
            .map(String::from_utf8)
            .filter_map(Result::ok)
    {
        pos = pos.add(s.trim().parse::<Dir>().unwrap());
        let steps = pos.distance_to(&coord::Axis::default());
        if steps > max_steps {
            max_steps = steps;
        }
    }
    println!("Steps: {}, max: {}",
             pos.distance_to(&coord::Axis::default()),
             max_steps);
}
