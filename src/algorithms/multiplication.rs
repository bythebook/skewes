use std::convert::TryFrom; // For downcasting u128 to u64
use super::{add, add_mut};

pub fn mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut accumulator = vec!(0);
    let mut significance = 0;
    for digit in a.iter() {
        let result = mul_by_single_digit(b, *digit, significance);
        accumulator = add(&result, &accumulator);
        significance += 1;
    }
    accumulator
}

// Multiply a slice by a single u64 digit, allowing for significance number of zeroes at the start
// Used to build up longhand multiplication
pub fn mul_by_single_digit(digits: &[u64], digit: u64, significance: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut msd: u64 = 0;
    let mut lsd: u64;
    for _ in 1..(significance+1) {
        result.push(0);
    }

    for other_digit in digits.iter() {
        let (a, b) = mul_with_carry(*other_digit, digit);
        lsd = a; 
        let (current_digit, carry) = lsd.overflowing_add(msd);
        result.push(current_digit);
        msd = b + (carry as u64);
    }
    if msd > 0 {
        result.push(msd);
    }
    result
}

fn mul_with_carry(digita: u64, digitb: u64) -> (u64, u64) {
    let result = (digita as u128) * (digitb as u128);
    (u64::try_from(result & (u64::MAX as u128)).unwrap(),
     u64::try_from(result >> 64).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const NINE: u64 = u64::MAX;
    const EIGHT: u64 = u64::MAX - 1;
    const ONE: u64 = 1;

    /*
    ------------------------------------
    Multiplication with carry tests
    ------------------------------------
    */

    #[test]
    fn test_multiply_with_carry() {
        let a = NINE;
        let b = NINE;
        let c = (1, EIGHT);
        assert_eq!(mul_with_carry(a, b), c);
    }

    #[test]
    fn test_single_digit_multiply() {
        let a = &[NINE];
        let b = NINE;
        let c = &[1, EIGHT];
        assert_eq!(mul_by_single_digit(a, b, 0), c);
    }

    #[test]
    fn test_single_digit_multiply_with_significance() {
        let a = &[NINE];
        let b = NINE;
        let c = &[0, 0, 1, EIGHT];
        assert_eq!(mul_by_single_digit(a, b, 2), c);
    }

    #[test]
    fn test_two_digit_by_two_digit_multiply() {
        let a = &[NINE, ONE];
        let b = &[2, 1];
        let c = &[EIGHT, 2, 2];
        assert_eq!(mul(a, b), c);
    }


    #[test]
    fn test_single_mul_temp() {
        const SEVEN: u64 = u64::MAX - 2;
        let a = SEVEN;
        let b = [3];
        assert_eq!(mul_by_single_digit(&b, a, 0), [u64::MAX-8, 2]);
    }

    #[test]
    fn test_mul_temp() {
        const SEVEN: u64 = u64::MAX - 2;
        let a = [SEVEN];
        let b = [3];
        assert_eq!(mul(&a, &b), [u64::MAX - 8, 2]);
    }
}
