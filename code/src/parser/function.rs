// use crate::{ast::FunctionDeclaration, Parser, tags::{async_tag, function_tag, positioned}};
//
// use nom::{
//     combinator::{map, opt, value},
//     sequence::{preceded, tuple},
// };
//
// use tsr_lexer::{
//     globals::{Positioned, TokenResult},
//     token::Modifier,
//     tokens::Tokens,
// };
// use tsr_lexer::state::AstState;
// use crate::parsing::{parse_code_block, parse_ident};
// use crate::parsing::signatures::parse_call_signature;
// use crate::parsing::statement::expression::parse_expression;
// use crate::tags::fat_arrow_tag;
//
// // pub fn parse_function_declaration(input: Tokens) -> TokenResult<Positioned<FunctionDeclaration>> {
// //     positioned(map(
// //         tuple((
// //             opt(positioned(value(Modifier::Async, async_tag))),
// //             preceded(function_tag, parse_ident),
// //             parse_call_signature,
// //             opt(parse_code_block),
// //         )),
// //         |(async_modifier, name, signature, body)| FunctionDeclaration {
// //             name,
// //             type_parameters: signature.value.0,
// //             parameters: signature.value.1,
// //             ty: signature.value.2,
// //             body,
// //             modifiers: async_modifier
// //                 .map(|modifier| vec![modifier])
// //                 .unwrap_or_default(),
// //         },
// //     ))(input)
// // }
// impl Parser {
//     pub fn parse_function_declaration<'a>(&'a self, input: Tokens<'a>) -> TokenResult<Positioned<FunctionDeclaration>> {
//         self.state.enter_scope();
//         println!("get_indent{:?}",self.state.get_indent());
//         positioned(map(
//             tuple((
//                 opt(positioned(value(Modifier::Async, async_tag))),
//                 parse_ident,
//                 parse_call_signature,
//                 opt(parse_code_block),
//             )),
//             |(async_modifier, name, signature, body)| FunctionDeclaration {
//                 name,
//                 type_parameters: signature.value.0,
//                 parameters: signature.value.1,
//                 ty: signature.value.2,
//                 body,
//                 modifiers: async_modifier
//                     .map(|modifier| vec![modifier])
//                     .unwrap_or_default(),
//             },
//         ))(input)
//     }
// }