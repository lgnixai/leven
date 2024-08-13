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
use crate::ast::node::{Expr, Literal};
use crate::parsing::parse_function::parse_function_call;
use crate::parsing::parse_ident::parse_identifier;
use crate::parsing::parse_literal::parse_literal;
use crate::parsing::parse_op::parse_binary_op;


pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        // 字面量表达式
        map(parse_literal, Expr::Literal),
        // 变量表达式
        map(parse_identifier, |id: &str| Expr::Variable(id.to_string())),
        // 二元运算
        parse_binary_op,
        // 函数调用
        parse_function_call,
        // 其他表达式
    ))(input)
}