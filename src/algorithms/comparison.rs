use core::cmp::Ordering;

/// Compares two little-endian slices representing natural numbers
/// 
/// Expects that the numbers passed are normalised to have no leading zeroes
pub fn cmp_slice(first: &[u64], second: &[u64]) -> Ordering {
    match first.len().cmp(&second.len()) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        _ => {
            let iter = first.iter().zip(second.iter());
            for (digit, other_digit) in iter.rev() {
                match (*digit).cmp(other_digit) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    _ => continue,
                }
            }
            Ordering::Equal
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
}