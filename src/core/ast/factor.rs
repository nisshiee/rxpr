use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::{delimited, pair};
use nom::IResult;

use crate::core::ast::{constant, expr};
use crate::core::ast::{Calculatable, Constant, Expr};
use crate::core::Num;

#[derive(Eq, PartialEq, Debug)]
pub enum Factor<N: Num> {
    Constant(Constant<N>),
    ExprInParen(Box<Expr<N>>),
}

impl<N: Num> Calculatable<N> for Factor<N> {
    fn calc(&self) -> N {
        match self {
            Factor::Constant(c) => c.calc(),
            Factor::ExprInParen(e) => e.calc(),
        }
    }
}

pub fn factor<N: Num>(input: &str) -> IResult<&str, Factor<N>> {
    alt((constant, expr_in_paren))(input)
}

fn constant<N: Num>(input: &str) -> IResult<&str, Factor<N>> {
    map(constant::constant::<N>, |c| Factor::Constant(c))(input)
}

fn expr_in_paren<N: Num>(input: &str) -> IResult<&str, Factor<N>> {
    map(
        delimited(pair(space0, tag("(")), expr::expr, pair(tag(")"), space0)),
        |e| Factor::ExprInParen(Box::new(e)),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factor_ok() {
        assert_eq!(
            factor::<i64>("123"),
            Ok(("", Factor::Constant(constant::constant("123").unwrap().1)))
        );
        assert_eq!(
            factor::<i64>(" (123 + 456) "),
            Ok((
                "",
                Factor::ExprInParen(Box::new(expr::expr("123 + 456").unwrap().1))
            ))
        );
    }

    #[test]
    fn factor_err() {
        assert!(factor::<i64>("abc").is_err());
    }
}
