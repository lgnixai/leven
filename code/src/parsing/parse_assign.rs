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
use crate::ast::node::Stmt::VariableDeclaration;
use crate::parsing::parse_express::parse_expr;
use crate::parsing::parse_ident::parse_identifier;
use crate::parsing::parse_variable::parse_variable_declaration;

pub fn parse_assignment(input: &str) -> IResult<&str, Stmt> {
    let (input, (declaration_mode, var_type, identifier)) = parse_variable_declaration(input)?;
    let (input, _) = preceded(multispace0, char('='))(input)?;  // 解析等号

    //let (input, expr) = preceded(multispace0, take_while1(|c| c != '\n'))(input)?;  // 解析表达式
    let (input, expr) = preceded(multispace0, not_line_ending)(input)?;  // 解析表达式

    // Ok((input, (mode, var_type, identifier, expr)))

    let variable_declaration = VariableDeclaration {
        declaration_mode,
        var_type,
        identifier: identifier.to_string(),
        value: parse_expr(expr).unwrap().1, // 使用 parse_expr 解析表达式
    };

    Ok((input, variable_declaration))
}

