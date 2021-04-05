use core::convert::TryFrom; // For downcasting u128 to u64
use core::cmp::min;
use super::{add, add_mut};
use super::subtraction::sub_slice;
use super::util::shl_mut_vec;

pub fn mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut accumulator = Vec::new();
    for (significance, digit) in a.iter().enumerate() {
        let result = mul_by_single_digit(b, *digit, significance as u64);
        accumulator = add(&result, &accumulator);
    }
    accumulator
}

// Future feature
#[allow(dead_code)]
fn karatsuba(a: &[u64], b: &[u64]) -> Vec<u64> {
    // Make recursive
    let x = a; let y = b;
    let n = min(x.len(), y.len());
    let result : Vec<u64>;
    if n > 16 {
        // Split x = x_1 * B^[n/2] + x_0;
        //       y = y_1 * B^[n/2] + y_0;
        // where B is the base (2^64)
        let (x_0, x_1) = x.split_at(n / 2); // Little-endian
        let (y_0, y_1) = y.split_at(n / 2); // Little-endian

        let mut z_2 = karatsuba(x_1, y_1);
        let z_0 = karatsuba(x_0, y_0);
        let z = karatsuba(&add(x_0, x_1), &add(y_0, y_1));
        let mut z_1 = sub_slice(&sub_slice(&z, &z_2), &z_0);
        shl_mut_vec(&mut z_2, n);
        shl_mut_vec(&mut z_1, n/2);
        add_mut(&mut z_2, &z_1);
        add_mut(&mut z_2, &z_0);
        result = z_2;
    }
    else {
        result = mul(&x, &y);
    }

    result
}

// Multiply a slice by a single u64 digit, allowing for significance number of zeroes at the start
// Used to build up longhand multiplication
pub fn mul_by_single_digit(digits: &[u64], digit: u64, significance: u64) -> Vec<u64> {
    let mut result = vec![0;significance as usize];
    let mut msd: u64 = 0;
    let mut lsd: u64;

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

    /*
     ----------------------------------
     Test Karatsuba Multiplication
     ----------------------------------
     */
    #[test]
    fn test_karatsuba_big() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16];
        let b = [16, 15, 14, 13, 12, 11, 10, 9,
                 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(karatsuba(&a, &b), 
                   vec!(16, 47, 92, 150, 220, 301, 392, 492, 600,
                        715, 836, 962, 1092, 1225, 1360, 1496, 
                        1360, 1225, 1092, 962, 836, 715, 600, 
                        492, 392, 301, 220, 150, 92, 47, 16));
    }
}
