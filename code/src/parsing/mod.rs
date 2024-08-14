use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, space0};
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::{pair, tuple};

pub mod parse_identifier;
pub mod parse_assign;
mod take;
pub mod parse_literal;
pub mod parse_express;
pub mod parse_op;
pub mod parse_stmt;
pub mod parse_tuple;
pub mod parse_function_call;
pub mod parse_for;
pub mod parse_variable;
pub mod parse_declaration_mode;
pub mod parse_type;
mod test;
pub mod parse_function;
pub mod parse_atom;
mod parse_block;
mod parse_enums;
mod parse_indent;
mod parse_body;
pub mod utils;

