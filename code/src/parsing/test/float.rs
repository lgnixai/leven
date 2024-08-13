// use nom::{
//     IResult,
//     branch::alt,
//     bytes::complete::{take_while, tag},
//     character::complete::{char, digit1},
//     combinator::{map, map_res, recognize},
//     sequence::{preceded, pair},
// };
// use regex::Regex;
// use std::str::FromStr;
// use nom::sequence::delimited;
// use crate::parsing::parse_literal::parse_literal;
//
// // Custom float parser using regex
// fn parse_float2(input: &str) -> IResult<&str, &str> {
//     let re = Regex::new(r"^[+-]?(\d+(\.\d*)?|\.\d+)([eE][+-]?\d+)?$").unwrap();
//     if re.is_match(input) {
//         // Find the end of the float number
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
// fn parse_float(input: &str) -> IResult<&str, &str> {
//     let re = Regex::new(r"^[+-]?(\d+(\.\d*)?|\.\d+)([eE][+-]?\d+)?$").unwrap();
//     if re.is_match(input) {
//         // Match the float part from the input
//         let pos = re.find(input).map_or(input.len(), |m| m.end());
//         Ok((&input[pos..], &input[..pos]))
//     } else {
//         Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Float)))
//     }
// }
//
// // Enum for different literal types
// #[derive(Debug)]
// pub enum Literal {
//     Int(i32),
//     Float(f64),
//     Bool(bool),
//     String(String),
// }
// // 解析字符串
// fn parse_string(input: &str) -> IResult<&str, Literal> {
//     let (input, _) = char('"')(input)?;
//     let (input, content) = take_while(|c| c != '"')(input)?;
//     let (input, _) = char('"')(input)?;
//     Ok((input, Literal::String(content.to_string())))
// }
// // Parser for literals
// // pub fn parse_literal2(input: &str) -> IResult<&str, Literal> {
// //     alt((
// //
// //         //map_res(parse_float, |s: &str| s.parse::<f64>().map(Literal::Float)),
// //
// //         map_res(digit1, |s: &str| s.parse::<i32>().map(Literal::Int)),
// //         map(tag("true"), |_| Literal::Bool(true)),
// //         map(tag("false"), |_| Literal::Bool(false)),
// //         //parse_string
// //         //map(parse_string,|s: &str| Literal::String(s.to_string())), // Parse strings
// //
// //          map(preceded(char('"'), preceded(take_while(|c| c != '"'), char('"'))), |s: &str| Literal::String(s.to_string())),
// //     ))(input)
// // }
// #[test]
// // Test function
// fn main() {
//     let tests = vec![
//         "123",
//         "3.14",
//         "true",
//         "false",
//         "\"hello world\"",
//     ];
//
//     for test in tests {
//         match parse_literal(test) {
//             Ok((remaining, literal)) => {
//                 println!("Parsed literal: {:?}, Remaining: '{}'", literal, remaining);
//             }
//             Err(err) => println!("Failed to parse '{}': {:?}", test, err),
//         }
//     }
// }
