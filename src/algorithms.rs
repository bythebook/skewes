use std::convert::TryFrom;
use std::cmp::Ordering;

use crate::Natural;
use crate::Sign;
use crate::division_result::DivisionResult;

pub fn add(a: &[u64], b: &[u64]) -> Vec<u64> {
    let first: &[u64];
    let second: &[u64];
    // Order numbers so that first is the one with the greater number of digits
    if a.len() < b.len() {
        first  = b;
        second = a;
    }
    else {
        first  = a;
        second = b;
    }

    let mut result = Vec::with_capacity(first.len()); 
    // Allocate the exact required f

    let mut firstiter = first.iter();
    let mut seconditer = second.iter();
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
                return result;
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

    result
}

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

fn sub_slice_assign(first: &mut [u64], second: &[u64]) {
    let mut carry: bool = false;
    let mut other_iter = second.iter();
    for digit in first.iter_mut() {
        match other_iter.next() {
            Some(other_digit) => {
                let (a, b) = sub_with_carry(*digit, *other_digit, carry);
                carry = b;
                *digit = a;
            },
            None => {
                let (a, b) = sub_with_carry(*digit, 0, carry);
                carry = b;
                *digit = a;
            }
        }
    }
}

fn sub_slice(first: &[u64], second: &[u64]) -> Vec<u64> {
    let mut carry: bool = false;
    let mut result: Vec<u64> = Vec::new(); // TODO can size this
    let mut other_iter = second.iter();
    for digit in first.iter() {
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

    if result.last() == Some(&0) {
        result.pop();
    }

    result
}

pub fn sub_signed(first: &Natural, second: &Natural) -> (Sign, Natural) {
    match cmp_slice(&first.digits, &second.digits) {
        Ordering::Greater => (Sign::Positive, Natural::from(sub_slice(&first.digits, &second.digits))),
        Ordering::Equal => (Sign::Positive, Natural::from(sub_slice(&first.digits, &second.digits))),
            // The reason I haven't returned zero directly here is that I don't want to check for 
            // the special case within the division algorithm
        Ordering::Less => (Sign::Negative, Natural::from(sub_slice(&second.digits, &first.digits))),
    }
}

pub fn div(p: &Natural, q: &Natural) -> (Natural, Natural) {
    let msd = q.digits.last().unwrap();
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
    println!("p: {:?}, q: {:?}", p, q);
    let mut a = p.clone();
    let n = q.digits.len();
    if n > a.digits.len() {
        (Natural::zero(), q.clone())
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
            println!("j: {:?}", j);
            println!("a: {:?}", a);
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

pub fn shl(p: &Natural, n: usize) -> Natural {
    let mut q = p.clone();
    shl_mut(&mut q, n);
    q
}

pub fn shl_mut(p: &mut Natural, n: usize) {
    for _ in 1..(n+1) { // n times
        p.digits.insert(0, 0); // Insert a zero in the first position (least significant digit)
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

// Normalize


pub fn cmp_slice(first: &[u64], second: &[u64]) -> Ordering {
    if digit_length(first) > digit_length(second) {
        return Ordering::Greater
    }
    else if digit_length(first) < digit_length(second) {
        return Ordering::Less
    }
    else {
        let iter = first.iter().zip(second.iter());
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

pub fn digit_length(digits: &[u64]) -> usize {
    let mut l = digits.len();
    for digit in digits.iter().rev() {
        if *digit == 0 { l -= 1; }
        else { break; }
    }
    l
}


// Multiply a slice by a single u64 digit, allowing for significance number of zeroes at the start
// Used to build up longhand multiplication
fn mul_by_single_digit(digits: &[u64], digit: u64, significance: u64) -> Vec<u64> {
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

fn normalize(n: &mut Natural) {
    while let Some(&0) = n.digits.last() {
        n.digits.pop();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const NINE: u64 = u64::MAX;
    const EIGHT: u64 = u64::MAX - 1;
    const ONE: u64 = 1;

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
    fn test_divide_two_normalised_numbers() {
        let a = Natural::from(vec!(1<<63, 1<<63));
        let b = Natural::from(vec!(1<<63));
        let c = Natural::from(vec!(1, 1));
        assert_eq!(div(&a, &b), (c, Natural::zero()));
    }

    #[test]
    fn test_divide_two_numbers() {
        let a = Natural::from(vec!(1, 7));
        let b = Natural::from(vec!(7));
        let c = Natural::from(vec!(0, 1));
        assert_eq!(div(&a, &b), (c, Natural::one()));
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

    #[test]
    fn test_mul_by_power_2() {
        let n = Natural::from(vec!(1, 7));
        assert_eq!(mul_by_2_to_power_k(&n, 3), Natural::from(vec!(8, 56)));

        let n = Natural::from(156);
        assert_eq!(mul_by_2_to_power_k(&n, 60), Natural::from(vec!(156 << 60, 156u64.rotate_left(60) & 0xff)))
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
    fn test_cmp_slice_more_digits() {
        let a = vec!(1, 2); // i.e. (21) base 2**64
        let b = vec!(5);
        assert_eq!(cmp_slice(&a, &b), Ordering::Greater);
    }

    #[test]
    fn test_cmp_slice_fewer_digits() {
        let a = vec!(567);
        let b = vec!(1, 1);
        assert_eq!(cmp_slice(&a, &b), Ordering::Less);
    }

    #[test]
    fn test_cmp_slice_greater() {
        let a = vec!(3, 5);
        let b = vec!(1, 5);
        assert_eq!(cmp_slice(&a, &b), Ordering::Greater);
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

    #[test]
    fn test_digit_length() {
        let a = Natural::from(vec!(0, 0, 1));
        let b = Natural::from(vec!(1, 2, 3));
        let c = Natural::from(vec!(1, 2, 0, 0));
        let d = Natural::from(vec!(1, 0, 3, 4));
        assert_eq!(digit_length(&a.digits), 3);
        assert_eq!(digit_length(&b.digits), 3);
        assert_eq!(digit_length(&c.digits), 2);
        assert_eq!(digit_length(&d.digits), 4);
    }

    #[test]
    fn test_div_by_ten() {
        let a = Natural::from(156);
        let b = Natural::from(10);
        let (d, r) = div(&a, &b);
        assert_eq!(d, Natural::from(15));
        assert_eq!(r, Natural::from(6));
    }
}
