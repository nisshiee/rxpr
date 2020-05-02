use crate::core::ast::{constant, expr};
use crate::core::ast::{Calculatable, Constant, Expr};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::{delimited, pair};
use nom::IResult;

#[derive(Eq, PartialEq, Debug)]
pub enum Factor {
    Constant(Constant),
    ExprInParen(Box<Expr>),
}

impl Calculatable for Factor {
    fn calc(&self) -> i64 {
        match self {
            Factor::Constant(c) => c.calc(),
            Factor::ExprInParen(e) => e.calc(),
        }
    }
}

pub fn factor(input: &str) -> IResult<&str, Factor> {
    alt((constant, expr_in_paren))(input)
}

fn constant(input: &str) -> IResult<&str, Factor> {
    map(constant::constant, |c: Constant| Factor::Constant(c))(input)
}

fn expr_in_paren(input: &str) -> IResult<&str, Factor> {
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
            factor("123"),
            Ok(("", Factor::Constant(constant::constant("123").unwrap().1)))
        );
        assert_eq!(
            factor(" (123 + 456) "),
            Ok((
                "",
                Factor::ExprInParen(Box::new(expr::expr("123 + 456").unwrap().1))
            ))
        );
    }

    #[test]
    fn factor_err() {
        assert!(factor("abc").is_err());
    }
}
