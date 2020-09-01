use std::convert::From;
use std::convert::TryFrom;
use std::cmp::{Ord, Ordering};

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
        if self.digits.len() > other.digits.len() {
            return Ordering::Greater
        }
        else if self.digits.len() < other.digits.len() {
            return Ordering::Less
        }
        else {
            let iter = self.digits.iter().zip(other.digits.iter());
            for (digit, other_digit) in iter.rev() {
                if *digit > *other_digit {
                    return Ordering::Greater
                }
                else if *digit < *other_digit {
                    return Ordering::Less
                }
            }
            Ordering::Equal
        }
    }
}

impl PartialOrd for BigUint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl BigUint {
    pub fn new() -> Self {
        Self {
            digits: vec!(0),
        }
    }

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

    pub fn plus(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        let first: &Self;
        let second: &Self;
        
        // Order numbers so that first is the one with the greater number of digits
        if self.digits.len() < other.digits.len() {
            first = other;
            second = self;
        }
        else {
            first = self;
            second = other;
        }
        let mut firstiter = first.digits.iter();
        let mut seconditer = second.digits.iter();
        let mut carry = false;

        // Add digits of the second number to the first
        loop {
            match seconditer.next() {
                Some(seconddigit) => {
                    let firstdigit = firstiter.next().unwrap();
                    let (a, b) = add_with_carry(*firstdigit, *seconddigit, carry);
                    carry = b;
                    result.push(a);
                }
                None => break,
            }
        }

        // Propagate any left over carries from the second number to the first
        loop {
            match firstiter.next() {
                Some(firstdigit) => {
                    let (a, b) = add_with_carry(*firstdigit, 0, carry);
                    carry = b;
                    match carry {
                        true => result.push(a),
                        false => {result.push(a); break;}
                    }
                },
                None => {
                    if carry {
                        result.push(1);
                    }
                    return Self::from(result);
                }
            }
        }

        // Add remaining digits of the first number to the result
        loop {
            match firstiter.next() {
                Some(firstdigit) => result.push(*firstdigit),
                None => break,
            }
        }

        // Add the final one if there are any remaining carries
        if carry {
            result.push(1);
        }

        Self::from(result)
    }

    pub fn minus(&self, other: &Self) -> Option<Self> {
        let mut carry: bool = false;
        let mut result: Vec<u64> = Vec::new(); // TODO can size this
        let mut other_iter = other.digits.iter();
        if other > self {
            return None
        }
        else {
            for digit in self.digits.iter() {
                match other_iter.next() {
                    Some(other_digit) => {
                        let (a, b) = sub_with_carry(*digit, *other_digit, carry);
                        carry = b;
                        result.push(a);
                    },
                    None => {
                        let (a, b) = sub_with_carry(*digit, 0, carry);
                        carry = b;
                        result.push(a);
                    }
                }
            }
            if carry {
                return None // Can't happen, as we checked for one bigger than the other
            }
        }
        let mut ret = Self::from(result);
        ret.normalize();
        Some(ret)
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut accumulator = Self::new();
        let mut significance = 0;
        for digit in self.digits.iter() {
            let result = other.mul_by_single_digit(*digit, significance);
            accumulator = result.plus(&accumulator);
            significance += 1;
        }
        accumulator
    }

    pub fn div(&self, _other: &Self) -> (Self, Self) {
        (Self::zero(), Self::zero())
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

    // Idea: have this operate on slices of u64 instead, then can reuse data when performing long division
    pub fn short_div(&self, other: &Self) -> (u64, Self) {
        let mut d = 0;
        let mut candidate = other.clone();
        loop {
            match self.minus(&candidate) {
                None => break,
                Some(_) => {
                    candidate = candidate.plus(other); //TODO: in-place addition
                    d += 1;
                },
            }
        }
        // TODO: This smells bad
        (d, self.minus(&candidate.minus(&other).unwrap()).unwrap()) // Guaranteed to be safe TODO: add unchecked minus
    }

    // Multiply a BigUint by a single u64 digit, allowing for significance number of zeroes at the start
    // Used to build up longhand multiplication
    fn mul_by_single_digit(&self, digit: u64, significance: u64) -> Self {
        let mut result = Vec::new();
        let mut msd: u64 = 0;
        let mut lsd: u64;
        for _ in 1..(significance+1) {
            result.push(0);
        }

        for other_digit in self.digits.iter() {
            let (a, b) = mul_with_carry(*other_digit, digit);
            lsd = a; 
            let (current_digit, carry) = lsd.overflowing_add(msd);
            result.push(current_digit);
            msd = b + (carry as u64);
        }
        if msd > 0 {
            result.push(msd);
        }
        BigUint::from(result)
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

fn add_with_carry(digita: u64, digitb: u64, prev_carry: bool) -> (u64, bool) {
    let (resultdigit, new_carry) = digita.overflowing_add(digitb);
    let carrydigit = prev_carry as u64;
    match new_carry {
        true => (resultdigit + carrydigit, true),
        false => resultdigit.overflowing_add(carrydigit),
    }
}

fn sub_with_carry(digita: u64, digitb: u64, prev_carry: bool) -> (u64, bool) {
    let (resultdigit, new_carry) = digita.overflowing_sub(digitb);
    match prev_carry {
        true => {
            // If the previous carry causes overflow, the result of the place digits was 0, so carry should be true
            // If the previous carry doesn't, we use the overflow result previously arrived at
            match resultdigit.overflowing_sub(1) {
                (res, true) => (res, true),
                (res, false) => (res, new_carry),
            }
        },
        false => (resultdigit, new_carry)
    }
}

fn mul_with_carry(digita: u64, digitb: u64) -> (u64, u64) {
    let result = (digita as u128) * (digitb as u128);
    (u64::try_from(result & (u64::MAX as u128)).unwrap(),
     u64::try_from(result >> 64).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const NINE: u64 = u64::MAX;
    const EIGHT: u64 = u64::MAX - 1;
    const ONE: u64 = 1;

    macro_rules! assert_plus_identity {
        ($a:expr, $b:expr => $c:expr) => (
            assert_eq!($a.plus(&$b), $c);
            assert_eq!($b.plus(&$a), $c);
        );
    }

    #[test]
    fn add_with_carry_one_plus_nine_wo_carry() {
        let (result, carry) = add_with_carry(NINE, ONE, false);
        assert_eq!(result, 0);
        assert_eq!(carry, true);
    }

    #[test]
    fn add_with_carry_one_plus_nine_w_carry() {
        let (result, carry) = add_with_carry(NINE, ONE, true);
        assert_eq!(result, 1);
        assert_eq!(carry, true);
    }

    #[test]
    fn add_with_carry_one_plus_eight_wo_carry() {
        let (result, carry) = add_with_carry(EIGHT, ONE, false);
        assert_eq!(result, NINE);
        assert_eq!(carry, false);
    }

    #[test]
    fn add_with_carry_one_plus_eight_w_carry() {
        let (result, carry) = add_with_carry(EIGHT, ONE, true);
        assert_eq!(result, 0);
        assert_eq!(carry, true);
    }

    #[test]
    fn can_create_from_u64() {
        let a = BigUint::from(42);
        let mut b = BigUint::new();
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
    fn test_multiply_with_carry() {
        let a = NINE;
        let b = NINE;
        let c = (1, EIGHT);
        assert_eq!(mul_with_carry(a, b), c);
    }


    #[test]
    fn test_single_digit_multiply() {
        let a = BigUint::from(NINE);
        let b = NINE;
        let c = BigUint::from(vec!(1, EIGHT));
        assert_eq!(a.mul_by_single_digit(b, 0), c);
    }

    #[test]
    fn test_single_digit_multiply_with_significance() {
        let a = BigUint::from(NINE);
        let b = NINE;
        let c = BigUint::from(vec!(0, 0, 1, EIGHT));
        assert_eq!(a.mul_by_single_digit(b, 2), c);
    }

    #[test]
    fn test_two_digit_by_two_digit_multiply() {
        let a = BigUint::from(vec!(NINE, ONE));
        let b = BigUint::from(vec!(2, 1));
        let c = BigUint::from(vec!(EIGHT, 2, 2));
        assert_eq!(a.mul(&b), c);
    }

    #[test]
    fn test_sub_two_digits_wo_carry() {
        let (result, carry) = sub_with_carry(1, NINE, false);
        assert_eq!((result, carry), (2, true));

        let (result, carry) = sub_with_carry(NINE, 1, false);
        assert_eq!((result, carry), (EIGHT, false));

        let (result, carry) = sub_with_carry(NINE, EIGHT, false);
        assert_eq!((result, carry), (1, false));
    }

    #[test]
    fn test_sub_two_digits_w_carry() {
        let (result, carry) = sub_with_carry(1, NINE, true);
        assert_eq!((result, carry), (1, true));

        let (result, carry) = sub_with_carry(NINE, 1, true);
        assert_eq!((result, carry), (u64::MAX - 2, false));

        let (result, carry) = sub_with_carry(NINE, EIGHT, true);
        assert_eq!((result, carry), (0, false));
    }

    #[test]
    fn test_sub_more_digits() {
        let a = BigUint::from(vec!(1, 2));
        let b = BigUint::from(vec!(8, 1, 3));
        assert_eq!(a.minus(&b), None);
    }

    #[test]
    fn test_sub_fewer_digits() {
        let a = BigUint::from(vec!(1, 2, 3));
        let b = BigUint::from(vec!(NINE, 1));
        let c = BigUint::from(vec!(2, 0, 3));
        assert_eq!(a.minus(&b), Some(c));
    }

    #[test]
    fn test_sub_bigger_number() {
        let a = BigUint::from(vec!(1, 2));
        let b = BigUint::from(vec!(3, 2));
        assert_eq!(a.minus(&b), None);
    }

    #[test]
    fn test_sub_smaller_number() {
        let a = BigUint::from(vec!(3, 2));
        let b = BigUint::from(vec!(1, 2));
        let c = BigUint::from(vec!(2));
        assert_eq!(a.minus(&b), Some(c));
    }

    #[test]
    fn test_short_div() {
        let a = BigUint::from(234);
        let b = BigUint::from(123);
        let c = BigUint::from(14);
        assert_eq!(a.short_div(&b), (1, BigUint::from(111)));
        assert_eq!(c.short_div(&b), (0, c.clone()));
        assert_eq!(a.short_div(&c), (0, BigUint::from(10)));
    }

}
