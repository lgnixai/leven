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
use nom::multi::{separated_list0, separated_list1};
use nom::number::complete::float;
use nom::sequence::{delimited, tuple};
use crate::ast::node::{BinOp, Expr, Literal, Stmt};
use crate::input::{Input, PineResult};
use crate::parsing::parse_express::parse_expr;
use crate::parsing::parse_identifier::parse_identifier;

pub fn parse_function_call(input: Input) -> PineResult< Expr> {
    let (input, (name, args)) = delimited(
        char('('),
        tuple((
            parse_identifier,
            preceded(
                char('('),
                separated_list0(
                    char(','),
                    preceded(multispace0, parse_expr)
                ),
            ),
        )),
        char(')'),
    )(input)?;

    Ok((
        input,
        Expr::FunctionCall {
            name: name.to_string(),
            args,
        },
    ))
}