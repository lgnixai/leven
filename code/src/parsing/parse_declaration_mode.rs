use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0},
    combinator::{opt, recognize},
    sequence::{pair, preceded, tuple},
    IResult,
};
use nom::bytes::complete::take_while;
use nom::character::complete::not_line_ending;
use nom::combinator::map;
use nom::sequence::terminated;
use crate::ast::node::{DeclarationMode, Stmt, Type};
use crate::parsing::parse_ident::parse_identifier;

pub fn parse_declaration_mode(input: &str) -> IResult<&str, DeclarationMode> {
    alt((
        map(tag("varip"), |_| DeclarationMode::Varip),

        map(tag("var"), |_| DeclarationMode::Var),
    ))(input)
}