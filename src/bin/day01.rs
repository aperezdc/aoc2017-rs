//
// main.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

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


fn main()
{
    let mut sum = 0;

    let mut digits = io::stdin().bytes()
        .map(|item| item.unwrap())
        .take_while(is_ascii_digit)
        .map(ascii_digit);

    let first = digits.next().unwrap();
    let mut last = first;

    loop {
        match digits.next() {
            None => break,
            Some(digit) => {
                if digit == last {
                    sum += last;
                }
                last = digit;
            },
        }
    }

    // Wrap around.
    if first == last {
        sum += last;
    }

    println!("{}", sum);
}
