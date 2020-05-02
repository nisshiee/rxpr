pub mod expr;
pub use expr::Expr;

pub mod term;
pub use term::Term;

pub mod factor;
pub use factor::Factor;

pub mod constant;
pub use constant::Constant;

use nom::combinator::all_consuming;
use nom::IResult;

pub trait Calculatable {
    fn calc(&self) -> i64;
}

pub fn parse(input: &str) -> IResult<&str, Expr> {
    all_consuming(expr::expr)(input)
}
