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
use nom::multi::separated_list1;
use nom::number::complete::float;
use nom::sequence::delimited;
use crate::ast::node::{BinOp, Expr, Literal, Stmt};
use crate::parsing::parse_express::parse_expr;
use crate::parsing::parse_function::parse_function_call;
use crate::parsing::parse_ident::parse_identifier;


fn parse_tuple_declaration(input: &str) -> IResult<&str, Stmt> {
    let (input, identifiers) = delimited(
        char('('),
        separated_list1(char(','), parse_identifier),
        char(')'),
    )(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value) = alt((
        parse_function_call,
        // 处理结构体解析
        // parse_structure,
    ))(input)?;

    Ok((
        input,
        Stmt::TupleDeclaration {
            identifiers: identifiers.into_iter().map(String::from).collect(),
            value,
        },
    ))
}
