//
// lib.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

#![feature(conservative_impl_trait)]

use std::io::{ self, BufRead, Read };
use std::str::FromStr;


#[inline]
fn is_ascii_digit(byte: &u8) -> bool
{
    *byte >= 0x30 && *byte <= 0x39
}


fn ascii_digit(byte: u8) -> u32
{
    if is_ascii_digit(&byte) {
        byte as u32 - 0x30
    } else {
        panic!("Byte 0x{:02X} is not a number", byte)
    }
}


pub fn iter_digits<R: Read>(iter: io::Bytes<R>) -> impl Iterator<Item=u32>
{
    iter.map(|item| item.unwrap())
        .take_while(is_ascii_digit)
        .map(ascii_digit)
}


pub fn rows_of_digits<BR: BufRead>(iter: BR) -> impl Iterator<Item=Vec<i32>>
{
    iter.lines()
        .filter_map(io::Result::ok)
        .map(|line| {
            line.split_whitespace()
                .map(|s| i32::from_str(s).unwrap())
                .collect()
        })
}


#[derive(Debug)]
pub struct VecPermutations<T> {
   v: Vec<T>,
   i: usize,
   j: usize,
}


impl<T> Iterator for VecPermutations<T>
    where T: Clone
{
    type Item = (T, T);

    fn size_hint(&self) -> (usize, Option<usize>) {
        let nitems = self.v.len() * self.v.len();
        (nitems, Some(nitems))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.v.len() && self.j < self.v.len() {
            let item = (self.v[self.i].clone(), self.v[self.j].clone());
            self.i += 1;
            if self.i >= self.v.len() {
                self.i = 0;
                self.j += 1;
            }
            Some(item)
        } else {
            None
        }
    }
}


pub trait Permutations
{
    type IteratorType;

    fn permutations(self) -> Self::IteratorType;
}


impl<T> Permutations for Vec<T>
{
    type IteratorType = VecPermutations<T>;

    fn permutations(self) -> Self::IteratorType
    {
        VecPermutations { v: self, i: 0, j: 0 }
    }
}


pub mod day03 {
    pub fn grid_size_for_cell(cellindex: u32) -> u32
    {
        let mut size = 1;
        while cellindex > size * size {
            size += 2;
        }
        return size;
    }
}


pub mod day10
{
    use std::fmt;

    pub struct KnotHash
    {
        list: [u8; 256],
        pos: usize,
        skip: usize,
    }

    impl KnotHash
    {
        const ROUNDS: usize = 64;
        const XORITEMS: usize = 16;
        const XORGROUPS: usize = 256 / Self::XORITEMS;

        pub fn new() -> Self {
            let mut kh = KnotHash { list: [0u8; 256], pos: 0, skip: 0 };
            for i in 0 .. kh.list.len() {
                kh.list[i] = i as u8;
            }
            kh
        }

        fn apply(&mut self, n: u8) {
            // 1. Reverse the order of elements pos..n
            let len = self.list.len();
            let mut i = self.pos;
            let mut j = self.pos + (n as usize) - 1;
            while i < j {
                self.list.swap(i % len, j % len);
                i += 1;
                j -= 1;
            }

            // 2. Increase position by n+skip
            self.pos = (self.pos + (n as usize) + self.skip) % self.list.len();

            // 3. Increase skip by one.
            self.skip += 1;
        }

        #[inline]
        pub fn rounds(&mut self, input: &[u8]) {
            for _ in 0..Self::ROUNDS {
                for &b in input {
                    self.apply(b);
                }
                // Apply the standard tail lengths.
                for &b in &[17, 31, 73, 47, 23] {
                    self.apply(b);
                }
            }
        }
    }

    impl fmt::LowerHex for KnotHash
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for group in 0 .. Self::XORGROUPS {
                let startpos = group * Self::XORITEMS;
                let mut xor = 0;
                for x in &self.list[startpos .. startpos + Self::XORITEMS] {
                    xor ^= x;
                }
                write!(f, "{:02x}", xor)?;
            }
            Ok(())
        }
    }
}
