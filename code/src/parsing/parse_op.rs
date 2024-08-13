use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alpha1, alphanumeric1, char},
    combinator::{recognize},
    sequence::{pair, preceded},
    IResult,
};
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, map_res};
use nom::number::complete::float;
use nom::sequence::{delimited, tuple};
use crate::ast::node::{BinaryOperation, BinOp, Expr, Literal};
use crate::input::{Input, PineResult};
use crate::parsing::parse_atom::parse_atom;

use crate::parsing::parse_express::parse_expr;


pub fn parse_binary_operator(input: Input) -> PineResult< BinaryOperation> {
    alt((
        map(char('+'), |_| BinaryOperation::Plus),
        map(char('-'), |_| BinaryOperation::Minus),
        map(char('*'), |_| BinaryOperation::Times),
        map(char('/'), |_| BinaryOperation::Divide),
        map(tag("=="), |_| BinaryOperation::Equal),
        map(tag("!="), |_| BinaryOperation::NotEqual),
    ))(input)
}

pub fn parse_binary_operation(input: Input) -> PineResult< Expr> {
    let (rest, (left, _, op, _, right)) = tuple((
        parse_atom,
        multispace0,
        parse_binary_operator,
        multispace0,
        parse_atom,
    ))(input)?;

    let expression = Expr::BinaryOp( Box::new(left),op, Box::new(right));
    Ok((rest, expression))
}

