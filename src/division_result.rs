use std::alloc::{alloc, Layout};
use crate::natural::Natural;

/// A container that
///  - is allocated on the heap
///  - has a fixed capacity
///  - can only grow downwards
/// 
///  This exists to allow a vector to be populated 'backwards',
///  required for the traditional division algorithm
///
pub struct DivisionResult<'a> {
    data: &'a mut [u64],
    length: usize,
    capacity: usize,
    seen_nonzero: bool,
    zeroes: usize,
}

impl<'a> DivisionResult<'a> {
    pub fn new(capacity: usize) -> Self {
        let data = unsafe {
            let l = Layout::array::<u64>(capacity).unwrap();
            let ptr = alloc(l);
            std::slice::from_raw_parts_mut(ptr as *mut u64, capacity)
        };
        Self {
            data: data,
            length: 0,
            capacity: capacity,
            seen_nonzero: false,
            zeroes: 0,
        }
    }

    pub fn push(&mut self, value: u64) {
        self.data[self.capacity - self.length - 1] = value;
        self.length += 1;
        if !self.seen_nonzero && value == 0 {
            self.zeroes += 1;
        }
        else if !self.seen_nonzero && value != 0 {
            self.seen_nonzero = true;
        }
    }
}

impl From<DivisionResult<'_>> for Natural {
    fn from(array: DivisionResult) -> Natural {
        debug_assert!(array.length == array.capacity);
        let zeroes = array.zeroes;
        let size = array.capacity;
        let mut digits = array.data.to_vec();
        for _ in 1..array.zeroes+1 {
            digits.pop();
        }
        Natural::from(digits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_to_vec(){
        let mut ra = DivisionResult::new(3);
        ra.push(1);
        ra.push(2);
        ra.push(3);
        let result = Natural::from(ra);
        let expected = Natural::from(vec!(3, 2, 1));
        assert_eq!(result, expected);
    }

    fn test_leading_insignificant_zeroes() {
        let mut dr = DivisionResult::new(3);
        dr.push(0);
        dr.push(3);
        dr.push(0);
        let result = Natural::from(dr);
        let expected = Natural::from(vec!(0, 3));
        assert_eq!(result, expected);
    }

}
