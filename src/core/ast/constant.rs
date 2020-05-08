use crate::core::ast::Calculatable;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::{map, map_res, opt};
use nom::sequence::{delimited, pair};
use nom::IResult;

#[derive(Eq, PartialEq, Debug)]
pub struct Constant {
    value: i64,
}

impl Constant {
    pub fn new(value: i64) -> Constant {
        Constant { value }
    }
}

impl Calculatable for Constant {
    fn calc(&self) -> i64 {
        self.value
    }
}

pub fn constant(input: &str) -> IResult<&str, Constant> {
    map(
        delimited(
            space0,
            pair(opt(tag("-")), map_res(digit1, |s: &str| s.parse::<i64>())),
            space0,
        ),
        |(minus, mut num)| {
            if minus.is_some() {
                num *= -1
            }
            Constant::new(num)
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_ok() {
        assert_eq!(constant("123"), Ok(("", Constant::new(123))));
        assert_eq!(constant(" 123 "), Ok(("", Constant::new(123))));
        assert_eq!(constant("-123"), Ok(("", Constant::new(-123))));
    }

    #[test]
    fn constant_err() {
        assert!(constant("abc").is_err());
        assert!(constant("- 123").is_err());
    }
}
