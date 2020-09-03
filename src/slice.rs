use std::convert::TryFrom;
use std::cmp::Ordering;

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
    // Allocate the exact required 

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

pub fn sub(first: &[u64], second: &[u64]) -> Option<Vec<u64>> {
    let mut carry: bool = false;
    let mut result: Vec<u64> = Vec::new(); // TODO can size this
    let mut other_iter = second.iter();
    if cmp(first, second) == Ordering::Less {
        return None
    }
    else {
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
        if carry {
            return None // Can't happen, as we checked for one bigger than the other
        }
    }
    normalize(&mut result);
    Some(result)
}

pub fn div(p: &[u64], q: &[u64]) -> (Vec<u64>, Vec<u64>) {
    let mut result: Vec<u64> = Vec::new();
    let mut remainder: Vec<u64>;
    // [1, 2] / [7]
    // p_digits = 2
    // q_digits = 1
    // num_digits = 1
    // numerator = [1, 2]@[1..2] = [2]
    // short_div => (0, [2])
    // b = [2]
    // result = [0]
    // carry = [p[0]] = [1]
    // carry = [1].extend([2]) = [1, 2]
    // numerator = [1,2]
    // short_div => 
    let p_digits = p.len();
    let q_digits = q.len();
    let mut carry: Vec<u64>;
    let mut num_digits = q_digits;
    let mut numerator = &p[p_digits-num_digits..p_digits].to_vec();
    loop {
        // Divide
        let (a, b) = short_div(&numerator, q);
        
        // Update state
        result.push(a); // Can result in leading zeroes
        remainder = b.clone();
        num_digits += 1;
        if num_digits > p_digits {
            break;
        }
        carry = vec!(p[p_digits-num_digits]);
        carry.extend(b);
        numerator = &carry;
    }

    let ret: Vec<u64> = result.into_iter().rev().collect();
    (ret, remainder)
}

// Idea: have this operate on slices of u64 instead, then can reuse data when performing long division
pub fn short_div(p: &[u64], q: &[u64]) -> (u64, Vec<u64>) {
    // vec!(20, 3) / vec!(7)
    let mut d = 0;
    let mut candidate = q.clone().to_vec();
    loop {
        println!("Trying: {:?}, Divisor: {}", candidate, d);
        match sub(p, &candidate) {
            None => break,
            Some(_) => {
                candidate = add(&candidate, q); //TODO: in-place addition
                d += 1;
            },
        }
    }
    let rem = sub(p, &sub(&candidate, q).unwrap()).unwrap();
    (d, rem) // Guaranteed to be safe TODO: add unchecked minus
}

// Normalize



pub fn cmp(first: &[u64], second: &[u64]) -> Ordering {
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

fn normalize(digits: &mut Vec<u64>) -> () {
    // There might be a better way to do this. Using iterators, can't delete from vector while iterating.
    // See contain-rs's 'Cursor' trait for a possible alternative to iterators
    let mut last_index = 1; // Set last_index to 1 so that if a different answer isn't found, we keep least significant zero
    for (index, digit) in digits.iter().rev().enumerate() {
        if *digit > 0 { 
            last_index = digits.len() - index; // Remember that this is a reverse iterator
            break;
        }
    }
    for _ in last_index..digits.len() {
        digits.pop();
    }
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
    fn test_divide_two_numbers() {
        let a = vec!(1, 7);
        let b = vec!(7);
        let c = vec!(0, 1);
        assert_eq!(div(&a, &b), (c, vec!(1)));
    }

    #[test]
    fn test_divide_two_numbers_with_carry() {
        let a = vec!(20, 3);
        let b = vec!(7);
        let (d, r) = short_div(&a, &b);
        println!("Divisor: {}, remainder: {:?}", d, r);
        assert_eq!(div(&a, &b), (vec!(d), r));

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
    fn test_short_div_multiple_digits() {
        const SEVEN: u64 = u64::MAX - 2;
        let a = [u64::MAX - 8, 2];
        let b = [SEVEN];
        assert_eq!(short_div(&a, &b), (3, vec!(0)));

        let a = vec!(20, 3);
        let b = vec!(7);
        let (d, r) = short_div(&a, &b);
        println!("Divisor: {}, remainder: {:?}", d, r);
        assert_eq!(0, 1);
    }

    // 3 * SEVEN = 3 * (BASE - 3) = 3 * BASE - 9 = 2 * BASE + (BASE - 9)


    #[test]
    fn test_short_div() {
        let a = [234];
        let b = [123];
        let c = [14];
        assert_eq!(short_div(&a, &b), (1, vec!(111)));
        assert_eq!(short_div(&c, &b), (0, vec!(14)));
        assert_eq!(short_div(&a, &c), (16, vec!(10)));
    }

    #[test]
    fn test_normalize() {
        let mut a = vec!(1, 0, 2, 0, 0);
        let mut b = vec!(0, 0);
        let mut c = vec!(1, 0, 2);

        normalize(&mut a);
        normalize(&mut b);
        normalize(&mut c);

        assert_eq!(a, vec!(1, 0, 2));
        assert_eq!(b, vec!(0));
        assert_eq!(c, vec!(1, 0, 2));
    }

}
