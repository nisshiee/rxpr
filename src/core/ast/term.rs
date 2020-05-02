use crate::core::ast::{factor, Calculatable, Factor};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, map_res};
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;

#[derive(Eq, PartialEq, Debug)]
pub enum Term {
    Factors {
        head: Factor,
        tail: Vec<FactorWithOperator>,
    },
}

impl Calculatable for Term {
    fn calc(&self) -> i64 {
        match self {
            Term::Factors { head, tail } => tail.into_iter().fold(head.calc(), |a, e| match e {
                FactorWithOperator {
                    operator: Operator::Multiply,
                    factor: f,
                } => a * f.calc(),
                FactorWithOperator {
                    operator: Operator::Divide,
                    factor: f,
                } => a / f.calc(),
            }),
        }
    }
}

pub fn term(input: &str) -> IResult<&str, Term> {
    map(
        pair(factor::factor, many0(factor_with_operator)),
        |(head, tail)| Term::Factors { head, tail },
    )(input)
}

#[derive(Eq, PartialEq, Debug)]
pub struct FactorWithOperator {
    operator: Operator,
    factor: Factor,
}

fn factor_with_operator(input: &str) -> IResult<&str, FactorWithOperator> {
    map(pair(operator, factor::factor), |(operator, factor)| {
        FactorWithOperator { operator, factor }
    })(input)
}

#[derive(Eq, PartialEq, Debug)]
pub enum Operator {
    Multiply,
    Divide,
}

fn operator(input: &str) -> IResult<&str, Operator> {
    map_res(alt((tag("*"), tag("/"))), |tag: &str| match tag {
        "*" => Ok(Operator::Multiply),
        "/" => Ok(Operator::Divide),
        _ => Err(()),
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_ok() {
        assert_eq!(operator("*"), Ok(("", Operator::Multiply)));
        assert_eq!(operator("/"), Ok(("", Operator::Divide)));
    }

    #[test]
    fn operator_err() {
        assert!(operator("a").is_err());
    }

    #[test]
    fn factor_with_operator_ok() {
        assert_eq!(
            factor_with_operator("* 123"),
            Ok((
                "",
                FactorWithOperator {
                    operator: Operator::Multiply,
                    factor: factor::factor("123").unwrap().1,
                }
            ))
        );
    }

    #[test]
    fn term_ok() {
        assert_eq!(
            term("123"),
            Ok((
                "",
                Term::Factors {
                    head: factor::factor("123").unwrap().1,
                    tail: vec![]
                }
            )),
        );
    }

    #[test]
    fn term_err() {
        assert!(term("abc").is_err());
    }
}
