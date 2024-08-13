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
use nom::combinator::{map, map_res, opt};
use nom::number::complete::float;
use nom::sequence::delimited;
use regex::Regex;
use crate::ast::node::Literal;
// fn parse_float(input: &str) -> IResult<&str, &str> {
//     let re = Regex::new(r"^[+-]?(\d+(\.\d*)?|\.\d+)([eE][+-]?\d+)?$").unwrap();
//     if re.is_match(input) {
//         let pos = input.find(|c: char| !c.is_digit(10) && c != '.' && c != '+' && c != '-' && c != 'e' && c != 'E');
//         if let Some(pos) = pos {
//             Ok((&input[pos..], &input[..pos]))
//         } else {
//             Ok(("", input))
//         }
//     } else {
//         Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Float)))
//     }
// }
// 判断字符是否是数字
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

// 解析整数
fn parse_int(input: &str) -> IResult<&str, Literal> {
    map_res(
        digit1,
        |s: &str| s.parse::<i32>().map(Literal::Int),
    )(input)
}

// 解析浮点数
fn parse_float(input: &str) -> IResult<&str, Literal> {
    map_res(
        take_while(|c: char| is_digit(c) || c == '.'),
        |s: &str| s.parse::<f64>().map(Literal::Float),
    )(input)
}
fn parse_string(input: &str) -> IResult<&str, Literal> {
    let (input, _) = char('"')(input)?;
    let (input, content) = take_while(|c| c != '"')(input)?;
    let (input, _) = char('"')(input)?;
    Ok((input, Literal::String(content.to_string())))
}
// 解析布尔值
fn parse_bool(input: &str) -> IResult<&str, Literal> {
    alt((
        map(tag("true"), |_| Literal::Bool(true)),
        map(tag("false"), |_| Literal::Bool(false)),
    ))(input)
}

pub fn parse_literal(input: &str) -> IResult<&str, Literal> {

    alt((
        parse_bool,
        parse_float,
        parse_int,
        parse_string,
       //map_res(parse_float, |s: &str| s.parse::<f64>().map(Literal::Float)),

        // map_res(digit1, |s: &str| s.parse::<i32>().map(Literal::Int)),
        //
        // map(delimited(char('"'), take_until("\""), char('"')), |s: &str| {
        //     Literal::String(s.to_string())
        // }),
        // parse_string
    ))(input)
}