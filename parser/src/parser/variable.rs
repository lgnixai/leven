use crate::{ast::FunctionDeclaration, Parser, tags::{async_tag, function_tag, positioned}};

use nom::{
    combinator::{map, opt, value},
    sequence::{preceded, tuple},
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use tsr_ast::node::enumeration::EnumDeclaration;

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    token::Modifier,
    tokens::Tokens,
};
use tsr_lexer::globals::{Input, PineResult};
use tsr_lexer::state::AstState;
use tsr_lexer::token::{ReservedWord, Token};
use crate::ast::{VariableDeclaration, VariableStatement};
use crate::parser::parse_ident_dec::parse_identifier;
use crate::parsing::{parse_code_block, parse_ident};
use crate::parsing::signatures::parse_call_signature;
use crate::parsing::statement::expression::parse_expression;
use crate::parsing::types::parse_type;
use crate::tags::{colon_tag, comma_tag, const_tag, eq_tag, fat_arrow_tag, let_tag, question_tag, semi_tag, spaned};

// pub fn parse_function_declaration(input: Tokens) -> TokenResult<Positioned<FunctionDeclaration>> {
//     positioned(map(
//         tuple((
//             opt(positioned(value(Modifier::Async, async_tag))),
//             preceded(function_tag, parse_ident),
//             parse_call_signature,
//             opt(parse_code_block),
//         )),
//         |(async_modifier, name, signature, body)| FunctionDeclaration {
//             name,
//             type_parameters: signature.value.0,
//             parameters: signature.value.1,
//             ty: signature.value.2,
//             body,
//             modifiers: async_modifier
//                 .map(|modifier| vec![modifier])
//                 .unwrap_or_default(),
//         },
//     ))(input)
// }
impl Parser {
     pub fn parse_variable_statement<'a>(&'a self, input: Input<'a>) -> PineResult<Positioned<EnumDeclaration>> {
         spaned(map(
             tuple((
                 spaned( tag("enum1")),
                 parse_identifier,
             )),
             |(_, name)| EnumDeclaration { name },
         ))(input)
    }

}