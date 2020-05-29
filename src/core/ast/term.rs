use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, map_res};
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;

use crate::core::ast::{factor, Calculatable, Factor};
use crate::core::Num;

#[derive(Eq, PartialEq, Debug)]
pub enum Term<N: Num> {
    Factors {
        head: Factor<N>,
        tail: Vec<FactorWithOperator<N>>,
    },
}

impl<N: Num> Calculatable<N> for Term<N> {
    fn calc(&self) -> N {
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

pub fn term<N: Num>(input: &str) -> IResult<&str, Term<N>> {
    map(
        pair(factor::factor::<N>, many0(factor_with_operator::<N>)),
        |(head, tail)| Term::Factors { head, tail },
    )(input)
}

#[derive(Eq, PartialEq, Debug)]
pub struct FactorWithOperator<N: Num> {
    operator: Operator,
    factor: Factor<N>,
}

fn factor_with_operator<N: Num>(input: &str) -> IResult<&str, FactorWithOperator<N>> {
    map(pair(operator, factor::factor::<N>), |(operator, factor)| {
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
            factor_with_operator::<i64>("* 123"),
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
            term::<i64>("123"),
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
        assert!(term::<i64>("abc").is_err());
    }
}
