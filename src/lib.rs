use std::convert::From;
use std::cmp::{Ord, Ordering};

mod slice;

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct BigUint {
    digits: Vec<u64>
}

impl From<u64> for BigUint {
    fn from(digit: u64) -> Self {
        Self {
            digits: vec!(digit),
        }
    }
}

impl From<Vec<u64>> for BigUint {
    fn from(digits: Vec<u64>) -> Self {
        Self {
            digits: digits,
        }
    }
}

impl Ord for BigUint {
    fn cmp(&self, other: &Self) -> Ordering {
        slice::cmp(&self.digits, &other.digits)
    }
}

impl PartialOrd for BigUint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl BigUint {

    pub fn zero() -> Self {
        Self {
            digits: vec!(0),
        }
    }

    pub fn one() -> Self {
        Self {
            digits: vec!(1),
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let result = slice::add(&self.digits, &other.digits);
        Self::from(result)
    }


    pub fn mul(&self, other: &Self) -> Self {
        let result = slice::mul(&self.digits, &other.digits);
        Self::from(result)
    }

    pub fn sub(&self, other: &Self) -> Option<Self> {
        match slice::sub(&self.digits, &other.digits) {
            Some(result) => {
                let mut ret = Self::from(result);
                ret.normalize();
                Some(ret)
            },
            None => None
        }
    }

    pub fn div(&self, other: &Self) -> (Self, Self) {
        let (d, r) = slice::div(&self.digits, &other.digits);
        (Self::from(d), Self::from(r))
    }

    // TODO: This increments a number in-place. Implement in-place addition for +=
    pub fn inc(&mut self) -> () {
        let mut carry = false;
        for digit in &mut self.digits {
            let (a, b) = digit.overflowing_add(1);
            *digit = a;
            carry = b;
            if !carry {
                break
            }
        }
        if carry {
            self.digits.push(1);
        }
    }



    fn normalize(&mut self) -> () {
        // There might be a better way to do this. Using iterators, can't delete from vector while iterating.
        // See contain-rs's 'Cursor' trait for a possible alternative to iterators
        let mut last_index = self.digits.len();
        for (index, digit) in self.digits.iter().rev().enumerate() {
            if *digit > 0 { 
                last_index = self.digits.len() - index; // Remember that this is a reverse iterator
                break;
            }
        }
        for _ in last_index..self.digits.len() {
            self.digits.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const NINE: u64 = u64::MAX;
    const EIGHT: u64 = u64::MAX - 1;
    const ONE: u64 = 1;

    macro_rules! assert_plus_identity {
        ($a:expr, $b:expr => $c:expr) => (
            assert_eq!($a.add(&$b), $c);
            assert_eq!($b.add(&$a), $c);
        );
    }

    #[test]
    fn can_create_from_u64() {
        let a = BigUint::from(42);
        let mut b = BigUint::zero();
        b.digits[0] = 42;
        assert_eq!(a, b);
    }

    #[test]
    fn can_add_two_one_digit_numbers_without_carry() {
        let a = BigUint::from(21);
        let b = BigUint::from(42);
        let c = BigUint::from(63);
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn can_add_two_one_digit_numbers_with_carry() {
        let a = BigUint::from(NINE);
        let b = BigUint::from(ONE);
        let c = BigUint::from(vec!(0, 1));
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn eight_plus_one_is_nine () {
        let a = BigUint::from(EIGHT);
        let b = BigUint::from(ONE);
        let c = BigUint::from(NINE);
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn can_add_numbers_with_same_number_of_digits () {
        let a = BigUint::from(vec!(1, 2));
        let b = BigUint::from(vec!(2, 4));
        let c = BigUint::from(vec!(3, 6));
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn can_add_numbers_with_different_number_of_digits_wo_carry() {
        let a = BigUint::from(vec!(1, 2));
        let b = BigUint::from(vec!(2, 4, 6, 8));
        let c = BigUint::from(vec!(3, 6, 6, 8));
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn addition_series_of_carries() {
        let a = BigUint::from(vec!(1));
        let b = BigUint::from(vec!(NINE, NINE, NINE, NINE));
        let c = BigUint::from(vec!(0, 0, 0, 0, 1));
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn test_two_digit_by_two_digit_multiply() {
        let a = BigUint::from(vec!(NINE, ONE));
        let b = BigUint::from(vec!(2, 1));
        let c = BigUint::from(vec!(EIGHT, 2, 2));
        assert_eq!(a.mul(&b), c);
    }

    #[test]
    fn test_sub_more_digits() {
        let a = BigUint::from(vec!(1, 2));
        let b = BigUint::from(vec!(8, 1, 3));
        assert_eq!(a.sub(&b), None);
    }

    #[test]
    fn test_sub_fewer_digits() {
        let a = BigUint::from(vec!(1, 2, 3));
        let b = BigUint::from(vec!(NINE, 1));
        let c = BigUint::from(vec!(2, 0, 3));
        assert_eq!(a.sub(&b), Some(c));
    }

    #[test]
    fn test_sub_bigger_number() {
        let a = BigUint::from(vec!(1, 2));
        let b = BigUint::from(vec!(3, 2));
        assert_eq!(a.sub(&b), None);
    }

    #[test]
    fn test_sub_smaller_number() {
        let a = BigUint::from(vec!(3, 2));
        let b = BigUint::from(vec!(1, 2));
        let c = BigUint::from(vec!(2));
        assert_eq!(a.sub(&b), Some(c));
    }

}
