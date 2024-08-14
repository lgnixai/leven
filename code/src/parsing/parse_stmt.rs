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
use crate::ast::node::Stmt;

use crate::input::{Input, PineResult};
use crate::parsing::parse_assign::parse_assignment;
use crate::parsing::parse_enums::parse_enum_declaration;
use crate::parsing::parse_express::parse_expr;
use crate::parsing::parse_variable::parse_variable_declaration;


pub fn parse_stmt(input: Input) -> PineResult<Stmt> {
    alt((
        map(parse_assignment, |declaration| {
            Stmt::Variable(declaration)
        }),
        map(parse_enum_declaration, |declaration| {
            Stmt::Enum((declaration))
        }),
        map(parse_expr, Stmt::ExprStmt),

        //map(take_until("\n"), |s: &str| Stmt::Original(String::from(s))),

    ))(input)
    // alt((
    //
    //     map(parse_assignment, |declaration| {
    //         Stmt::Variable(Box::new(declaration))
    //     }),
    //
    //     // map(parse_enum_declaration, |declaration| {
    //     //     Stmt::Enum(Box::new(declaration))
    //     // }),
    //    //  //parse_assignment,
    //    //  parse_enum_declaration,
    //    // // parse_expr,
    //     // parse_if_stmt,
    //     // parse_while_stmt,
    //     //parse_for_stmt,
    //     //map(parse_expr, Stmt::ExprStmt),
    //     // 其他语句解析
    // ))(input)
}