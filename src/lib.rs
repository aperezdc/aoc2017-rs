//
// lib.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

#![feature(conservative_impl_trait)]

use std::io::{ self, Read };


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
