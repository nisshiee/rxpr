use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, opt};
use nom::sequence::{pair, tuple};
use nom::IResult;
use std::fmt::Display;

pub trait Num:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + FromStr
    + Copy
    + Display
{
    fn constant(input: &str) -> IResult<&str, Self>;
}

impl Num for i64 {
    fn constant(input: &str) -> IResult<&str, Self> {
        map_res(pair(opt(tag("-")), digit1), |(sign, digit)| {
            Self::from_str(&([sign.unwrap_or(""), digit].concat()))
        })(input)
    }
}

impl Num for f64 {
    fn constant(input: &str) -> IResult<&str, Self> {
        let pattern = tuple((opt(tag("-")), digit1, opt(pair(tag("."), digit1))));
        map_res(pattern, |(sign, int, decimal)| {
            let decimal = match decimal {
                None => "".to_string(),
                Some((_, digit)) => [".", digit].concat(),
            };
            Self::from_str(&([sign.unwrap_or(""), int, &decimal].concat()))
        })(input)
    }
}
