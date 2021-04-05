// When 'Step' is stabilised, it will become ergonomic to use Rust ranges with Naturals
// Until then, we provide our own range object with an iterator
// This is memory efficient, as it maintains just the end and loop variable
// Only supports increasing loops this way. May change this in future

use core::iter::Iterator;
use crate::natural::Natural;

///
/// A range of positive integers
/// 
/// # Example
/// ```rust
/// use skewes::{Natural,Range};
/// 
/// // Create a range
/// let numbers = Range::new(Natural::from(1), Natural::from(10));
/// for number in numbers {
///     println!("{}", number);
/// }
/// ```
/// 
pub struct Range {
    current: Natural,
    end: Natural,
}

impl Range {
    ///
    /// Create a new range [a,b)
    /// 
    /// The created range represents natural numbers greater than or equal to a
    /// and strictly less than b
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
        self.current = &self.current + &Natural::from(1);
        value
    }
}