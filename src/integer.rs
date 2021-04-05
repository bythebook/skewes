use core::ops::{Add, Mul, Sub, Div};
use core::convert::From;

use crate::natural::Natural;
use crate::algorithms::{sub_signed, div};

use core::fmt;

//pub struct ParseError(String);

///
/// Represents the sign of an integer (Positive/Negative)
/// 
#[derive(Debug,Eq,PartialEq,Copy,Clone)]
pub enum Sign {
    /// Greater-than-or-equal-to zero
    Positive,
    /// Less-than zero
    Negative,
}

impl Sign {
    ///
    /// Negates sign
    /// 
    /// Example
    /// ```rust
    /// use skewes::Sign;
    /// assert_eq!(Sign::Positive.negate(), Sign::Negative);
    /// ```
    pub fn negate(self) -> Self {
        match self {
            Sign::Positive => Self::Negative,
            Sign::Negative => Self::Positive,
        }
    }
}

///
/// A type representing signed integers with arbitrary precision
/// 
/// # Example
/// ```rust
/// use skewes::Integer;
/// 
/// // Create an integer from a string
/// let z = Integer::from_string("-1234");
/// 
/// println!("z = {}", z);
/// ```
/// 
#[derive(Debug,Eq,PartialEq)]
pub struct Integer {
    sign: Sign,
    size: Natural,
}

impl Integer {
    /// 
    /// Parses a string and returns an integer
    /// 
    /// The API for this function is likely to change in future,
    /// with the addition of an error type for failed parsing
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
                    sign, 
                    size: Natural::from_string(&chars[count..])
                }
            }
        }
        Self {
            sign, 
            size: Natural::from(0),
        }
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
                    sign,
                    size,
                }
            },
            (Sign::Positive, Sign::Negative) => {
                let (sign, size) = sub_signed(&self.size, &other.size);
                Integer {
                    sign,
                    size,
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
            sign,
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

    // Clippy thinks it's suspicious to use the multiplication operator
    // in an implementation of division, but of course it's completely
    // legitimate here.
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: &Integer) -> Integer {
        let (d, _r) = div(&self.size, &other.size);
        Integer {
            sign: self.sign * other.sign,
            size: d,
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign_text = match self.sign {
            Sign::Positive => "",
            Sign::Negative => "-",
        };
        write!(f, "{}{}", sign_text, self.size)
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

    #[test]
    fn test_can_print_to_string() {
        let z = Integer::from_string("123456");
        assert_eq!(z.to_string(), "123456");

        let y = Integer::from_string("-54321");
        assert_eq!(y.to_string(), "-54321");
    }
}


