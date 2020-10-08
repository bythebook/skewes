use crate::natural::Natural;

#[derive(Debug,Eq,PartialEq)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug,Eq,PartialEq)]
pub struct Integer {
    sign: Sign,
    size: Natural,
}

