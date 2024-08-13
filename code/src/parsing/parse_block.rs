use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, space0};
use nom::combinator::opt;
use nom::IResult;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, preceded};
use crate::ast::node::{Block, Parameter};
use crate::input::{Input, PineResult};
use crate::parsing::parse_express::parse_expr;
use crate::parsing::parse_identifier::parse_identifier;
use crate::parsing::parse_stmt::parse_stmt;

pub fn parse_parameter(input:Input) -> PineResult< Parameter> {
    let (input, ident) = parse_identifier(input)?;
    let (input, default_value) = opt(preceded(tag("="), parse_expr))(input)?;
    Ok((input, Parameter { name:ident.name, default_value }))
}

pub fn parse_parameter_list(input:Input) -> PineResult< Vec<Parameter>> {
    delimited(
        tag("("),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            parse_parameter,
        ),
        tag(")"),
    )(input)
}

pub fn parse_block(input:Input) -> PineResult<Block> {
    let (input, statements) = many0(preceded(multispace0, parse_stmt))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, return_expr) = parse_expr(input)?;
    Ok((input, Block { statements, return_expr }))
}

fn parse_single_line_body(input:Input) -> PineResult<Block> {
    let (input, return_expr) = parse_expr(input)?;
    Ok((input, Block { statements: Vec::new(), return_expr }))
}

pub fn parse_function_body(input:Input) -> PineResult<Block> {
    alt((
        preceded(
            delimited(space0, tag("=>"), space0),
            parse_single_line_body,
        ),
        delimited(
            delimited(space0, tag("=>"), space0),
            parse_block,
            multispace0,
        )
    ))(input)
}
