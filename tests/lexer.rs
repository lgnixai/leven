

use std::fs::{self, File};
use std::io::{self, Write};
use nom::{
    IResult,
    bytes::complete::{tag, take_until, take_while1},
    sequence::{tuple, preceded},
    character::complete::{alpha1, space0, multispace0, char},
    combinator::{map, opt},
    multi::{separated_list0, many0},
    branch::alt,
};
use nom::character::complete::multispace1;

use nom::error::{Error, ErrorKind};
use nom::sequence::delimited;


#[derive(Debug)]
struct VarDeclaration {
    name: String,
    value: String,
}

#[derive(Debug)]
struct FunctionParameter {
    name: String,
    default_value: Option<String>,
}

#[derive(Debug)]
enum Statement {
    VarDeclaration(VarDeclaration),
    Expression(String),
    Return(String),
    Comment(String),
    Original(String),
}

#[derive(Debug)]
struct FunctionDeclaration {
    name: String,
    parameters: Vec<FunctionParameter>,
    body: Vec<Statement>,
}

#[derive(Debug)]
struct AST {
    elements: Vec<Statement>,
}

fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string())(input)
}

fn parse_var_declaration(input: &str) -> IResult<&str, Statement> {
    let (input, name) = parse_identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = space0(input)?;
    let (input, expr) = take_while1(|c: char| !c.is_whitespace())(input)?;
    Ok((input, Statement::VarDeclaration(VarDeclaration {
        name: name.to_string(),
        value: expr.to_string(),
    })))
}

fn parse_comment(input: &str) -> IResult<&str, Statement> {
    map(
        tuple((tag("#"), take_until("\n"))),
        |(_, comment): (&str, &str)| Statement::Comment(comment.trim().to_string()),
    )(input)
}

fn parse_parameter(input: &str) -> IResult<&str, FunctionParameter> {
    map(
        tuple((
            parse_identifier,
            opt(tuple((space0, tag("="), space0, take_while1(|c: char| c.is_alphanumeric())))),
        )),
        |(name, default_value)| FunctionParameter {
            name: name.to_string(),
            default_value: default_value.map(|(_, _, _, val)| val.to_string()),
        },
    )(input)
}
// Function to parse and validate indentation
fn parse_indentation(input: &str) -> IResult<&str, usize> {
    let (input, indent) = take_while1(|c| c == ' ' || c == '\t')(input)?;
    let indent_length = indent.chars().count();
    if indent_length % 4 == 0 || indent_length == 1 {
        Ok((input, indent_length))
    } else {


            // 这里我们可以生成一个自定义错误
            let custom_error = nom::Err::Error(Error::new(input, ErrorKind::Tag));
            println!("Error: {:?}", custom_error);
        Err(custom_error)

    }
}

fn parse_function_body(input: &str) -> IResult<&str, Vec<Statement>> {
    let mut body = Vec::new();
    let mut remaining_input = input;

    let mut prev_indent = 0;

    while !remaining_input.is_empty() {
        // Skip empty lines
        if remaining_input.starts_with('\n') {
            remaining_input = &remaining_input[1..];
            continue;
        }

        // Parse indentation
        let (input, indent_length) = match parse_indentation(remaining_input) {
            Ok((i, len)) => (i, len),
            Err(_) => break,
        };

        // Check if the indentation is the same level as before or new
        if indent_length < prev_indent {
            break;
        }

        prev_indent = indent_length;

        let (input, stmt) = alt((
            parse_var_declaration,
            map(take_until("\n"), |s: &str| Statement::Expression(s.trim().to_string())),
        ))(remaining_input)?;

        body.push(stmt);
        remaining_input = input;
    }

    Ok((remaining_input, body))
}
fn parse_function_declaration(input: &str) -> IResult<&str, Statement> {
    map(
        tuple((
            parse_identifier,
            char('('),
            separated_list0(tag(","), parse_parameter),
            char(')'),
            space0,
            tag("=>"),

            alt((
                 parse_function_body,
                map(parse_statement, |stmt| vec![stmt]),
            )),
        )),
        |(name, _, params, _, _, _, body)| {
            let mut body = body;

            println!("body===={:?}",body);
            if let Some(last_stmt) = body.pop() {
                body.push(Statement::Return(match last_stmt {
                    Statement::Expression(expr) => expr,
                    _ => format!("{:?}", last_stmt),
                }));
            }
            Statement::Original(format!(
                "function {}({}) {{\n{}\n}}",
                name,
                params.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", "),
                body.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("\n")
            ))
        },
    )(input)
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {

    println!("3333{:?}",input);
    alt((
        parse_comment,
        parse_var_declaration,
        parse_function_declaration,
        map(take_until("\n"), |original: &str| Statement::Original(original.trim().to_string())),
    ))(input)
}

fn parse_ast(input: &str) -> IResult<&str, AST> {
    map(
        many0(preceded(multispace0, parse_statement)),
        |statements| AST {
            elements: statements.into_iter().collect(),
        },
    )(input)
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::VarDeclaration(var) => write!(f, "    let {} = {};", var.name, var.value),
            Statement::Expression(expr) => write!(f, "    {}", expr),
            Statement::Return(expr) => write!(f, "    return {};", expr),
            Statement::Comment(comment) => write!(f, "//{}", comment),
            Statement::Original(original) => write!(f, "{}", original),
        }
    }
}
fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn write_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())
}
fn convert_to_ts(ast: &AST) -> String {
    let mut result = String::new();
    for elem in &ast.elements {
        result.push_str(&elem.to_string());
        result.push_str("\n");
    }
    result
}
// #[test]
// fn main() {
//     let input = r"#多行函数
// geom(x,y) =>
//     a = x+y
//     a+3
// #调用多行函数
// geom(2,3)";
//     let (_, ast) = parse_ast(input).unwrap();
//     let output = convert_to_ts(&ast);
//     println!("{}", output);
// }


// Main function
#[test]
fn main1() -> io::Result<()> {
    // Read Pine Script from a file
        let input = read_file("input.ps")?;

    // Parse the input to AST
    let (_, ast) = parse_ast(&input).unwrap();


    println!("{:?}",ast);
    // Convert AST to TypeScript
    let output = convert_to_ts(&ast);

    // Write the output to a file
    write_file("output.ts", &output)?;

    println!("Conversion complete. Check output.ts for the result.");
    Ok(())
}