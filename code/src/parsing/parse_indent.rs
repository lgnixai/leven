
use nom::character::complete::one_of;
use nom::combinator::map_res;

use nom::IResult;
use nom::multi::many0;
use crate::ast::error::PineErrorKind;
use crate::input::Input;
use nom::error::{context, ErrorKind, ParseError};
//
// fn indent_token(input: Input) -> IResult<Input, Token> {
//     context(
//         "indent_token",
//         map_res(
//             many0(one_of(" \t\n\r")),
//             |whitespaces| {
//                 let mut indent = 0;
//                 let mut has_newline = false;
//                 for c in whitespaces {
//                     match c {
//                         ' ' => indent += 1,
//                         '\t' => indent += 4,
//                         '\n' => {
//                             indent = 0;
//                             has_newline = true;
//                         }
//                         '\r' => indent = 0,
//                         _ => panic!("unexpected whitespace character: {}", c),
//                     }
//                 }
//                 if has_newline {
//                     Ok(Token::Indent(input.location_line(), indent))
//                 } else {
//                     Err(nom::Err::Error(nom::error::Error { input, code: ErrorKind::Space }))
//                 }
//             },
//         ))(input)
// }