use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alpha1, alphanumeric1, char},
    combinator::{recognize},
    sequence::{pair, preceded},
    IResult,
};
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::number::complete::float;
use nom::sequence::delimited;
use crate::ast::node::{BinOp, Expr, Literal};
use crate::parsing::parse_express::parse_expr;

pub fn parse_binary_op(input: &str) -> IResult<&str, Expr> {
    let (input, left) = parse_expr(input)?;
    let (input, op) = parse_bin_op(input)?;
    let (input, right) = parse_expr(input)?;
    Ok((input, Expr::BinaryOp(Box::new(left), op, Box::new(right))))
}

pub fn parse_bin_op(input: &str) -> IResult<&str, BinOp> {
    alt((
        map(tag("+"), |_| BinOp::Add),
        map(tag("-"), |_| BinOp::Sub),
        map(tag("*"), |_| BinOp::Mul),
        map(tag("/"), |_| BinOp::Div),
        // 其他运算符解析
    ))(input)
}