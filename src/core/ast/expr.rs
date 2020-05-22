use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, map_res};
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;

use crate::core::ast::{term, Calculatable, Term};
use crate::core::Num;

#[derive(Eq, PartialEq, Debug)]
pub enum Expr<N: Num> {
    Terms {
        head: Term<N>,
        tail: Vec<TermWithOperator<N>>,
    },
}

impl<N: Num> Calculatable<N> for Expr<N> {
    fn calc(&self) -> N {
        match self {
            Expr::Terms { head, tail } => tail.into_iter().fold(head.calc(), |a, t| a + t.calc()),
        }
    }
}

pub fn expr<N: Num>(input: &str) -> IResult<&str, Expr<N>> {
    map(
        pair(term::term::<N>, many0(term_with_operator::<N>)),
        |(head, tail)| Expr::Terms { head, tail },
    )(input)
}

#[derive(Eq, PartialEq, Debug)]
pub struct TermWithOperator<N: Num> {
    operator: Operator,
    term: Term<N>,
}

impl<N: Num> Calculatable<N> for TermWithOperator<N> {
    fn calc(&self) -> N {
        let ret = self.term.calc();
        match self.operator {
            Operator::Plus => ret,
            Operator::Minus => -ret,
        }
    }
}

fn term_with_operator<N: Num>(input: &str) -> IResult<&str, TermWithOperator<N>> {
    map(pair(operator, term::term::<N>), |(operator, term)| {
        TermWithOperator { operator, term }
    })(input)
}

#[derive(Eq, PartialEq, Debug)]
pub enum Operator {
    Plus,
    Minus,
}

fn operator(input: &str) -> IResult<&str, Operator> {
    map_res(alt((tag("+"), tag("-"))), |tag: &str| match tag {
        "+" => Ok(Operator::Plus),
        "-" => Ok(Operator::Minus),
        _ => Err(()),
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_ok() {
        assert_eq!(operator("+"), Ok(("", Operator::Plus)));
        assert_eq!(operator("-"), Ok(("", Operator::Minus)));
    }

    #[test]
    fn operator_err() {
        assert!(operator("a").is_err());
    }

    #[test]
    fn term_with_operator_ok() {
        assert_eq!(
            term_with_operator::<i64>("+ 123"),
            Ok((
                "",
                TermWithOperator {
                    operator: Operator::Plus,
                    term: term::term("123").unwrap().1,
                }
            ))
        );
    }

    #[test]
    fn term_with_operator_err() {
        assert!(term_with_operator::<i64>("123").is_err());
        assert!(term_with_operator::<i64>("+ ").is_err());
    }

    #[test]
    fn expr_ok() {
        assert_eq!(
            expr::<i64>("1"),
            Ok((
                "",
                Expr::Terms {
                    head: term::term("1").unwrap().1,
                    tail: vec![]
                }
            ))
        );
        assert_eq!(
            expr::<i64>("1 + 2 - 3"),
            Ok((
                "",
                Expr::Terms {
                    head: term::term("1").unwrap().1,
                    tail: vec![
                        TermWithOperator {
                            operator: Operator::Plus,
                            term: term::term("2").unwrap().1,
                        },
                        TermWithOperator {
                            operator: Operator::Minus,
                            term: term::term("3").unwrap().1,
                        }
                    ]
                }
            ))
        );
    }

    #[test]
    fn expr_err() {
        assert!(expr::<i64>("+").is_err());
    }

    #[test]
    fn integration() {
        assert_eq!(expr::<i64>("(1 + 2) * 3").unwrap().1.calc(), 9);
    }
}
