//! Provides types for dealing with integers larger than native size
//!
//! The main type is Integer, while it also provides a Natural type for positive numbers.
//! 
//! This is supported on machines with 64-bit word size though this might be revisited in a future revision
//!
//! Quick Start
//! =============
//! 
//! ```rust
//! use skewes::Natural;
//! 
//! // Create a number from a string
//! let n = Natural::from_string("1234567890000000000");
//! 
//! // Create a number from a native type
//! let m = Natural::from(123);
//! 
//! // Do arithmetic with these numbers
//! let x = &m + &n;
//! 
//! println!("x = {}", x);
//! 
#![warn(clippy::all)]

// Allow single-character variable names in functions
// This is a 'mathsy' coding style, decision to use this may be revisited in future
#![allow(clippy::many_single_char_names)]

#![warn(missing_docs)]

mod natural;
mod integer;
mod algorithms;
mod division_result;
mod range;

pub use natural::Natural;
pub use integer::Sign;
pub use integer::Integer;
pub use range::Range;
