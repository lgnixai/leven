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
use crate::ast::node::AstNode::Stmt;
use crate::input::{Input, PineResult};
use crate::parsing::parse_function_call::parse_function_call;
use crate::parsing::parse_identifier::parse_identifier;
use crate::parsing::parse_literal::parse_literal;
use crate::parsing::parse_op::{parse_binary_operation};

// 解析单项表达式（term）的函数
fn parse_term(input: Input) -> PineResult< Expr> {
    let (input, var) = parse_identifier(input)?;
    Ok((input, Expr::Variable(var.name)))
}

pub fn parse_expr(input: Input) -> PineResult< Expr> {


   // println!("==={:?}",input);
    alt((
        //parse_term,
        // 字面量表达式
        // 变量表达式

        // 二元运算

        // 函数调用
        parse_function_call,

        parse_binary_operation,
        map(parse_identifier, |id| Expr::Variable(id.to_string())),
        map(parse_literal, Expr::Literal),
        //map(take_until("\n"), |original: &str| Expr::Na),

        // 其他表达式
    ))(input)
}