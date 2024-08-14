use nom::combinator::map;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::tuple;
use crate::ast::state::AstState;

pub mod parsing;
pub mod ast;
pub mod parser;
pub mod input;
pub mod tags;
mod err;
mod inputctx;

