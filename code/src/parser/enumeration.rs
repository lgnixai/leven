//
// use nom::{
//     combinator::{map, opt, value},
//     sequence::{preceded, tuple},
// };
// use nom::branch::alt;
// use nom::bytes::complete::tag;
// use nom::multi::{separated_list0, separated_list1};
// use nom::sequence::{delimited, terminated};
// use crate::input::{Input, PineResult, Positioned};
// use crate::Parser;
// use crate::parsing::parse_ident::parse_identifier;
// use crate::tags::spaned;
//
// impl Parser {
//
//     pub fn parse_enum_declaration<'a>(&'a self, input: Input<'a>) -> PineResult<Positioned<EnumDeclaration>> {
//         spaned(map(
//             tuple((
//                       spaned( tag("enum")),
//                       parse_identifier,
//             )),
//             |(_, name)| EnumDeclaration { name },
//         ))(input)
//     }
//
// }