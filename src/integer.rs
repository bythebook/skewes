use core::ops::{Add, Mul, Sub, Div};
use core::convert::From;

use crate::natural::Natural;
use crate::algorithms::{sub_signed, div};

//pub struct ParseError(String);

#[derive(Debug,Eq,PartialEq,Copy,Clone)]
pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    pub fn negate(self) -> Self {
        match self {
            Sign::Positive => Self::Negative,
            Sign::Negative => Self::Positive,
        }
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct Integer {
    sign: Sign,
    size: Natural,
}

impl Integer {
    pub fn from_string<S: Into<String>>(s: S) -> Self {
        let mut sign = Sign::Positive;
        let chars = s.into();
        for (count, ch) in chars.chars().enumerate() {
            if ch == '-' {
                sign = sign.negate();
            }
            else if ch == ' ' {
                // Do nothing
            }
            else if ch.is_digit(10) {
                return Self {
                    sign: sign, 
                    size: Natural::from_string(&chars[count..])
                }
            }
        }
        return Self{
            sign: sign, 
            size: Natural::from(0),
        };
    }
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

impl Sub<&Integer> for &Integer {
    type Output = Integer;

    fn sub(self, other: &Integer) -> Integer {
        let (sign, result) = match(self.sign, other.sign) {
            (Sign::Positive, Sign::Positive) => sub_signed(&self.size, &other.size),
            (Sign::Negative, Sign::Negative) => sub_signed(&other.size, &self.size),
            (Sign::Positive, Sign::Negative) => (Sign::Positive, &self.size + &other.size),
            (Sign::Negative, Sign::Positive) => (Sign::Negative, &self.size + &other.size),
        };
        Integer {
            sign: sign,
            size: result,
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

impl Div<&Integer> for &Integer {
    type Output = Integer;

    fn div(self, other: &Integer) -> Integer {
        let (d, _r) = div(&self.size, &other.size);
        Integer {
            sign: self.sign * other.sign,
            size: d,
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
    fn test_sub_positive_positive_eq_positive() {
        let a = Integer{sign: Sign::Positive, size: Natural::from(400)};
        let b = Integer{sign: Sign::Positive, size: Natural::from(100)};
        assert_eq!(&a - &b, Integer{sign: Sign::Positive, size: Natural::from(300)});
    }

    #[test]
    fn test_sub_positive_positive_eq_negative() {
        let a = Integer{sign: Sign::Positive, size: Natural::from(100)};
        let b = Integer{sign: Sign::Positive, size: Natural::from(400)};
        assert_eq!(&a - &b, Integer{sign: Sign::Negative, size: Natural::from(300)});
    }

    #[test]
    fn test_sub_positive_negative() {
        let a = Integer{sign: Sign::Positive, size: Natural::from(400)};
        let b = Integer{sign: Sign::Negative, size: Natural::from(100)};
        assert_eq!(&a - &b, Integer{sign: Sign::Positive, size: Natural::from(500)});
        
        let a = Integer{sign: Sign::Positive, size: Natural::from(100)};
        let b = Integer{sign: Sign::Negative, size: Natural::from(400)};
        assert_eq!(&a - &b, Integer{sign: Sign::Positive, size: Natural::from(500)});
    }

    #[test]
    fn test_sub_negative_positive() {
        let a = Integer{sign: Sign::Negative, size: Natural::from(400)};
        let b = Integer{sign: Sign::Positive, size: Natural::from(100)};
        assert_eq!(&a - &b, Integer{sign: Sign::Negative, size: Natural::from(500)});

        let a = Integer{sign: Sign::Negative, size: Natural::from(100)};
        let b = Integer{sign: Sign::Positive, size: Natural::from(400)};
        assert_eq!(&a - &b, Integer{sign: Sign::Negative, size: Natural::from(500)});
    }

    #[test]
    fn test_sub_negative_negative_eq_positive() {
        let a = Integer{sign: Sign::Negative, size: Natural::from(100)};
        let b = Integer{sign: Sign::Negative, size: Natural::from(400)};
        assert_eq!(&a - &b, Integer{sign: Sign::Positive, size: Natural::from(300)});
    }

    #[test]
    fn test_sub_negative_negative_eq_negative() {
        let a = Integer{sign: Sign::Negative, size: Natural::from(400)};
        let b = Integer{sign: Sign::Negative, size: Natural::from(100)};
        assert_eq!(&a - &b, Integer{sign: Sign::Negative, size: Natural::from(300)});
    }

    #[test]
    fn test_mul_positive_positive() {
        let a = Integer{sign: Sign::Positive, size: Natural::from(20)};
        let b = Integer{sign: Sign::Positive, size: Natural::from(10)};
        assert_eq!(&a * &b, Integer{sign: Sign::Positive, size: Natural::from(200)});
    }

    #[test]
    fn test_mul_positive_negative() {
        let a = Integer{sign: Sign::Positive, size: Natural::from(20)};
        let b = Integer{sign: Sign::Negative, size: Natural::from(10)};
        assert_eq!(&a * &b, Integer{sign: Sign::Negative, size: Natural::from(200)});
    }

    #[test]
    fn test_can_parse_string() {
        let one_hundred = Integer::from_string("100");
        let minus_five = Integer::from_string("-5");
        assert_eq!(one_hundred, Integer{sign: Sign::Positive, size: Natural::from(100)});
        assert_eq!(minus_five, Integer{sign: Sign::Negative, size: Natural::from(5)});
    }
}


