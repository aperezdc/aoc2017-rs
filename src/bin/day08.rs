//
// day08.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

#[macro_use] extern crate failure;

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;


#[derive(Debug)]
enum Cond
{
    EQ(String, i32),  // <name> == <num>
    NE(String, i32),  // <name> != <num>
    GT(String, i32),  // <name> >  <num>
    GE(String, i32),  // <name> >= <num>
    LT(String, i32),  // <name> <  <num>
    LE(String, i32),  // <name> <= <num>
}

impl FromStr for Cond
{
    type Err = ::failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if let Some(pos) = s.find(" == ") {
            let (regname, value) = s.split_at(pos);
            Ok(Cond::EQ(regname.trim().to_string(), value[4..].trim().parse()?))
        } else if let Some(pos) = s.find(" != ") {
            let (regname, value) = s.split_at(pos);
            Ok(Cond::NE(regname.trim().to_string(), value[4..].trim().parse()?))
        } else if let Some(pos) = s.find(" >= ") {
            let (regname, value) = s.split_at(pos);
            Ok(Cond::GE(regname.trim().to_string(), value[4..].trim().parse()?))
        } else if let Some(pos) = s.find(" <= ") {
            let (regname, value) = s.split_at(pos);
            Ok(Cond::LE(regname.trim().to_string(), value[4..].trim().parse()?))
        } else if let Some(pos) = s.find(" > ") {
            let (regname, value) = s.split_at(pos);
            Ok(Cond::GT(regname.trim().to_string(), value[3..].trim().parse()?))
        } else if let Some(pos) = s.find(" < ") {
            let (regname, value) = s.split_at(pos);
            Ok(Cond::LT(regname.trim().to_string(), value[3..].trim().parse()?))
        } else {
            bail!("Condition '{}' has an invalid relational operator", s)
        }
    }
}


#[derive(Debug)]
enum Op
{
    Inc(String, i32),  // <name> inc <num>
    Dec(String, i32),  // <name> dec <num>
}

impl FromStr for Op
{
    type Err = ::failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if let Some(pos) = s.find(" inc ") {
            let (regname, value) = s.split_at(pos);
            Ok(Op::Inc(regname.trim().to_string(), value[5..].trim().parse()?))
        } else if let Some(pos) = s.find(" dec ") {
            let (regname, value) = s.split_at(pos);
            Ok(Op::Dec(regname.trim().to_string(), value[5..].trim().parse()?))
        } else {
            bail!("Instruction '{}' contains invalid operation", s)
        }
    }
}


#[derive(Debug)]
struct Instr
{
    op: Op,
    cond: Cond,
}

impl FromStr for Instr
{
    type Err = ::failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let if_pos = if let Some(pos) = s.find(" if ") { pos } else {
            bail!("Input '{}' does not contain 'if'", s);
        };
        let (op_str, cond_str) = s.split_at(if_pos);
        Ok(Instr {
            op: op_str.trim().parse()?,
            cond: cond_str[4..].trim().parse()?,
        })
    }
}


struct Machine
{
    regs: HashMap<String, i32>,
    max_seen: i32,
}


impl Machine
{
    fn new() -> Self {
        Machine {
            regs: HashMap::new(),
            max_seen: i32::min_value(),
        }
    }

    #[inline]
    fn get(&self, regname: &str) -> i32 {
        *self.regs.get(regname).unwrap_or(&0)
    }

    fn check_condition(&self, cond: &Cond) -> bool {
        match *cond {
            Cond::EQ(ref r, v) => self.get(r) == v,
            Cond::NE(ref r, v) => self.get(r) != v,
            Cond::GT(ref r, v) => self.get(r) >  v,
            Cond::GE(ref r, v) => self.get(r) >= v,
            Cond::LT(ref r, v) => self.get(r) <  v,
            Cond::LE(ref r, v) => self.get(r) <= v,
        }
    }

    fn execute(&mut self, ins: &Instr) {
        if self.check_condition(&ins.cond) {
            match ins.op {
                Op::Inc(ref r, v) => self.inc(r, v),
                Op::Dec(ref r, v) => self.dec(r, v),
            }
        }
    }

    #[inline]
    fn inc(&mut self, regname: &str, value: i32) {
        let reg = self.regs.entry(regname.to_string()).or_insert(0);
        *reg += value;
        if *reg > self.max_seen {
            self.max_seen = *reg;
        }
    }

    #[inline]
    fn dec(&mut self, regname: &str, value: i32) {
        let reg =  self.regs.entry(regname.to_string()).or_insert(0);
        *reg -= value;
        if *reg > self.max_seen {
            self.max_seen = *reg;
        }
    }
}

impl fmt::Display for Machine
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for (regname, value) in &self.regs {
            write!(f, "{}: {}\n", regname, value)?;
        }
        Ok(())
    }
}


fn main()
{
    let mut m = Machine::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(io::Result::ok) {
        let instr = line.parse().unwrap();
        m.execute(&instr);
    }

    let mut max_value = i32::min_value();
    for (_regname, &value) in &m.regs {
        if value > max_value {
            max_value = value;
        }
    }
    println!("max reg value: {}", max_value);
    println!("max seen value: {}", m.max_seen);
}
