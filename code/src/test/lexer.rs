// use nom::{
//     IResult,
//     bytes::complete::{tag, take_until, take_while1},
//     sequence::tuple,
//     character::complete::{alpha1, space0, multispace0, char},
//     combinator::{map, opt},
//     multi::{separated_list0, many0},
//     branch::alt,
// };
// use std::fs::{self, File};
// use std::io::{self, Write};
//
// // 1. 定义 AST 节点
// #[derive(Debug)]
// struct VarDeclaration {
//     name: String,
//     value: String,
// }
//
// #[derive(Debug)]
// struct FunctionParameter {
//     name: String,
//     default_value: Option<String>,
// }
//
// #[derive(Debug)]
// struct FunctionDeclaration {
//     name: String,
//     parameters: Vec<FunctionParameter>,
//     body: Vec<Statement>,
//     is_multi_line: bool,
// }
//
// #[derive(Debug)]
// enum Statement {
//     VarDeclaration(VarDeclaration),
//     Expression(String),
//     Return(String),
//     Comment(String),
//     Original(String),
// }
//
// #[derive(Debug)]
// struct AST {
//     elements: Vec<Statement>,
// }
//
// // 2. 实现解析器结构
//
// fn parse_identifier(input: &str) -> IResult<&str, String> {
//     map(alpha1, |s: &str| s.to_string())(input)
// }
//
// fn parse_var_declaration(input: &str) -> IResult<&str, Statement> {
//     let (input, name) = parse_identifier(input)?;
//     let (input, _) = space0(input)?;
//     let (input, _) = tag("=")(input)?;
//     let (input, _) = space0(input)?;
//     let (input, expr) = take_while1(|c: char| !c.is_whitespace())(input)?;
//     Ok((input, Statement::VarDeclaration(VarDeclaration {
//         name: name.to_string(),
//         value: expr.to_string(),
//     })))
// }
//
// fn parse_comment(input: &str) -> IResult<&str, Statement> {
//     map(
//         tuple((tag("#"), take_until("\n"))),
//         |(_, comment): (&str, &str)| Statement::Comment(comment.trim().to_string()),
//     )(input)
// }
//
// fn parse_parameter(input: &str) -> IResult<&str, FunctionParameter> {
//     map(
//         tuple((
//             parse_identifier,
//             opt(tuple((space0, tag("="), space0, take_while1(|c: char| c.is_alphanumeric())))),
//         )),
//         |(name, default_value)| FunctionParameter {
//             name: name.to_string(),
//             default_value: default_value.map(|(_, _, _, val)| val.to_string()),
//         },
//     )(input)
// }
//
// fn parse_function_body(input: &str) -> IResult<&str, Vec<Statement>> {
//     let mut statements = Vec::new();
//     let mut remaining_input = input;
//
//     while let Ok((new_input, statement)) = parse_statement(remaining_input) {
//         statements.push(statement);
//         remaining_input = new_input;
//
//         // 检查下一行是否未缩进
//         if let Ok((next_input, _)) = space0(remaining_input) {
//             if !next_input.starts_with("    ") && !next_input.starts_with("\t") {
//                 break;
//             }
//         }
//     }
//
//     Ok((remaining_input, statements))
// }
//
// fn parse_function_declaration(input: &str) -> IResult<&str, Statement> {
//     map(
//         tuple((
//             parse_identifier,
//             char('('),
//             separated_list0(tag(","), parse_parameter),
//             char(')'),
//             space0,
//             tag("=>"),
//             multispace0,
//             alt((
//                 map(parse_statement, |stmt| (vec![stmt], false)),
//                 map(parse_function_body, |stmts| (stmts, true)),
//             )),
//         )),
//         |(name, _, params, _, _, _, _, (body, is_multi_line))| {
//             let mut body = body;
//             if !is_multi_line {
//                 if let Some(last_stmt) = body.pop() {
//                     body.push(Statement::Return(match last_stmt {
//                         Statement::Expression(expr) => expr,
//                         _ => format!("{}", last_stmt),
//                     }));
//                 }
//             }
//             Statement::Original(format!(
//                 "function {}({}) {{\n{}\n}}",
//                 name,
//                 params.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", "),
//                 body.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("\n")
//             ))
//         },
//     )(input)
// }
//
// fn parse_statement(input: &str) -> IResult<&str, Statement> {
//     alt((
//         parse_comment,
//         parse_var_declaration,
//         parse_function_declaration,
//         map(take_until("\n"), |original: &str| Statement::Original(original.trim().to_string())),
//     ))(input)
// }
//
// fn parse_ast(input: &str) -> IResult<&str, AST> {
//     map(
//         many0(tuple((multispace0, parse_statement))),
//         |statements| AST {
//             elements: statements.into_iter().map(|(_, s)| s).collect(),
//         },
//     )(input)
// }
//
// // 3. 实现转换器
//
// impl std::fmt::Display for Statement {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Statement::VarDeclaration(var) => write!(f, "let {} = {};", var.name, var.value),
//             Statement::Expression(expr) => write!(f, "{}", expr),
//             Statement::Return(expr) => write!(f, "  return {};", expr),
//             Statement::Comment(comment) => write!(f, "//{}", comment),
//             Statement::Original(original) => write!(f, "{}", original),
//         }
//     }
// }
//
// // Convert AST to TypeScript
// fn convert_to_ts(ast: &AST) -> String {
//     let mut result = String::new();
//     for elem in &ast.elements {
//         result.push_str(&elem.to_string());
//         result.push_str("\n");
//     }
//     result
// }
//
// // 4. 文件读写
//
// fn read_file(path: &str) -> io::Result<String> {
//     fs::read_to_string(path)
// }
//
// fn write_file(path: &str, content: &str) -> io::Result<()> {
//     let mut file = File::create(path)?;
//     file.write_all(content.as_bytes())
// }
//
// // Main function
// #[test]
// fn main() -> io::Result<()> {
//     // 从文件中读取 Pine Script
//     let input = read_file("input.ps")?;
//
//     // 将输入解析为 AST
//     let (_, ast) = parse_ast(&input).unwrap();
//
//     // 将 AST 转换为 TypeScript
//     let output = convert_to_ts(&ast);
//
//     // 将结果写入文件
//     write_file("output.ts", &output)?;
//
//     println!("Conversion complete. Check output.ts for the result.");
//     Ok(())
// }
