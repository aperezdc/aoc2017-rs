//
// day13.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

use std::fmt;
use std::io;
use std::io::prelude::*;


#[derive(Debug, Clone)]
struct Layer
{
    len: u32,
    pos: u32,
    fwd: bool,  // Going down (forward) or up (backward).
}

impl Layer
{
    fn new(range: u32) -> Self {
        Self { len: range, pos: 0, fwd: true }
    }

    #[inline]
    fn reset(&mut self) {
        self.pos = 0;
        self.fwd = true;
    }

    #[inline]
    fn range(&self) -> u32 {
        self.len
    }

    #[inline]
    fn tick(&mut self) {
        if self.len > 1 {
            if self.fwd {
                if self.pos == self.len - 1 {
                    self.fwd = false;
                    self.pos -= 1;
                } else {
                    self.pos += 1;
                }
            } else {
                if self.pos == 0 {
                    self.fwd = true;
                    self.pos += 1;
                } else {
                    self.pos -= 1;
                }
            }
        }
    }

    fn scanner_at_top(&self) -> bool {
        self.len > 0 && self.pos == 0
    }
}


struct Firewall
{
    layers: Vec<Layer>,
    pos: usize,
    initial: bool,
    collision: Option<u32>,
}

impl Clone for Firewall {
    fn clone(&self) -> Self {
        Self {
            layers: self.layers.iter().map(Layer::clone).collect(),
            pos: self.pos,
            initial: self.initial,
            collision: self.collision,
        }
    }
}

impl fmt::Debug for Firewall
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut max_range = 0;
        for i in 0 .. self.layers.len() {
            write!(f, " {}  ", i)?;
            if self.layers[i].range() > max_range {
                max_range = self.layers[i].range();
            }
        }
        write!(f, "\n")?;
        for row in 0 .. max_range {
            for i in 0 .. self.layers.len() {
                if row == 0 && self.pos == i {
                    let item = if self.layers[i].range() == 0 { "." }
                          else if self.layers[i].pos == row { "S" }
                          else { " " };
                    if self.initial {
                        write!(f, "[{}] ", item)?;
                    } else {
                        write!(f, "({}) ", item)?;
                    }
                } else if self.layers[i].range() <= row {
                    write!(f, "... ")?;
                } else {
                    write!(f, "[{}] ", if self.layers[i].pos == row { "S" } else { " " })?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Firewall
{
    fn new() -> Self {
        Self {
            layers: Vec::new(),
            pos: 0,
            initial: true,
            collision: None,
        }
    }

    fn set(&mut self, depth: u32, layer: Layer) {
        let depth = depth as usize;
        for _ in 0 .. (depth - self.layers.len() + 1) {
            self.layers.push(Layer::new(0));  // Fill with empty layers.
        }
        assert!(depth < self.layers.len());
        self.layers[depth] = layer;
    }

    #[inline]
    fn reset(&mut self) {
        self.layers.iter_mut().for_each(Layer::reset);
        self.reset_packet();
    }

    #[inline]
    fn reset_packet(&mut self) {
        self.collision = None;
        self.initial = true;
        self.pos = 0;
    }

    #[inline]
    fn finished(&self) -> bool {
        self.pos >= self.layers.len()
    }

    #[inline]
    fn collided(&self) -> bool {
        !self.finished() && self.layers[self.pos].scanner_at_top()
    }

    #[inline]
    fn collision_severity(&self) -> Option<u32> {
        if self.collided() {
            Some(self.pos as u32 * self.layers[self.pos].range())
        } else {
            None
        }
    }

    #[inline]
    fn tick(&mut self) {
        if !self.finished() {
            if self.initial {
                self.initial = false;
            } else {
                self.pos += 1;
            }
            self.collision = self.collision_severity();
            self.layers.iter_mut().for_each(Layer::tick);
        }
    }

    fn trip_severity(&mut self) -> Option<u32> {
        let mut total_severity = 0;
        let mut caught = false;
        while !self.finished() {
            self.tick();
            if let Some(severity) = self.collision {
                total_severity += severity;
                caught = true;
            }
        }
        if caught {
            Some(total_severity)
        } else {
            None
        }
    }
}


fn main()
{
    let mut fw = Firewall::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        if let Some(pos) = line.find(':') {
            let (left, right) = line.split_at(pos);
            let depth = left.trim().parse().unwrap();
            let range = right[1..].trim().parse().unwrap();
            fw.set(depth, Layer::new(range));
        }
    }

    fw.reset();
    println!("Trip severity: {}", fw.trip_severity().unwrap_or(0));

    fw.reset();
    for delay in 0 .. {
        if delay % 100 == 0 {
            print!("\r[KDelay: {}", delay);
            io::stdout().flush().unwrap();
        }

        // Check what the severity of the trip would be now. We use a clone
        // so we can continue using "fw" to calculate states after the delay.
        if fw.clone().trip_severity() == None {
            println!("\r[KDelay to not be caught: {} picosends", delay);
            break;
        }

        // Add one tick of delay, reset packet to initial position.
        fw.tick();
        fw.reset_packet();
    }
}
