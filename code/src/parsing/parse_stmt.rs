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
use crate::ast::node::{Literal, Stmt};
use crate::parsing::parse_assign::parse_assignment;
use crate::parsing::parse_express::parse_expr;
use crate::parsing::parse_for::parse_for_stmt;
use crate::parsing::parse_variable::parse_variable_declaration;


pub fn parse_stmt(input: &str) -> IResult<&str, Stmt> {
    alt((
        parse_assignment,
        // parse_if_stmt,
        // parse_while_stmt,
        parse_for_stmt,
        map(parse_expr, Stmt::ExprStmt),
        // 其他语句解析
    ))(input)
}