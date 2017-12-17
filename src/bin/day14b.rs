//
// day14b.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

extern crate aoc2017;
extern crate bit_vec;
extern crate num;

use aoc2017::day14::{ make_row_hash, hex_char_to_u8 };
use bit_vec::BitVec;
use num::Num;
use std::io;
use std::io::prelude::*;


struct Bitmap
{
    bits: BitVec,
}


impl Bitmap
{
    const SIDE: usize = 128;

    fn new() -> Self {
        Self {
            bits: BitVec::from_elem(Self::SIDE * Self::SIDE, false)
        }
    }

    fn from_key(s: &str) -> Self {
        let mut bmap = Self::new();
        for row in 0 .. Self::SIDE {
            let chars: Vec<_> = make_row_hash(s, row as u16).chars().map(hex_char_to_u8).collect();
            let mut bytes = Vec::with_capacity(Self::SIDE / 2);
            for pair in chars.chunks(2) {
                bytes.push(pair[0] << 4 | pair[1]);
            }
            let row_bits = BitVec::from_bytes(&bytes);
            assert_eq!(row_bits.len(), Self::SIDE);
            for col in 0 .. Self::SIDE {
                bmap.set(col, row, row_bits.get(col).unwrap());
            }
        }
        bmap
    }

    #[inline]
    fn set(&mut self, col: usize, row: usize, value: bool) {
        self.bits.set(row * Self::SIDE + col, value);
    }

    #[inline]
    fn get(&self, col: usize, row: usize) -> bool {
        if row >= Self::SIDE {
            panic!("Row index out of bounds: {}", row);
        }
        if col >= Self::SIDE {
            panic!("Column index out of bounds: {}", col);
        }
        self.bits.get(row * Self::SIDE + col).unwrap()
    }
}


struct BitmapLabels<T: Num + Clone>
{
    bits: Vec<T>,
}


impl<T: Num + Clone> BitmapLabels<T>
{
    const SIDE: usize = Bitmap::SIDE;

    fn new() -> Self {
        Self { bits: vec![T::zero(); Self::SIDE * Self::SIDE] }
    }

    #[inline]
    fn set(&mut self, col: usize, row: usize, value: T) {
        self.bits[row * Self::SIDE + col] = value;
    }

    #[inline]
    fn get(&self, col: usize, row: usize) -> T {
        self.bits[row * Self::SIDE + col].clone()
    }
}


fn main()
{
    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let mut used = Bitmap::from_key(line.trim());

        // How many bits are set?
        println!("Used bits: {}", used.bits.iter().filter(|&x| x).count());

        // Label all the connected areas.
        let mut labels = BitmapLabels::new();
        let mut queue = Vec::new();
        let mut cur_label = 0;

        for row in 0 .. Bitmap::SIDE {
            for col in 0 .. Bitmap::SIDE {
                // Flood-fill unlabeled items with the current label. The
                // queue keeps a list of flooded items whose neighbours are
                // pending to be checked.
                if used.get(row, col) && labels.get(row, col) == 0 {
                    cur_label += 1;
                    labels.set(row, col, cur_label);
                    queue.push((row, col));
                    while !queue.is_empty() {
                        let (r, c) = queue.pop().unwrap();
                        // Up.
                        if r > 0 && used.get(r - 1, c) && labels.get(r - 1, c) == 0 {
                            labels.set(r - 1, c, cur_label);
                            queue.push((r - 1, c));
                        }
                        // Left.
                        if c > 0 && used.get(r, c - 1) && labels.get(r, c - 1) == 0 {
                            labels.set(r, c - 1, cur_label);
                            queue.push((r, c - 1));
                        }
                        // Right.
                        if c + 1 < Bitmap::SIDE && used.get(r, c + 1) && labels.get(r, c + 1) == 0 {
                            labels.set(r, c + 1, cur_label);
                            queue.push((r, c + 1));
                        }
                        // Bottom.
                        if r + 1 < Bitmap::SIDE && used.get(r + 1, c) && labels.get(r + 1, c) == 0 {
                            labels.set(r + 1, c, cur_label);
                            queue.push((r + 1, c));
                        }
                    }
                }
            }
        }

        println!("Total labels: {}", cur_label);
    }
}
