use crate::core::ast::{term, Calculatable, Term};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, map_res};
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;

#[derive(Eq, PartialEq, Debug)]
pub enum Expr {
    Terms {
        head: Term,
        tail: Vec<TermWithOperator>,
    },
}

impl Calculatable for Expr {
    fn calc(&self) -> i64 {
        match self {
            Expr::Terms { head, tail } => {
                head.calc() + tail.into_iter().fold(0, |a, t| a + t.calc())
            }
        }
    }
}

pub fn expr(input: &str) -> IResult<&str, Expr> {
    map(
        pair(term::term, many0(term_with_operator)),
        |(head, tail)| Expr::Terms { head, tail },
    )(input)
}

#[derive(Eq, PartialEq, Debug)]
pub struct TermWithOperator {
    operator: Operator,
    term: Term,
}

impl Calculatable for TermWithOperator {
    fn calc(&self) -> i64 {
        let ret = self.term.calc();
        match self.operator {
            Operator::Plus => ret,
            Operator::Minus => ret * -1,
        }
    }
}

fn term_with_operator(input: &str) -> IResult<&str, TermWithOperator> {
    map(pair(operator, term::term), |(operator, term)| {
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
            term_with_operator("+ 123"),
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
        assert!(term_with_operator("123").is_err());
        assert!(term_with_operator("+ ").is_err());
    }

    #[test]
    fn expr_ok() {
        assert_eq!(
            expr("1"),
            Ok((
                "",
                Expr::Terms {
                    head: term::term("1").unwrap().1,
                    tail: vec![]
                }
            ))
        );
        assert_eq!(
            expr("1 + 2 - 3"),
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
        assert!(expr("+").is_err());
    }

    #[test]
    fn integration() {
        assert_eq!(expr("(1 + 2) * 3").unwrap().1.calc(), 9);
    }
}
