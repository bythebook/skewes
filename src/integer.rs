use core::ops::{Add, Mul, Sub, Div};
use core::convert::From;

use crate::natural::Natural;
use crate::algorithms::sub_signed;

#[derive(Debug,Eq,PartialEq,Copy,Clone)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug,Eq,PartialEq)]
pub struct Integer {
    sign: Sign,
    size: Natural,
}

impl From<Natural> for Integer {
    fn from(n: Natural) -> Self {
        Self {
            sign: Sign::Positive,
            size: n,
        }
    }
}

impl Mul<Sign> for Sign {
    type Output = Self;

    fn mul(self, other: Self) -> Sign {
        match (self, other) {
            (Sign::Positive, Sign::Positive) => Sign::Positive,
            (Sign::Positive, Sign::Negative) => Sign::Negative,
            (Sign::Negative, Sign::Positive) => Sign::Negative,
            (Sign::Negative, Sign::Negative) => Sign::Positive,
        }
    }
}

impl Add<&Integer> for &Integer {
    type Output = Integer;

    fn add(self, other: &Integer) -> Integer {
        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Positive) => Integer {
                sign: Sign::Positive, 
                size: &self.size + &other.size
            },
            (Sign::Negative, Sign::Negative) => Integer {
                sign: Sign::Negative,
                size: &self.size + &other.size
            },
            (Sign::Negative, Sign::Positive) => {
                let (sign, size) = sub_signed(&other.size, &self.size);
                Integer {
                    sign: sign,
                    size: size,
                }
            },
            (Sign::Positive, Sign::Negative) => {
                let (sign, size) = sub_signed(&self.size, &other.size);
                Integer {
                    sign: sign,
                    size: size,
                }
            },
        }
    }
}

impl Mul<&Integer> for &Integer {
    type Output = Integer;

    fn mul(self, other: &Integer) -> Integer {
        Integer {
            sign: self.sign * other.sign,
            size: &self.size * &other.size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_positive() {
        let a = Integer{sign: Sign::Positive, size: Natural::from(1234)};
        let b = Integer{sign: Sign::Positive, size: Natural::from(123)};
        assert_eq!(&a + &b, Integer{sign: Sign::Positive, size: Natural::from(1357)});
    }

    #[test]
    fn test_add_positive_negative() {
        let a = Integer{sign: Sign::Positive, size: Natural::from(1234)};
        let b = Integer{sign: Sign::Negative, size: Natural::from(123)};
        assert_eq!(&a + &b, Integer{sign: Sign::Positive, size: Natural::from(1111)});
    }

    #[test]
    fn test_add_negative_positive() {
        let a = Integer{sign: Sign::Negative, size: Natural::from(1234)};
        let b = Integer{sign: Sign::Positive, size: Natural::from(123)};
        assert_eq!(&a + &b, Integer{sign: Sign::Negative, size: Natural::from(1111)});
    }

    #[test]
    fn test_add_negative_negative() {
        let a = Integer{sign: Sign::Negative, size: Natural::from(1234)};
        let b = Integer{sign: Sign::Negative, size: Natural::from(123)};
        assert_eq!(&a + &b, Integer{sign: Sign::Negative, size: Natural::from(1357)});
    }

    #[test]
    fn test_mul_positive_positive() {
        let a = Integer{sign: Sign::Positive, size: Natural::from(20)};
        let b = Integer{sign: Sign::Positive, size: Natural::from(10)};
        assert_eq!(&a * &b, Integer{sign: Sign::Positive, size: Natural::from(200)});
    }

    #[test]
    fn test_mul_positive_negative() {
        
    }
}


