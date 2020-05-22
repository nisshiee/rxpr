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

use crate::core::Num;

pub trait Calculatable<N: Num> {
    fn calc(&self) -> N;
}

pub fn parse<N: Num>(input: &str) -> IResult<&str, Expr<N>> {
    all_consuming(expr::expr)(input)
}
