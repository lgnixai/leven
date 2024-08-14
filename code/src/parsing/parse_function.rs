use std::path::PathBuf;
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alpha1, alphanumeric1, char},
    combinator::{recognize},
    sequence::{pair, preceded},
    IResult,
};
use nom::bytes::complete::{is_not, tag, take_until, take_while};
use nom::character::complete::{digit1, line_ending, multispace0, space0};
use nom::combinator::{map, map_res, opt};
use nom::multi::{many0, many1, separated_list0, separated_list1};
use nom::number::complete::float;
use nom::sequence::{delimited, terminated, tuple};
use crate::ast::node::{BinOp, Block, Body, Expr, FunctionDeclaration, Literal, Parameter, Stmt};
use crate::input::{Input, PineResult};
use crate::inputctx::ParserCtx;
use crate::parsing::parse_block::{parse_function_body, parse_parameter_list};
use crate::parsing::parse_body::{indent, parse_block_indent, parse_body};
use crate::parsing::parse_declaration_mode::parse_declaration_mode;
use crate::parsing::parse_express::parse_expr;


use crate::parsing::parse_identifier::parse_identifier;
use crate::parsing::parse_literal::parse_literal;
use crate::parsing::parse_stmt::parse_stmt;

use crate::parsing::parse_type::parse_type;

// pub fn parse_function1(input:Input) -> PineResult< FunctionDeclaration> {
//
//     println!("{:?}",input.fragment());
//     let (input, ident) = parse_identifier(input)?;
//     let (input, params) = parse_parameter_list(input)?;
//     let (input, body) = parse_function_body(input)?;
//     Ok((input, FunctionDeclaration { name: ident.name, params, body }))
// }

// fn parse_arrow_and_content(input: &str) -> PineResult<&str, (&str, &str)> {
//     let (input, arrow) = tag("=>")(input)?;
//     let (input, content) = take_until("\n")(input)?;
//     let (input, _) = alt((line_ending, tag("\n")))(input)?;
//
//     Ok((input, (arrow, content.trim())))
// }


fn parse_body_decls(input: Input) -> PineResult<Body> {
    println!("多行");
    map(
        indent(separated_list0(
            line_ending,
            preceded(
                parse_block_indent,
                parse_stmt,
            ),
        )),
        Body::new,
    )(input)
}

pub fn parse_function(input: Input) -> PineResult<FunctionDeclaration> {
    map(
        tuple((
            parse_identifier,
            parse_parameter_list,
            space0,
            alt((
                // 处理单行函数体
                //map(take_until("\n"), |body: &str| vec![body.trim().to_string()]),
                // map(preceded( take_until("\n"),parse_stmt), |stmt| Body::new(vec![stmt])),
                map(tuple((
                    space0,
                    tag("=>"),
                    space0,
                    parse_stmt,
                    //is_not("\n"), // 匹配 `=>` 后的非换行符内容
                    alt((tag("\n"), line_ending)), // 匹配行结束符
                )),
                    |(_,_,_, stmt, _)| {
                        Body::new(vec![stmt])
                    }),
                //
                //     preceded( take_until("\n"),parse_stmt), |stmt| (Body::new(vec![Stmt::ExprStmt(stmt)]), false)),

                // 处理多行函数体
                preceded(terminated(tag("=>"), line_ending), parse_body_decls),
            ))
            // terminated(tag("=>"), line_ending),
            // parse_body_decls,

            //map(preceded( take_until("\n"),parse_stmt), |stmt| (Body::new(vec![stmt]), false)),

            // alt((
            //     // map(preceded( take_until("\n"),parse_stmt), |stmt| (Body::new(vec![Stmt::ExprStmt(stmt)]), false)),
            //     map(preceded( take_until("\n"),parse_stmt), |stmt| (Body::new(vec![stmt]), false)),
            //     map(parse_body, |stmts| (stmts, true)),
            // )),
        )),
        |(ident, params, _, body)| {
            let mut body = body;
            //单行处理
            // if !is_multi_line {
            //     print!("单行函数")
            //     // if let Some(last_stmt) = body.pop() {
            //     //     body.push( Stmt::Return(match last_stmt {
            //     //         Statement::Expression(expr) => expr,
            //     //         _ => format!("{}", last_stmt),
            //     //     }));
            //     // }
            // } else{
            //     print!("多行函数")
            // }
            FunctionDeclaration { name: ident.name, params, body }
        },
    )(input)
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
#[test]
fn main() {
    let input2 = r#"geom_average(x, y) =>x+2
    "#;

    let input21 = r#"geom_average(x, y) =>
    a=x+y
    c=y-2
    b=a+c
genv()"#;


    println!("{:?}", input2);
    let mut path = PathBuf::new();
    let ctx = ParserCtx::new(path);

    let result = parse_function(Input::new_extra(input2, ctx));

    // let result = parse_function(input);
    //println!("{:?}",input);
    match result {
        Ok((remaining, func_decl)) => {
            println!("Parsed function: {:?}, Remaining input: {}", func_decl, remaining);
        }
        Err(e) => println!("Error parsing function: {:?}", e),
    }
}