use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::IResult;

use crate::core::ast::Calculatable;
use crate::core::Num;

#[derive(Eq, PartialEq, Debug)]
pub struct Constant<N: Num> {
    value: N,
}

impl<N: Num> Constant<N> {
    pub fn new(value: N) -> Constant<N> {
        Constant { value }
    }
}

impl<N: Num> Calculatable<N> for Constant<N> {
    fn calc(&self) -> N {
        self.value
    }
}

pub fn constant<N: Num>(input: &str) -> IResult<&str, Constant<N>> {
    map(delimited(space0, N::constant, space0), |num| {
        Constant::new(num)
    })(input)
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
        assert!(constant::<i64>("abc").is_err());
        assert!(constant::<i64>("- 123").is_err());
    }
}
