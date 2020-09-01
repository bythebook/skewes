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
    Some(result)
}

pub fn cmp(first: &[u64], second: &[u64]) -> Ordering {
    if first.len() > second.len() {
        return Ordering::Greater
    }
    else if first.len() < second.len() {
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

}
