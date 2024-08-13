use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alpha1, alphanumeric1, char},
    combinator::{recognize},
    sequence::{pair, preceded},
    IResult,
};
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{digit1, line_ending, multispace0};
use nom::combinator::{map, map_res, opt};
use nom::multi::{many0, separated_list0};
use nom::number::complete::float;
use nom::sequence::{delimited, terminated, tuple};
use crate::ast::node::{BinOp, Block, Expr, FunctionDeclaration, Literal, Parameter, Stmt};
use crate::input::{Input, PineResult};
use crate::parsing::parse_block::{parse_function_body, parse_parameter_list};
use crate::parsing::parse_declaration_mode::parse_declaration_mode;
use crate::parsing::parse_express::parse_expr;


use crate::parsing::parse_identifier::parse_identifier;
use crate::parsing::parse_literal::parse_literal;
use crate::parsing::parse_stmt::parse_stmt;

use crate::parsing::parse_type::parse_type;

fn parse_function(input:Input) -> PineResult< FunctionDeclaration> {
    let (input, ident) = parse_identifier(input)?;
    let (input, params) = parse_parameter_list(input)?;
    let (input, body) = parse_function_body(input)?;
    Ok((input, FunctionDeclaration { name: ident.name, params, body }))
}

//
// fn parse_function(input: &str) -> IResult<&str, FunctionDeclaration> {
//     let (input, (name, params)) = tuple((
//         preceded(multispace0, parse_identifier),
//         delimited(
//             char('('),
//             separated_list0(
//                 preceded(multispace0, char(',')),
//                 parse_parameter
//             ),
//             char(')')
//         ),
//     ))(input)?;
//
//     // 先尝试解析单行函数
//     if let Ok((input, return_value)) = preceded(
//         tuple((multispace0, tag("=>"), multispace0)),
//         parse_expr
//     )(input)
//     {
//         return Ok((input, FunctionDeclaration {
//             name:String::from(name),
//             params,
//             body:Block{
//                 statements:vec![],
//                 return_expr:return_value
//             }
//
//
//         }));
//     }
//
//     // 解析多行函数
//     let (input, _) = tuple((multispace0, tag("=>"), line_ending))(input)?;
//
//     // 解析函数体
//     let mut body = vec![];
//     let mut input = input;
//     let mut ret="";
//     loop {
//         if let Ok((new_input, line)) = preceded(
//             indent_parser,
//             alt((
//                 parse_stmt,
//                 map(parse_expr, Stmt::ExprStmt),
//             )),
//         )(input)
//         {
//             body.push(line);
//             input = new_input;
//         } else {
//             break;
//         }
//     }
//
//     // 返回解析的函数
//     Ok((input, FunctionDeclaration {
//         name:String::from(name),
//         params,
//         body:Block{
//             statements:body,
//             return_expr:Stmt::Return(Stmt::ExprStmt(body.last()))
//         }
//
//     }))
// }
//
// // 辅助函数：解析缩进
// fn indent_parser(input: &str) -> IResult<&str, &str> {
//     alt((
//         tag("    "), // 4 个空格
//         tag("\t"),   // 1 个制表符
//     ))(input)
// }
//
// // 解析函数参数
// // 解析参数（包括可选的默认值）
// fn parse_parameter(input: &str) -> IResult<&str, Parameter> {
//     // 参数可以有一个名称和一个可选的默认值
//     let (input, (name, default_value)) = tuple((
//         parse_identifier, // 解析参数名称
//         opt(preceded(
//             tuple((multispace0, char('='), multispace0)), // 解析等号和可能的空格
//             parse_literal, // 解析字面量作为默认值
//         )),
//     ))(input)?;
//
//     Ok((
//         input,
//         Parameter {
//             name:String::from(name),
//             default_value:default_value,
//
//
//         },
//     ))
//
//
// }
//
//#[test]
// fn main() {
//     let input = r#"geom_average(x, y) =>
//     a = x*x
//     b = y*y
//     math.sqrt(a + b)
//
//
//     "#;
//     let result = parse_function(input);
//     println!("{:?}",input);
//     match result {
//         Ok((remaining, func_decl)) => {
//             println!("Parsed function: {:?}, Remaining input: {}", func_decl, remaining);
//         }
//         Err(e) => println!("Error parsing function: {:?}", e),
//     }
// }