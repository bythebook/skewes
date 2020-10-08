use std::cmp::Ordering;
use crate::integer::Sign;
use crate::algorithms::{add, mul, div, cmp_slice, sub_signed};

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Natural {
    pub(crate) digits: Vec<u64>
}


impl From<u64> for Natural {
    fn from(digit: u64) -> Self {
        Self {
            digits: vec!(digit),
        }
    }
}

impl From<Vec<u64>> for Natural {
    fn from(digits: Vec<u64>) -> Self {
        Self {
            digits: digits,
        }
    }
}

impl Ord for Natural {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_slice(&self.digits, &other.digits)
    }
}

impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Natural {

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
        let result = add(&self.digits, &other.digits);
        Self::from(result)
    }

    pub fn sub(&self, other: &Self) -> Option<Self> {
        let (sign, value) = sub_signed(&self, &other);
        match sign {
            Sign::Negative => None,
            _ => Some(value)
        }
    }


    pub fn mul(&self, other: &Self) -> Self {
        let result = mul(&self.digits, &other.digits);
        Self::from(result)
    }

    pub fn div(&self, other: &Self) -> (Self, Self) {
        div(&self, &other)
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
        let a = Natural::from(42);
        let mut b = Natural::zero();
        b.digits[0] = 42;
        assert_eq!(a, b);
    }

    #[test]
    fn can_add_two_one_digit_numbers_without_carry() {
        let a = Natural::from(21);
        let b = Natural::from(42);
        let c = Natural::from(63);
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn can_add_two_one_digit_numbers_with_carry() {
        let a = Natural::from(NINE);
        let b = Natural::from(ONE);
        let c = Natural::from(vec!(0, 1));
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn eight_plus_one_is_nine () {
        let a = Natural::from(EIGHT);
        let b = Natural::from(ONE);
        let c = Natural::from(NINE);
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn can_add_numbers_with_same_number_of_digits () {
        let a = Natural::from(vec!(1, 2));
        let b = Natural::from(vec!(2, 4));
        let c = Natural::from(vec!(3, 6));
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn can_add_numbers_with_different_number_of_digits_wo_carry() {
        let a = Natural::from(vec!(1, 2));
        let b = Natural::from(vec!(2, 4, 6, 8));
        let c = Natural::from(vec!(3, 6, 6, 8));
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn addition_series_of_carries() {
        let a = Natural::from(vec!(1));
        let b = Natural::from(vec!(NINE, NINE, NINE, NINE));
        let c = Natural::from(vec!(0, 0, 0, 0, 1));
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn test_two_digit_by_two_digit_multiply() {
        let a = Natural::from(vec!(NINE, ONE));
        let b = Natural::from(vec!(2, 1));
        let c = Natural::from(vec!(EIGHT, 2, 2));
        assert_eq!(a.mul(&b), c);
    }
    #[test]
    fn test_sub_more_digits() {
        let a = Natural::from(vec!(1, 2));
        let b = Natural::from(vec!(8, 1, 3));
        assert_eq!(a.sub(&b), None);
    }

    #[test]
    fn test_sub_fewer_digits() {
        let a = Natural::from(vec!(1, 2, 3));
        let b = Natural::from(vec!(NINE, 1));
        let c = Natural::from(vec!(2, 0, 3));
        assert_eq!(a.sub(&b), Some(c));
    }

    #[test]
    fn test_sub_bigger_number() {
        let a = Natural::from(vec!(1, 2));
        let b = Natural::from(vec!(3, 2));
        assert_eq!(a.sub(&b), None);
    }

    #[test]
    fn test_sub_smaller_number() {
        let a = Natural::from(vec!(3, 2));
        let b = Natural::from(vec!(1, 2));
        let c = Natural::from(vec!(2));
        assert_eq!(a.sub(&b), Some(c));
    }
}


