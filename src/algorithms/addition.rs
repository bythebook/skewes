use crate::Limb;

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

/// Add together two slices, allocating a new vector to hold the result.
#[inline]
fn _add(a: &[Limb], b: &[Limb]) -> Vec<Limb> {
    // We try not to do too many carries or two much branching here, so we just allocate
    // a vector at least one bigger than it needs to be
    // It might be good to revisit this in future and see if this is optimal for, e.g.
    // repeated additions. (For these, a pattern of mutable addition may be best anyway)
    let mut result = Vec::with_capacity(a.len() + 1);
    let mut firstiter = a.iter();
    let seconditer = b.iter();
    let mut carry = false;


    // Add digits of the second number to the first
    for second_digit in seconditer {
        let firstdigit = firstiter.next().unwrap();
        let (a, b) = add_with_carry(*firstdigit, *second_digit, carry);
        carry = b;
        result.push(a);
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

/// Add a and b, storing the result in a
/// 
/// Contract: add_mut expects a to be sized so that it has at least enough room
/// to hold the result of a + b
#[inline]
pub fn add_mut(a: &mut [Limb], b: &[Limb]) -> bool {
    let mut firstiter = a.iter_mut();
    let seconditer = b.iter();
    let mut carry = false;

    // Add digits of the second number to the first
    for second_digit in seconditer {
        let firstdigit = firstiter.next().unwrap();
        let (a, b) = add_with_carry(*firstdigit, *second_digit, carry);
        carry = b;
        *firstdigit = a;
    }

    // Propagate any left over carries from the second number to the first
    for first_digit in firstiter { // Beginning at current position in first
        let (a, b) = add_with_carry(*first_digit, 0, carry);
        carry = b;
        *first_digit = a;
    }

    carry
}

fn add_with_carry(digita: Limb, digitb: Limb, prev_carry: bool) -> (Limb, bool) {
    let (resultdigit, new_carry) = digita.overflowing_add(digitb);
    let carrydigit = prev_carry as Limb;
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