use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, not_line_ending};
use nom::combinator::{map, opt};
use nom::IResult;
use nom::sequence::{preceded, tuple};
use crate::ast::node::{DeclarationMode, Stmt, Type};
use crate::ast::node::Stmt::VariableDeclaration;
use crate::parsing::parse_declaration_mode::parse_declaration_mode;
use crate::parsing::parse_express::parse_expr;
use crate::parsing::parse_ident::parse_identifier;
use crate::parsing::parse_type::parse_type;

pub fn parse_variable_declaration(input: &str) -> IResult<&str, (Option<DeclarationMode>, Option<Type>, &str)> {
    tuple((
        // multispace0,
        //opt(preceded(multispace0, parse_declaration_mode)
        opt(preceded(
            multispace0,
            parse_declaration_mode
            //alt((parse_declaration_mode, map(tag(""), |_| None))),
        )),
        opt(preceded(multispace0, parse_type)),
        preceded(multispace0, parse_identifier),
    ))(input)

    // let (input, _) = multispace0(input)?;
    // let (input, declaration_mode) = opt(preceded(multispace0, parse_declaration_mode))(input)?;
    //
    // println!("declaration_mode=={:?}", declaration_mode);
    // let (input, _) = multispace0(input)?;
    // let (input, var_type) = opt(preceded(multispace0, parse_type))(input)?;
    // let (input, _) = multispace0(input)?;
    // let (input, identifier) = preceded(multispace0, parse_identifier)(input)?;
    //
    // Ok((input, (declaration_mode, var_type, identifier)))
}


// 解析赋值表达式
