use crate::Natural;
use crate::Sign;

use crate::division_result::DivisionResult;
use super::multiplication::mul_by_single_digit;
use super::subtraction::{sub_signed,sub_slice_assign};
use super::util::shl;

#[inline]
pub fn div(p: &Natural, q: &Natural) -> (Natural, Natural) {
    if *q == Natural::ZERO {
        panic!("Divide by zero");
    }
    let msd = q.digits.last().unwrap(); // Unwrap works when non-zero
    let zeroes = msd.leading_zeros();
    if zeroes == 0 {
        div_normalised(p, q)
    }
    else {
        let new_p = mul_by_2_to_power_k(p, zeroes);
        let new_q = mul_by_2_to_power_k(q, zeroes);
        let (d, r) = div_normalised(&new_p, &new_q);
        (d, div_by_2_to_power_k(&r, zeroes))
    }
}

#[inline]
pub fn mul_by_2_to_power_k(n: &Natural, k: u32) -> Natural {
    let mut carry = 0;
    let low_mask  = (1<<k) - 1;
    let high_mask = u64::MAX ^ low_mask;
    let mut new_digits = Vec::with_capacity(n.digits.len());
    for digit in n.digits.iter() {
        let result = digit.rotate_left(k);
        let new_carry = result & low_mask;
        new_digits.push(result & high_mask | carry);
        carry = new_carry;
    }
    if carry != 0 {
        new_digits.push(carry);
    }
    Natural::from(new_digits)
}

pub fn div_by_2_to_power_k(n: &Natural, k: u32) -> Natural {
    let mut carry = 0;
    let low_mask  = (1<<(64-k)) - 1;
    let high_mask = u64::MAX ^ low_mask;
    let mut new_digits = DivisionResult::new(n.digits.len());
    for digit in n.digits.iter().rev() {
        let result = digit.rotate_right(k);
        let new_carry = result & high_mask;
        new_digits.push(result & low_mask | carry);
        carry = new_carry;
    }
    let mut result = Natural::from(new_digits);
    if carry != 0 {
        result.digits.push(carry);
    }
    result
}


/// Returns the result (quotient, remainder) of p / q
/// 
/// q must be normalised
fn div_normalised(p: &Natural, q: &Natural) -> (Natural, Natural) {
    // This is a mess
    let mut a = p.clone();
    let n = q.digits.len();
    if n > a.digits.len() {
        (Natural::ZERO, q.clone())
    }
    else {
        let m = a.digits.len() - n;
        let mut digits = DivisionResult::new(m+1);
        let first_candidate = shl(&q, m);
        if a >= first_candidate {
            digits.push(1);
            sub_slice_assign(&mut a.digits, &first_candidate.digits);
        }
        else {
            digits.push(0);
        }

        for j in (0..m).rev() {
            // Need to handle cases where we've had a near-exact division here
            if a.digits.len() == 1 {
                let last_digit = a.digits[0] / q.digits[n-1];
                digits.push(last_digit);
            }
            if a == Natural::ZERO {
                // Can we put all the remaining zeroes in at once?
                digits.push(0);
                continue;
            }

            // Otherwise we have lots of work to do
            let mut q_j = short_div(a.digits[n+j], a.digits[n+j-1], q.digits[n-1]);
            let mut sign: Sign;
            let (s, temp_a) = sub_signed(&a, &Natural::from(mul_by_single_digit(&q.digits, q_j, j as u64)));
            sign = s;
            a = temp_a;
            while sign == Sign::Negative {
                q_j -= 1;
                let (s, temp_a) = sub_signed(&Natural::from(shl(&q, j)), &a);
                sign = s;
                a = temp_a;
            }
            digits.push(q_j);
        }

        let mut n = Natural::from(digits);
        normalize(&mut n);
        normalize(&mut a);
        (n, a)
    }
}

/// Returns the result of min(floor[(p_1 * Base + p_0) / q_0], Base - 1)
/// 
/// This function is used as an intermediate step in long division of numbers with arbitrary digit counts
/// It represents the results of dividing a two-digit number by a single-digit number
fn short_div(p_1: u64, p_0: u64, q_0: u64) -> u64 {
    let p: u128 = ((p_1 as u128) << 64) |   (p_0 as u128);
    let q: u128 = q_0 as u128;
    let d = p / q;
    if d >> 64 != 0 {
        u64::MAX
    }
    else {
        // Ok to truncate as we've checked above
        d as u64
    }
}



fn normalize(n: &mut Natural) {
    while let Some(&0) = n.digits.last() {
        n.digits.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::convert::TryFrom;

    const NINE: u64 = u64::MAX;
    const EIGHT: u64 = u64::MAX - 1;

    #[test]
    fn test_mul_by_power_2() {
        let n = Natural::from(vec!(1, 7));
        assert_eq!(mul_by_2_to_power_k(&n, 3), Natural::from(vec!(8, 56)));

        let n = Natural::from(156);
        assert_eq!(mul_by_2_to_power_k(&n, 60), Natural::from(vec!(156 << 60, 156u64.rotate_left(60) & 0xff)))
    }

    #[test]
    fn test_divide_two_normalised_numbers() {
        let a = Natural::from(vec!(1<<63, 1<<63));
        let b = Natural::from(vec!(1<<63));
        let c = Natural::from(vec!(1, 1));
        assert_eq!(div(&a, &b), (c, Natural::ZERO));
    }

    #[test]
    fn test_divide_two_numbers() {
        let a = Natural::from(vec!(1, 7));
        let b = Natural::from(vec!(7));
        let c = Natural::from(vec!(0, 1));
        assert_eq!(div(&a, &b), (c, Natural::from(1)));
    }

    #[test]
    fn test_short_div_big_answer() {
        let p = (4u128 << 64) | (4u128);
        let q = 2u128;
        let d = p/q;
        let d64 = u64::try_from(d & 0xffffffffffffffff).unwrap();
        assert_eq!(d, (2u128 << 64) | 2u128);
        assert_eq!(d64, 2u64);


        let result = short_div(4, 4, 2);
        assert_eq!(result, NINE); // Answer bigger than 1 digit, therefore max 9
    }

    #[test]
    fn test_short_div_big_divisor_small_answer() {
        let result = short_div(1, 2, 2);
        let expected = (1u64<<63) + 1;
        assert_eq!(result, expected);
    }


    #[test]
    fn test_short_div_small_exact_answer() {
        let result = short_div(0, 21, 7);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_short_div_small_floored_answer() {
        let result = short_div(0, 23, 7);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_normalize() {
        let mut a = Natural::from(vec!(1, 0, 0));
        let mut b = Natural::from(vec!(0, 0, 2));
        let mut c = Natural::from(vec!(0, 3, 0));

        normalize(&mut a);
        normalize(&mut b);
        normalize(&mut c);

        assert_eq!(a, Natural::from(vec!(1)));
        assert_eq!(b, Natural::from(vec!(0, 0, 2)));
        assert_eq!(c, Natural::from(vec!(0, 3)));
    }

    #[test]
    fn test_div_by_ten() {
        let a = Natural::from(156);
        let b = Natural::from(10);
        let (d, r) = div(&a, &b);
        assert_eq!(d, Natural::from(15));
        assert_eq!(r, Natural::from(6));
    }

    #[test]
    fn test_div_three_digit_by_one_digit() {
        // This test case arose in calculating factorial(100) and trying to print
        // The regression in this case was that the test result was actually 0, rather
        // than the expected here.
        let p = Natural::from(vec!(0, 0, 4788272403190906880));
        let q = Natural::from(11529215046068469760);
        assert_eq!(p.div(&q), (Natural::from(vec!(0, 7661235845105451008)), 
                               Natural::ZERO)
        );
    }
}