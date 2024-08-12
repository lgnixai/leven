use crate::{ast::FunctionDeclaration, Parser, tags::{async_tag, function_tag, positioned}};

use nom::{
    combinator::{map, opt, value},
    sequence::{preceded, tuple},
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, terminated};
use tsr_ast::node::enumeration::EnumDeclaration;

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    token::Modifier,
    tokens::Tokens,
};
use tsr_lexer::globals::{Input, PineResult};
use tsr_lexer::state::AstState;
use tsr_lexer::token::{ReservedWord, Token};
use crate::parser::parse_ident_dec::parse_identifier;
use crate::parsing::{parse_code_block, parse_ident};
use crate::parsing::signatures::parse_call_signature;
use crate::parsing::statement::expression::parse_expression;
use crate::parsing::types::parse_type;
use crate::tags::{brace_close_tag, brace_open_tag, colon_tag, comma_tag, const_tag, enum_tag, eq_tag, fat_arrow_tag, let_tag, question_tag, semi_tag, spaned};

impl Parser {

    pub fn parse_enum_declaration<'a>(&'a self, input: Input<'a>) -> PineResult<Positioned<EnumDeclaration>> {
        spaned(map(
            tuple((
                      spaned( tag("enum")),
                      parse_identifier,
            )),
            |(_, name)| EnumDeclaration { name },
        ))(input)
    }

}