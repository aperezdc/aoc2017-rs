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
