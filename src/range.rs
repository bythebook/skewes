// When 'Step' is stabilised, it will become ergonomic to use Rust ranges with Naturals
// Until then, we provide our own range object with an iterator
// This is memory efficient, as it maintains just the end and loop variable
// Only supports increasing loops this way. May change this in future

use core::iter::Iterator;
use crate::natural::Natural;

pub struct Range {
    current: Natural,
    end: Natural,
}

impl Range {
    pub fn new(a: Natural, b: Natural) -> Range {
        Self {
            current: a,
            end: b,
        }
    }
}

impl Iterator for Range {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        let value = if self.current < self.end {
            Some(self.current.clone())
        }
        else {
            None
        };
        self.current = &self.current + &Natural::one();
        value
    }
}