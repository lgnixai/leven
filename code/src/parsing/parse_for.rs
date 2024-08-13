
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
use nom::combinator::{map, map_res, opt};
use nom::multi::many0;
use nom::number::complete::float;
use nom::sequence::{delimited, terminated, tuple};
use crate::ast::node::{BinOp, Expr, Literal, Stmt};
use crate::parsing::parse_declaration_mode::parse_declaration_mode;

use crate::parsing::parse_express::parse_expr;
use crate::parsing::parse_ident::parse_identifier;
use crate::parsing::parse_stmt::parse_stmt;
use crate::parsing::parse_type::parse_type;

pub fn parse_for_stmt(input: &str) -> IResult<&str, Stmt> {

    map(
        tuple((
                  opt(preceded(
                      multispace0,
                      parse_declaration_mode
                      //alt((parse_declaration_mode, map(tag(""), |_| None))),
                  )),
                  opt(preceded(multispace0, parse_type)),
                  preceded(multispace0, parse_identifier),
                  preceded(tuple((multispace0, char('='), multispace0)), parse_expr),
                  preceded(multispace0, tag("to")),
                  preceded(multispace0, parse_expr),
                  opt(preceded(
                      tuple((multispace0, tag("by"), multispace0)),
                      parse_expr,
                  )),
                  delimited(
                      multispace0,
                      many0(terminated(parse_stmt, multispace0)),
                      multispace0,
                  ),

        )),|(declaration_mode, var_type, identifier, start_expr,_, end_expr, step_expr, body)|Stmt::ForLoop {
            declaration_mode,
            var_type,
            identifier: identifier.to_string(),
            start_expr,
            end_expr,
            step_expr,
            body,
            return_value: None, // 这里需要进一步处理返回值
        },
    )(input)


}