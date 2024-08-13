// use nom::branch::alt;
// use nom::bytes::complete::tag;
// use nom::combinator::{map, opt};
// use nom::sequence::terminated;
// use crate::ast::node::Stmt;
// use crate::input::{Input, PineResult, Positioned};
//
// use crate::Parser;
// use crate::tags::spaned;
//
// impl Parser {
//     pub fn parse_program_statement<'a>(&'a self, input: Input<'a>) -> PineResult<Positioned<Stmt>> {
//         let parse_enum_declaration = |input| self.parse_enum_declaration(input);
//         let parse_variable_statement = |input| self.parse_variable_statement(input);
//         terminated(
//             spaned(alt((
//                 map(
//                     parse_enum_declaration,
//                     Statement::EnumDeclaration,
//                 ),
//                 map(
//                     parse_variable_statement,
//                     Statement::EnumDeclaration,
//                 ),
//             ))),
//             opt(tag(";")),
//         )(input)
//     }
// }