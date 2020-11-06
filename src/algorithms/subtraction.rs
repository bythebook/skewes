use std::cmp::Ordering;
use super::comparison::cmp_slice;
use crate::{Natural,Sign};

#[inline]
pub fn sub_signed(first: &Natural, second: &Natural) -> (Sign, Natural) {
    match cmp_slice(&first.digits, &second.digits) {
        Ordering::Greater => (Sign::Positive, Natural::from(sub_slice(&first.digits, &second.digits))),
        Ordering::Equal => (Sign::Positive, Natural::ZERO),
        Ordering::Less => (Sign::Negative, Natural::from(sub_slice(&second.digits, &first.digits))),
    }
}

#[inline]
pub (in crate::algorithms) fn sub_slice_assign(first: &mut [u64], second: &[u64]) {
    let mut carry: bool = false;
    let mut other_iter = second.iter();
    for digit in first.iter_mut() {
        let other_digit = other_iter.next().or(Some(&0)).unwrap();
        let (a, b) = sub_with_carry(*digit, *other_digit, carry);
        carry = b;
        *digit = a;
    }
}

#[inline]
fn sub_slice(first: &[u64], second: &[u64]) -> Vec<u64> {
    let mut carry: bool = false;
    let mut result: Vec<u64> = Vec::with_capacity(first.len());

    let mut other_iter = second.iter();
    for digit in first.iter() {
        let other_digit = other_iter.next().or(Some(&0)).unwrap();
        let (a, b) = sub_with_carry(*digit, *other_digit, carry);
        carry = b;
        result.push(a);
    }

    normalize_vec(&mut result);
    result
}

#[inline]
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

#[inline]
fn normalize_vec(n: &mut Vec<u64>) {
    while let Some(&0) = n.last() {
        n.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NINE: u64 = u64::MAX;
    const EIGHT: u64 = u64::MAX - 1;
    
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
        let a = Natural::from(vec!(1, NINE));
        let b = Natural::from(vec!(8, 1, 3));
        let c = Natural::from(vec!(7, 2, 2));
        assert_eq!(sub_signed(&a, &b), (Sign::Negative, c));
    }

    #[test]
    fn test_sub_fewer_digits() {
        let a = Natural::from(vec!(1, 2, 3));
        let b = Natural::from(vec!(NINE, 1));
        let c = Natural::from(vec!(2, 0, 3));
        assert_eq!(sub_signed(&a, &b), (Sign::Positive, c));
    }

    #[test]
    fn test_sub_bigger_number() {
        let a = Natural::from(vec!(1, 2));
        let b = Natural::from(vec!(3, 2));
        assert_eq!(sub_signed(&a, &b), (Sign::Negative, Natural::from(2)));
    }

    #[test]
    fn test_sub_smaller_number() {
        let a = Natural::from(vec!(3, 2));
        let b = Natural::from(vec!(1, 2));
        assert_eq!(sub_signed(&a, &b), (Sign::Positive, Natural::from(2)));
    }

    /*
      ------------------------------
       Invariants
      ------------------------------
    */
    #[test]
    fn test_sub_a_a_equals_0 () {
        let a = &[3, 4, 5];
        let b = sub_slice(a, a);
        assert!(b.is_empty());
    }
}