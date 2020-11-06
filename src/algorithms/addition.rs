#[inline]
pub fn add(a: &[u64], b: &[u64]) -> Vec<u64> {
    // Order numbers so that first is the one with the greater number of digits
    if a.len() < b.len() {
        _add(b, a)
    }
    else {
        _add(a, b)
    }
}

#[inline]
fn _add(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut result = Vec::with_capacity(a.len()); 
    // Allocate the exact required f

    let mut firstiter = a.iter();
    let mut seconditer = b.iter();
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
                result.push(a);
            },
            None => {
                if carry {
                    result.push(1);
                }
                break;
            }
        }
    }

    result
}

#[inline]
pub fn add_mut(a: &mut [u64], b: &[u64]) -> bool {
    let mut firstiter = a.iter_mut();
    let mut seconditer = b.iter();
    let mut carry = false;

    // Add digits of the second number to the first
    loop {
        match seconditer.next() {
            Some(seconddigit) => {
                let firstdigit = firstiter.next().unwrap();
                let (a, b) = add_with_carry(*firstdigit, *seconddigit, carry);
                carry = b;
                *firstdigit = a;
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
                *firstdigit = a;
            },
            None => break,
        }
    }

    carry
}

fn add_with_carry(digita: u64, digitb: u64, prev_carry: bool) -> (u64, bool) {
    let (resultdigit, new_carry) = digita.overflowing_add(digitb);
    let carrydigit = prev_carry as u64;
    match new_carry {
        true => (resultdigit + carrydigit, true),
        false => resultdigit.overflowing_add(carrydigit),
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

    /*
    -----------------------------------------
    Test slice addition
    -----------------------------------------
    */

    #[test]
    fn add_mut_same_size_numbers() {
        let mut a = [1, 2, 3];
        let b = [4, 5, 6];
        let c = [5, 7, 9];

        add_mut(&mut a, &b);
        assert_eq!(a, c);
    }

    #[test]
    fn add_mut_same_size_numbers_w_carry() {
        let mut a = [NINE, 2, 3];
        let b = [NINE, 5, 6];
        let c = [EIGHT, 8, 9];

        add_mut(&mut a, &b);
        assert_eq!(a, c);
    }

    #[test]
    fn add_mut_different_size_numbers() {
        let mut a = [1, 2, 3, 4, 5];
        let b = [4, 5, 6];
        let c = [5, 7, 9, 4, 5];

        add_mut(&mut a, &b);
        assert_eq!(a, c);
    }

    #[test]
    fn add_mut_different_size_numbers_w_carry() {
        let mut a = [1, 2, NINE, 4, 5];
        let b = [4, 5, NINE];
        let c = [5, 7, EIGHT, 5, 5];

        add_mut(&mut a, &b);
        assert_eq!(a, c);
    }
}